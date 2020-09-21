use cosmwasm_std::{
    Api, Binary, CosmosMsg, Env, Extern, HandleResponse, HandleResult, HumanAddr, InitResponse,
    InitResult, Querier, QueryResult, StdError, Storage, WasmMsg,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InitMsg {
    InitPairFactory {
        admin: HumanAddr,
        pair_code_id: u64,
        pair_code_hash: String,
    },
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    admin: HumanAddr,
    pair_code_id: u64,
    pair_code_hash: String,
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> InitResult {
    match msg {
        InitMsg::InitPairFactory {
            admin,
            pair_code_id,
            pair_code_hash,
        } => {
            let config = Config {
                admin,
                pair_code_id,
                pair_code_hash,
            };
            let all_pairs: Vec<Pair> = vec![];

            deps.storage
                .set(b"config", &bincode2::serialize(&config).unwrap());
            deps.storage
                .set(b"all_pairs", &bincode2::serialize(&all_pairs).unwrap());

            Ok(InitResponse::default())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreatePair {
        pair_label: String,
        token_a: HumanAddr,
        token_a_code_hash: String,
        token_b: HumanAddr,
        token_b_code_hash: String,
    },
}

#[derive(Serialize, Deserialize, Clone)]
struct Pair {
    pair_label: String,
    token_0: HumanAddr,
    token_0_code_hash: String,
    token_1: HumanAddr,
    token_1_code_hash: String,
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: HandleMsg,
) -> HandleResult {
    match msg {
        HandleMsg::CreatePair {
            pair_label,
            token_a,
            token_a_code_hash,
            token_b,
            token_b_code_hash,
        } => {
            if token_a == token_b {
                return Err(StdError::generic_err("Same Token"));
            }

            let token_0: HumanAddr;
            let token_0_code_hash: String;
            let token_1: HumanAddr;
            let token_1_code_hash: String;

            if token_a.0 < token_b.0 {
                token_0 = token_a;
                token_0_code_hash = token_a_code_hash;
                token_1 = token_b;
                token_1_code_hash = token_b_code_hash;
            } else {
                token_0 = token_b;
                token_0_code_hash = token_b_code_hash;
                token_1 = token_a;
                token_1_code_hash = token_a_code_hash;
            }

            let pair_storage_key_0_1 = format!("{}{}", token_0.0, token_1.0);
            let pair_storage_key_1_0 = format!("{}{}", token_1.0, token_0.0);

            if deps.storage.get(pair_storage_key_0_1.as_bytes()).is_none() {
                return Err(StdError::generic_err("Pair Exists"));
            }

            let mut all_pairs: Vec<Pair> =
                bincode2::deserialize(&deps.storage.get(b"all_pairs").unwrap()).unwrap();
            let pair = Pair {
                pair_label: pair_label.clone(),
                token_0: token_0.clone(),
                token_0_code_hash: token_0_code_hash.clone(),
                token_1: token_1.clone(),
                token_1_code_hash: token_1_code_hash.clone(),
            };
            all_pairs.push(pair);

            deps.storage
                .set(pair_storage_key_0_1.as_bytes(), pair_label.as_bytes());
            deps.storage
                .set(pair_storage_key_1_0.as_bytes(), pair_label.as_bytes());
            deps.storage
                .set(b"all_pairs", &bincode2::serialize(&all_pairs).unwrap());

            let config: Config =
                bincode2::deserialize(&deps.storage.get(b"config").unwrap()).unwrap();

            Ok(HandleResponse {
                messages: vec![CosmosMsg::Wasm(WasmMsg::Instantiate {
                    code_id: config.pair_code_id,
                    callback_code_hash: config.pair_code_hash,
                    msg: Binary(
                        format!(
                            r#"{{"init_pair":{{"a":"{}","a_code_hash":"{}","b":"{}","b_code_hash":"{}"}}}}"#,
                            token_0.clone(),
                            token_0_code_hash.clone(),
                            token_1.clone(),
                            token_1_code_hash.clone(),
                        )
                        .as_bytes()
                        .into(),
                    ),
                    send: vec![],
                    label: pair_label,
                })],
                log: Vec::default(),
                data: None,
            })
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetAllPairs {},
}

pub fn query<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, msg: QueryMsg) -> QueryResult {
    match msg {
        QueryMsg::GetConfig {} => {
            let config: Config =
                bincode2::deserialize(&deps.storage.get(b"config").unwrap()).unwrap();
            Ok(Binary(serde_json_wasm::to_vec(&config).unwrap()))
        }
        QueryMsg::GetAllPairs {} => {
            let all_pairs: Vec<Pair> =
                bincode2::deserialize(&deps.storage.get(b"all_pairs").unwrap()).unwrap();
            Ok(Binary(serde_json_wasm::to_vec(&all_pairs).unwrap()))
        }
    }
}

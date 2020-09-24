use cosmwasm_std::{
    Api, Binary, Env, Extern, HandleResponse, HandleResult, HumanAddr, InitResponse, InitResult,
    Querier, QueryRequest, QueryResult, Storage, WasmMsg, WasmQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const SECRET20_VIEWING_KEY: &str = "TODO";

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InitMsg {
    InitPair {
        token_a: HumanAddr,
        token_a_code_hash: String,
        token_b: HumanAddr,
        token_b_code_hash: String,
    },
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    factory: HumanAddr,
    token_a: HumanAddr,
    token_a_code_hash: String,
    token_b: HumanAddr,
    token_b_code_hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Reserves {
    token_a: u128,
    token_b: u128,
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> InitResult {
    match msg {
        InitMsg::InitPair {
            token_a,
            token_a_code_hash,
            token_b,
            token_b_code_hash,
        } => {
            let config = Config {
                factory: env.message.sender,
                token_a: token_a.clone(),
                token_a_code_hash: token_a_code_hash.clone(),
                token_b: token_b.clone(),
                token_b_code_hash: token_b_code_hash.clone(),
            };

            let reserves = Reserves {
                token_a: 0,
                token_b: 0,
            };

            deps.storage
                .set(b"config", &bincode2::serialize(&config).unwrap());
            deps.storage
                .set(b"reserves", &bincode2::serialize(&reserves).unwrap());

            Ok(InitResponse {
                log: vec![],
                messages: vec![
                    WasmMsg::Execute {
                        contract_addr: token_a.clone(),
                        callback_code_hash: token_a_code_hash.clone(),
                        msg: Binary(
                            format!(
                                r#"{{"set_viewing_key":{{"viewing_key":"{}"}}}}"#,
                                SECRET20_VIEWING_KEY,
                            )
                            .as_bytes()
                            .into(),
                        ),
                        send: vec![],
                    }
                    .into(),
                    WasmMsg::Execute {
                        contract_addr: token_b.clone(),
                        callback_code_hash: token_b_code_hash.clone(),
                        msg: Binary(
                            format!(
                                r#"{{"set_viewing_key":{{"viewing_key":"{}"}}}}"#,
                                SECRET20_VIEWING_KEY,
                            )
                            .as_bytes()
                            .into(),
                        ),
                        send: vec![],
                    }
                    .into(),
                ],
            })
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    AddLiquidity {},
    RemoveLiquidity {},
    Swap {},
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> HandleResult {
    let config: Config = bincode2::deserialize(&deps.storage.get(b"config").unwrap()).unwrap();
    let reserves: Reserves =
        bincode2::deserialize(&deps.storage.get(b"reserves").unwrap()).unwrap();
    match msg {
        HandleMsg::AddLiquidity {} => {
            // This is a low-level funtion and should be called by a caller that knows what they're doing
            // Otherwise you WILL lose your deposited tokens
            // We assume that in this tx in previous messages the user had already transferred token_a and token_b to us

            let balance_a = get_my_balance(
                &deps,
                &env,
                config.token_a.clone(),
                config.token_a_code_hash.clone(),
            );
            let balance_b = get_my_balance(
                &deps,
                &env,
                config.token_b.clone(),
                config.token_b_code_hash.clone(),
            );

            let amount_added_a = balance_a - reserves.token_a;
            let amount_added_b = balance_b - reserves.token_b;
        }
        HandleMsg::RemoveLiquidity {} => {
            // This is a low-level funtion and should be called by a caller that knows what they're doing
            // Otherwise you WILL lose your deposited tokens
        }
        HandleMsg::Swap {} => {
            // This is a low-level funtion and should be called by a caller that knows what they're doing
            // Otherwise you WILL lose your deposited tokens
        }
    }

    Ok(HandleResponse::default())
}

fn get_my_balance<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    env: &Env,
    token: HumanAddr,
    code_hash: String,
) -> u128 {
    let balance: u128 = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: token,
            callback_code_hash: code_hash,
            msg: Binary::from(
                format!(
                    r#"{{"balance":{{"address":"{}","viewing_key":"{}"}}}}"#,
                    env.contract.address, SECRET20_VIEWING_KEY
                )
                .as_bytes()
                .to_vec(),
            ),
        }))
        .unwrap();
    balance
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
}

pub fn query<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, msg: QueryMsg) -> QueryResult {
    match msg {
        QueryMsg::GetConfig {} => {
            let config: Config =
                bincode2::deserialize(&deps.storage.get(b"config").unwrap()).unwrap();
            Ok(Binary(serde_json_wasm::to_vec(&config).unwrap()))
        }
    }
}

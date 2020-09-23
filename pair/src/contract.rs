use cosmwasm_std::{
    Api, Binary, Env, Extern, HandleResponse, HandleResult, HumanAddr, InitResponse, InitResult,
    Querier, QueryResult, Storage, WasmMsg,
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

            deps.storage
                .set(b"config", &bincode2::serialize(&config).unwrap());

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
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: HandleMsg,
) -> HandleResult {
    match msg {
        HandleMsg::AddLiquidity {} => {}
        HandleMsg::RemoveLiquidity {} => {}
        HandleMsg::Swap {} => {}
    }

    Ok(HandleResponse::default())
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

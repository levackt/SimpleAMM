use cosmwasm_std::{
    Api, Binary, Env, Extern, HandleResponse, HandleResult, HumanAddr, InitResponse, InitResult,
    Querier, QueryResult, Storage,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    token_a: HumanAddr,
    token_a_code_hash: String,
    token_b: HumanAddr,
    token_b_code_hash: String,
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
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
                token_a,
                token_a_code_hash,
                token_b,
                token_b_code_hash,
            };

            deps.storage
                .set(b"config", &bincode2::serialize(&config).unwrap());

            Ok(InitResponse::default())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct HandleMsg {}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: HandleMsg,
) -> HandleResult {
    Ok(HandleResponse::default())
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryMsg {}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _msg: QueryMsg,
) -> QueryResult {
    Ok(Binary::default())
}

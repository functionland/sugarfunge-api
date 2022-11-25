use std::str::FromStr;

use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use codec::Decode;
use serde_json::json;
use subxt::ext::sp_core::sr25519::Public;
use subxt::ext::sp_runtime::AccountId32;
// use subxt::storage::address::{StorageHasher, StorageMapKey};
use subxt::tx::PairSigner;
use sugarfunge_api_types::pool::*;
use sugarfunge_api_types::primitives::Account;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::functionland_pool::Pool as PoolRuntime;
use sugarfunge_api_types::sugarfunge::runtime_types::functionland_pool::PoolRequest as PoolRequestRuntime;
use sugarfunge_api_types::sugarfunge::runtime_types::functionland_pool::User as UserRuntime;
use sugarfunge_api_types::sugarfunge::runtime_types::sp_core::bounded::bounded_vec::BoundedVec;
// use sugarfunge_api_types::sugarfunge::runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec;

pub async fn create_pool(
    data: web::Data<AppState>,
    req: web::Json<CreatePoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let pool_name = req.pool_name.clone().into_bytes();

    let peer_id = req.peer_id.clone().into_bytes();
    let peer_id = BoundedVec(peer_id);

    let api = &data.api;

    let call = sugarfunge::tx().pool().create(pool_name, peer_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::PoolCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreatePoolOutput {
            owner: transform_option_value(event.owner).into(),
            pool_id: event.pool_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::PoolCreated"),
            description: format!(""),
        })),
    }
}

pub async fn leave_pool(
    data: web::Data<AppState>,
    req: web::Json<LeavePoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx().pool().leave_pool(req.pool_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::ParticipantLeft>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(LeavePoolOutput {
            account: event.account.into(),
            pool_id: event.pool_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::ParticipantLeft"),
            description: format!(""),
        })),
    }
}

pub async fn join_pool(
    data: web::Data<AppState>,
    req: web::Json<JoinPoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let peer_id = req.peer_id.clone().into_bytes();
    let peer_id = BoundedVec(peer_id);

    let api = &data.api;

    let call = sugarfunge::tx().pool().join(req.pool_id, peer_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::JoinRequested>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(JoinPoolOutput {
            account: event.account.into(),
            pool_id: event.pool_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::ParticipantLeft"),
            description: format!(""),
        })),
    }
}

pub async fn cancel_join_pool(
    data: web::Data<AppState>,
    req: web::Json<CancelJoinPoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx().pool().cancel_join(req.pool_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::RequestWithdrawn>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CancelJoinPoolOutput {
            account: event.account.into(),
            pool_id: event.pool_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::RequestWithdrawn"),
            description: format!(""),
        })),
    }
}

pub async fn vote(
    data: web::Data<AppState>,
    req: web::Json<VoteInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let account = AccountId32::try_from(&req.account).map_err(map_account_err)?;

    let api = &data.api;

    let call = sugarfunge::tx()
        .pool()
        .vote(req.pool_id, account, req.vote_value);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::Accepted>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(VoteOutput {
            account: event.account.into(),
            pool_id: event.pool_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::Accepted"),
            description: format!(""),
        })),
    }
}

pub async fn get_all_pools(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let query_key = sugarfunge::storage().pool().pools_root().to_bytes();
    // println!("query_key pool_root len: {}", query_key.len());

    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let pool_id_idx = 48;
        let pool_id_key = key.0.as_slice()[pool_id_idx..(pool_id_idx + 4)].to_vec();
        let pool_id_id = u32::decode(&mut &pool_id_key[..]);
        let pool_id = pool_id_id.unwrap();
        // println!("pool_id: {:?}", pool_id);

        if let Some(storage_data) = api
            .storage()
            .fetch_raw(&key.0, None)
            .await
            .map_err(map_subxt_err)?
        {
            let value = PoolRuntime::decode(&mut &storage_data[..]);
            let pool_value = value.unwrap();

            let storage = pool_value.participants.0;

            let mut storage_vec: Vec<Account> = Vec::new();

            for storer in storage {
                let current_account = Account::try_from(storer).unwrap();
                storage_vec.push(current_account);
            }

            result_array.push(PoolData {
                pool_id,
                pool_name: String::from_utf8(pool_value.name.0).unwrap_or_default(),
                owner: transform_option_value(pool_value.owner),
                parent: pool_value.parent,
                participants: storage_vec,
            });
        }
    }
    Ok(HttpResponse::Ok().json(GetAllPoolsOutput {
        pools: result_array,
    }))
}

pub async fn get_all_pool_requests(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let query_key = sugarfunge::storage().pool().pool_requests_root().to_bytes();
    // println!("query_key pool_root len: {}", query_key.len());

    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let pool_id_idx = 48;
        let pool_id_key = key.0.as_slice()[pool_id_idx..(pool_id_idx + 4)].to_vec();
        let pool_id_id = u32::decode(&mut &pool_id_key[..]);
        let pool_id = pool_id_id.unwrap();
        // println!("pool_id: {:?}", pool_id);

        let account_idx = 68;
        let account_key = key.0.as_slice()[account_idx..(account_idx + 32)].to_vec();
        let account_id = AccountId32::decode(&mut &account_key[..]);
        let account_id = Account::from(account_id.unwrap());
        // println!("account_id: {:?}", account_id);

        if let Some(storage_data) = api
            .storage()
            .fetch_raw(&key.0, None)
            .await
            .map_err(map_subxt_err)?
        {
            let value = PoolRequestRuntime::decode(&mut &storage_data[..]);
            let poolrequest_value = value.unwrap();

            let voters = poolrequest_value.voted.0;

            let mut voters_vec: Vec<Account> = Vec::new();

            for voter in voters {
                let current_account = Account::try_from(voter).unwrap();
                voters_vec.push(current_account);
            }

            result_array.push(PoolRequestData {
                pool_id,
                account: account_id,
                voted: voters_vec,
                positive_votes: poolrequest_value.positive_votes,
                peer_id: String::from_utf8(poolrequest_value.peer_id.0).unwrap_or_default(),
            });
        }
    }
    Ok(HttpResponse::Ok().json(GetAllPoolRequestsOutput {
        poolrequests: result_array,
    }))
}

pub async fn get_all_pool_users(
    data: web::Data<AppState>,
    req: web::Json<GetAllPoolUsersInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let query_key = sugarfunge::storage().pool().users_root().to_bytes();
    // println!("query_key pool_root len: {}", query_key.len());

    // if let Some(account_value) = req.account.clone() {
    //     let account = AccountId32::try_from(&account_value).map_err(map_account_err)?;
    //     StorageMapKey::new(account, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
    //     // println!("query_key class_id len: {}", query_key.len());
    // }

    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        let mut meet_requirements = true;
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let account_idx = 48;
        let account_key = key.0.as_slice()[account_idx..(account_idx + 32)].to_vec();
        let account_id = AccountId32::decode(&mut &account_key[..]);
        let account_id = Account::from(account_id.unwrap());
        // println!("account_id: {:?}", account_id);

        if let Some(storage_data) = api
            .storage()
            .fetch_raw(&key.0, None)
            .await
            .map_err(map_subxt_err)?
        {
            let value = UserRuntime::<BoundedVec<u8>>::decode(&mut &storage_data[..]);
            let user_value = value.unwrap();

            if let Some(account_value) = req.account.clone() {
                if AccountId32::from(
                    Public::from_str(&account_value.as_str()).map_err(map_account_err)?,
                ) != AccountId32::from(
                    Public::from_str(&account_id.as_str()).map_err(map_account_err)?,
                ) {
                    meet_requirements = false;
                }
            }
            if meet_requirements {
                result_array.push(PoolUserData {
                    account: account_id,
                    pool_id: user_value.pool_id,
                    request_pool_id: user_value.request_pool_id,
                    peer_id: String::from_utf8(user_value.peer_id.0).unwrap_or_default(),
                });
            }
        }
    }
    Ok(HttpResponse::Ok().json(GetAllPoolUsersOutput {
        users: result_array,
    }))
}

fn transform_option_value(value: Option<AccountId32>) -> Option<Account> {
    if let Some(value) = value {
        return Some(value.into());
    }
    return None::<Account>;
}

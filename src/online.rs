use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use futures::stream::StreamExt;
use sugarfunge_api_types::online::*;
use sugarfunge_api_types::sugarfunge;

pub async fn get_authored_blocks(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;
    let result_array = Vec::new();

    let query_key = sugarfunge::storage()
        .im_online()
        .authored_blocks_iter()
        .to_root_bytes();

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys_stream = storage
        .fetch_raw_keys(query_key)
        .await
        .map_err(map_subxt_err)?;

    let keys: Vec<Vec<u8>> = keys_stream
        .collect::<Vec<_>>() // Collect into a Vec<Result<Vec<u8>, Error>>
        .await // Await the collection process
        .into_iter() // Convert into an iterator
        .filter_map(Result::ok) // Filter out Ok values, ignore errors
        .collect(); // Collect into a Vec<Vec<u8>>

    println!("Obtained keys:");
    for key in keys.iter() {
        println!("Key: len: {} 0x{}", key.len(), hex::encode(&key));
    }
    Ok(HttpResponse::Ok().json(GetAuthoredBlocks {
        validators: result_array,
    }))
}

pub async fn get_heartbeats(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;
    let result_array = Vec::new();

    let query_key = sugarfunge::storage()
        .im_online()
        .received_heartbeats_iter()
        .to_root_bytes();

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys_stream = storage
        .fetch_raw_keys(query_key)
        .await
        .map_err(map_subxt_err)?;

    let keys: Vec<Vec<u8>> = keys_stream
        .collect::<Vec<_>>() // Collect into a Vec<Result<Vec<u8>, Error>>
        .await // Await the collection process
        .into_iter() // Convert into an iterator
        .filter_map(Result::ok) // Filter out Ok values, ignore errors
        .collect(); // Collect into a Vec<Vec<u8>>

    // println!("Obtained keys:");
    for key in keys.iter() {
        println!("Key: len: {} 0x{}", key.len(), hex::encode(&key));
    }
    Ok(HttpResponse::Ok().json(GetHeartbeats {
        validators: result_array,
    }))
}

pub async fn get_heartbeat_time(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;

    let query_key = sugarfunge::storage().im_online().heartbeat_after();

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys_stream = storage.fetch(&query_key).await.map_err(map_subxt_err)?;

    Ok(HttpResponse::Ok().json(HeartbeatTime { time: keys_stream }))
}

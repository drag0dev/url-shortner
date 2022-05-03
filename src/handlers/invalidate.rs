use serde_json::{json, Value};
use warp::reply::Json;


pub async fn invalidate(data: Value) -> Result<Json, warp::Rejection>{
    // invalidte url
    let res = json!({
        "status" : "ok"
    });

    Ok(warp::reply::json(&res))
}

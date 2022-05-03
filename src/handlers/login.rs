use serde_json::{json, Value};
use warp::reply::Json;

pub async fn login(data: Value) -> Result<Json, warp::Rejection>{
    let res = json!({
        "status": "ok"
    });

    Ok(warp::reply::json(&res))
}

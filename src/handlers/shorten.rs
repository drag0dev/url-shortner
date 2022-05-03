use warp::reply::Json;
use serde_json::{json, Value};

pub async fn shorten(data: Value) -> Result<Json, warp::Rejection>{
    // TODO: actually implement shortening
    let shortened_url = "newUrl";
    let shortened_url = json!(
        {"new_url": shortened_url}
    );

    Ok(warp::reply::json(&shortened_url))
}

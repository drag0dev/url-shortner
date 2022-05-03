use warp::Filter;

mod handlers;

#[tokio::main]
async fn main(){

    // POST @ /shorten
    let shorten = warp::path("shorten")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and_then(handlers::shorten::shorten);

    // DELETE @ /invalidate
    let invalidate = warp::path("invalidate")
        .and(warp::path::end())
        .and(warp::delete())
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and_then(handlers::invalidate::invalidate);

    // POST @ /login
    let login = warp::path("login")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and_then(handlers::login::login);

    // POST @ /register
    let register = warp::path("register")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and_then(handlers::register::register);


    let routes = shorten.or(invalidate).or(login).or(register);
    // TODO: port env
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

use warp::Filter;
use super::handlers;
use super::models::PromptRequest;

fn prompt_request_json_body() -> impl Filter<Extract = (PromptRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_requests()
}

// A route to handle GET requests for a specific post
fn get_requests() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let prompt_route = warp::path("prompt")
        .and(warp::post())
        .and(prompt_request_json_body())
        .and_then(handlers::prompt);

    let promptamdpush_route = warp::path("promptandpush")
        .and(warp::post())
        .and(prompt_request_json_body())
        .and_then(handlers::promptandpush);

    let id_route = warp::path("id")
        .and(warp::get())
        .and_then(handlers::id);

    prompt_route.or(promptamdpush_route).or(id_route)
}
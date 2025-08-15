use std::convert::Infallible;
use warp::{Filter, Reply, Rejection};
use serde::{Deserialize, Serialize};
use handlebars::Handlebars;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Fortune {
    id: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct NewFortune {
    message: String,
}

fn get_env(key: &str, fallback: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| fallback.to_string())
}

async fn healthz_handler() -> Result<impl Reply, Infallible> {
    Ok(warp::reply::with_status("healthy", warp::http::StatusCode::OK))
}

async fn random_handler() -> Result<impl Reply, Infallible> {
    let backend_dns = get_env("BACKEND_DNS", "localhost");
    let backend_port = get_env("BACKEND_PORT", "9000");
    let url = format!("http://{}:{}/fortunes/random", backend_dns, backend_port);

    match reqwest::get(&url).await {
        Ok(response) => {
            match response.json::<Fortune>().await {
                Ok(fortune) => Ok(warp::reply::with_status(
                    fortune.message,
                    warp::http::StatusCode::OK,
                ).into_response()),
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                    Ok(warp::reply::with_status(
                        format!("Error parsing response: {}", e),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ).into_response())
                }
            }
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Ok(warp::reply::with_status(
                format!("Request failed: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ).into_response())
        }
    }
}

async fn all_handler() -> Result<impl Reply, Infallible> {
    let backend_dns = get_env("BACKEND_DNS", "localhost");
    let backend_port = get_env("BACKEND_PORT", "9000");
    let url = format!("http://{}:{}/fortunes", backend_dns, backend_port);

    match reqwest::get(&url).await {
        Ok(response) => {
            match response.json::<Vec<Fortune>>().await {
                Ok(fortunes) => {
                    // Create Handlebars template engine
                    let handlebars = Handlebars::new();
                    let template = r#"{{#each this}}
    <p>{{id}}: {{message}}</p>
{{/each}}"#;

                    match handlebars.render_template(template, &fortunes) {
                        Ok(rendered) => Ok(warp::reply::with_status(
                            warp::reply::html(rendered),
                            warp::http::StatusCode::OK,
                        ).into_response()),
                        Err(e) => {
                            eprintln!("Template rendering failed: {}", e);
                            Ok(warp::reply::with_status(
                                warp::reply::html(format!("Template error: {}", e)),
                                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                            ).into_response())
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                    Ok(warp::reply::with_status(
                        warp::reply::html(format!("Error parsing response: {}", e)),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ).into_response())
                }
            }
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Ok(warp::reply::with_status(
                warp::reply::html(format!("Request failed: {}", e)),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ).into_response())
        }
    }
}

async fn add_handler(new_fortune: NewFortune) -> Result<impl Reply, Infallible> {
    let backend_dns = get_env("BACKEND_DNS", "localhost");
    let backend_port = get_env("BACKEND_PORT", "9000");
    let url = format!("http://{}:{}/fortunes", backend_dns, backend_port);

    // Generate random ID like the Go version
    let id = rand::random::<u32>() % 10000;
    let fortune_data = Fortune {
        id: id.to_string(),
        message: new_fortune.message,
    };

    let client = reqwest::Client::new();
    match client.post(&url)
        .json(&fortune_data)
        .send()
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            "Cookie added!",
            warp::http::StatusCode::OK,
        ).into_response()),
        Err(e) => {
            eprintln!("Request failed: {}", e);
            let error_msg = format!("Request failed: {}", e);
            Ok(warp::reply::with_status(
                error_msg,
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ).into_response())
        }
    }
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            "Not Found",
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some(){
        Ok(warp::reply::with_status(
            "Invalid JSON",
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        Ok(warp::reply::with_status(
            "Method Not Allowed",
            warp::http::StatusCode::METHOD_NOT_ALLOWED,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

#[tokio::main]
async fn main() {
    // Health check endpoint
    let healthz = warp::path("healthz")
        .and(warp::get())
        .and_then(healthz_handler);

    // API endpoints
    let api_random = warp::path!("api" / "random")
        .and(warp::get())
        .and_then(random_handler);

    let api_all = warp::path!("api" / "all")
        .and(warp::get())
        .and_then(all_handler);

    let api_add = warp::path!("api" / "add")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add_handler);

    // Static file serving
    let static_files = warp::fs::dir("./static");

    // Combine all routes
    let routes = healthz
        .or(api_random)
        .or(api_all)
        .or(api_add)
        .or(static_files)
        .recover(handle_rejection);

    println!("Starting frontend server on port 8080...");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}

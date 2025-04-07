use std::sync::Arc;

use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_dev_take_home::{
    handles::OpenApiDoc,
    state::{AppState, Config},
};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<Config>().expect("parse env as Config");

    poem_dev_take_home::tracing::init(&config.log_level);

    let api_service = OpenApiService::new(OpenApiDoc, "Take Home Assessment API Docs", "1.0");
    let ui = api_service.swagger_ui();

    let app_state = AppState::build(config).expect("AppState initiate");
    let app_state = Arc::new(app_state);

    let app = Route::new()
        .nest("/docs", ui)
        .nest("/", api_service)
        .data(app_state);

    let listener = TcpListener::bind("0.0.0.0:8000");
    Server::new(listener).run(app).await
}

use std::sync::Arc;

use peom_dev_take_home::{OpenApiDoc, state::AppState};
use poem::{EndpointExt, Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(OpenApiDoc, "Take Home Assessment API Docs", "1.0");
    let ui = api_service.swagger_ui();

    let app_state = AppState::build().unwrap();
    let app_state = Arc::new(app_state);

    let app = Route::new()
        .nest("/docs", ui)
        .nest("/", api_service)
        .data(app_state);

    let listerner = TcpListener::bind("0.0.0.0:8000");
    Server::new(listerner).run(app).await
}

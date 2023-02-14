// lsof -i :3001
// kill -9 PID

use axum::{Json, routing::get, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(service_ok))
        .route("/46elks/sms", get(elks_sms))
        .route("/46elks/call", get(elks_call));
    
    let addr = "[::]:8080".parse().unwrap();
    println!("Server listening on port {}", addr);
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap()
}

#[derive(Serialize)]
struct SmsJson {
    forward : String,
}

#[derive(Serialize)]
struct CallJson {
    connect : String,
}

async fn service_ok () -> String {
    "Service running, all ok!".to_owned()
}

async fn elks_sms () -> Json<SmsJson> {
    let message = SmsJson { 
        forward: "+46704132860".to_owned() 
    };
    println!("sms route tapped");
    Json(message)
    
}

async fn elks_call () -> Json<CallJson> {
    let message = CallJson { 
        connect: "+46704132860".to_owned() 
    };
    println!("call route tapped");
    Json(message)
    
}
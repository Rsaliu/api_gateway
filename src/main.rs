
mod server;
mod messenger;
mod app_state;
mod helpers;
mod microservices;
use std::{env, net::ToSocketAddrs};
use actix_web::{web, App, HttpServer, Responder};
use app_state::{AppState,ChannelDetails};
use serde_json::ser;
use crate::server::routes;

use std::fmt;
use tracing::{info,log,debug,error,Level};
use tracing_subscriber::FmtSubscriber;
//use messenger::grpc::{email_proto::email_sender_client::EmailSenderClient};
//use crate::messenger::messenger::{self as MessengerObject,Message};
use tokio::sync::{mpsc, Mutex};
use std::thread;
use tokio::task;
use tokio_stream::StreamExt;
use std::sync::Arc;
use crate::microservices::user as UserMicroservice;
// use tonic::transport::Channel;
// use tonic::{transport::Server, Request, Response, Status};
// use email_proto::email_sender_client::{EmailSenderClient};
// use email_proto::{EmailSendRequest};
use web::Data;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};
// pub mod email_proto {
//     tonic::include_proto!("emailsender"); // The string specified here must match the proto package name
// }
#[derive(PartialEq)]
enum AppEnv {
    Dev,
    Prod,
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEnv::Dev => write!(f, "dev"),
            AppEnv::Prod => write!(f, "prod"),
        }
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    let app_env = match env::var("APP_ENV") {
        Ok(v) if v == "prod" => AppEnv::Prod,
        _ => AppEnv::Dev,
    };
    info!("Running in {} mode",app_env);
    if app_env == AppEnv::Dev {
        dotenvy::from_path(".env").expect("dot env error");
    }


    let grpc_server_url:String = env::var("GRPC_SERVER_URL").expect("env variable error");
    let user_microservice_client = UserMicroservice::UserServiceClient::connect(grpc_server_url).await;
    let mut app_state = AppState::default();
    if let Ok(user_microservice_client) = user_microservice_client{
        info!("connection to user micro_service successful");
        let user_microservice_grpc_messenger= UserMicroservice::GrpcMessenger::new(user_microservice_client);
        app_state.user_microservice_grpc_messenger = Arc::new(Mutex::new(Some(user_microservice_grpc_messenger)))
    }else{
        info!("error in grpc connection")
    }
    let server_url:String = env::var("SERVER_URL").expect("env variable error");
    let mut addr_iter = server_url.to_socket_addrs().expect("server address iter error");
    let server_address = addr_iter.next().expect("Socket address errror");
    info!("address is: {} and port is: {}",server_address.ip(),server_address.port());
    HttpServer::new( move || {
        App::new().configure(routes::config).app_data(Data::new(app_state.clone()))
    })
    .bind(server_address)?
    .run()
    .await
}
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::Mutex;
use lapin::Channel;

use crate::microservices::user as UserMicroservice;

#[derive(Clone)]
pub struct ChannelDetails{
    pub name:String,
    pub channel: Channel
}

#[derive(Clone,Default)]
pub struct AppState{
    // pub messenger_channel:Arc<Mutex<Sender<EmailSendRequest>>>,
    // pub db_service_channel:Arc<Mutex<ChannelDetails>>
    pub user_microservice_grpc_messenger: Arc<Mutex<Option<UserMicroservice::GrpcMessenger>>>
}
// pub mod email_proto {
//     tonic::include_proto!("emailsender"); // The string specified here must match the proto package name
// }
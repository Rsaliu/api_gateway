use std::future::Future;
use async_trait::async_trait;
pub struct Message{
    pub id: String,
    pub receipients: Vec<String>,
    pub topic: String,
    pub message: String
}
use std::sync::mpsc::Receiver;
use tracing::{info,log,debug, Level};

// #[async_trait]
// pub trait Messenger{
//     async fn send(&mut self,msg:Message)->Result<(),Box<dyn std::error::Error>>;
// }
// pub async fn messenger_service(mut messenger:Box<dyn Messenger + Send>,mut receiver: Receiver<Message>)->Result<(), Box<dyn std::error::Error>>{
//     while let Ok(received) = receiver.recv() {
//         messenger.send(received).await?;
//         info!("message sent waiting for next");
//     }
//     // for received in receiver.blocking_recv(){
//     //     messenger.send(received).await?;
//     //     log::info!("message sent waiting for next");
//     // }
//     info!("service ended");
//     Ok(())
// }
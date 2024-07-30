
use std::default;

use serde::{de::Error, Deserialize, Serialize};


#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EmailRequestMessage{
    /// Request message contains the name to be greeted
    pub message_id: String,

    pub email_address:String,

    pub title:String,

    pub message:String,
}

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EmailResponseMessage{
    /// Request message contains the name to be greeted
    pub message_id: String,

    pub email_address:String,

    pub title:String,

    pub message:String,

    pub status:EmailMessageStatus
}

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub enum EmailMessageStatus {
    #[default]
    SUCCESS,
    FAILED
}
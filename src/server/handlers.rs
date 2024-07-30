use core::fmt;

use actix_web::http::{StatusCode,header::ContentType};
use actix_web::{error, web, Error, HttpResponse, Responder};
use serde::Deserialize;
use tonic::transport::channel;
use tracing::info;
use validator::{Validate, ValidationError};
use once_cell::sync::Lazy;
use regex::Regex;
use lapin::{BasicProperties,options::BasicPublishOptions};
use crate::app_state::{AppState};
use crate::microservices::user as UserMicroservice;
use prost_types::Timestamp;
use crate::microservices::user::GrpcMessenger;
// use crate::helpers::email_helper::{EmailMessageStatus,EmailRequestMessage,EmailResponseMessage};

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
#[derive(Deserialize,Validate,Debug)]
pub struct SignupRequest {
    #[validate(length(min = 3, max = 100))]
    name: String,
    #[validate(length(min = 3, max = 100), regex = "EMAIL_REGEX")]
    email: String,
    #[validate(length(min = 5, max = 100), custom(function = "validate_password"))]
    password: String
}

fn validate_password(password:&str)->Result<(),ValidationError>{
    let uppercase_regex = Regex::new(r"[A-Z]").unwrap();
    let lowercase_regex = Regex::new(r"[a-z]").unwrap();
    let number_regex = Regex::new(r"[0-9]").unwrap();
    let special_char_regex = Regex::new(r"[@$!%*?&]").unwrap(); 
    if !uppercase_regex.is_match(password) {
        return Err(ValidationError::new("must contain at least one uppercase letter"));
    }

    if !special_char_regex.is_match(password) {
        return Err(ValidationError::new("must contain at least one special character"));
    }
    if !lowercase_regex.is_match(password) {
        return Err(ValidationError::new("must contain at least one lowercase character"));
    }
    if !number_regex.is_match(password) {
        return Err(ValidationError::new("must contain at least one number character"));
    }
    Ok(())
}


impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.status_code, self.message)
    }   
}
#[derive(Debug)]
pub struct CustomError{
    status_code:StatusCode,
    message: String,
}
impl error::ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
           self.status_code
    }
}

pub async fn signup(payload:web::Json<SignupRequest>,data:web::Data<AppState>) -> Result<&'static str, CustomError>{
    info!("payload is: {:?}",payload);
    match payload.validate() {
        Ok(_) => {
            let user_ms_messenger = &data.user_microservice_grpc_messenger;
            let user_ms_messenger = user_ms_messenger.clone();
            let user_ms_messenger = user_ms_messenger.lock().await;
            if user_ms_messenger.is_none() {
                    return Err(CustomError{
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: "Can't connect to user microservice".to_string()
                    });   
            }
            let mut user_ms_messenger = user_ms_messenger.clone().unwrap();
            let user= UserMicroservice::User{
                id:None,
                name:payload.name.clone(),
                email:payload.email.clone(),
                role: UserMicroservice::UserRole::Normal.into(),
                password:payload.password.clone(),
                status:UserMicroservice::UserStatus::Inactive.into(),
                provider:"Native".to_string(),
                created_at:None,
                update_at:None
            };
            let result= user_ms_messenger.create_user(user).await.map_err(|e| {
                CustomError{
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: e.to_string()
                }
            })?;
            let result = result.into_inner();
            if result.status != UserMicroservice::OperationStatus::Sucess as i32 {
                return Err(
                    CustomError{
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: result.message.clone()
                    }
                );
            } 
            // let mdata = user_ms_messenger.clone();
            // let user_data = 
            // let mut channel = mdata.lock().await;
            // let email_req = EmailRequestMessage{
            //     message_id:String::from("fdfd"),
            //     email_address:payload.email.clone(),
            //     title: String::from("Activation"),
            //     message:String::from("message")
            // };
            // let email_req_string =serde_json::to_string(&email_req).map_err(|e| {
            //     CustomError{
            //         status_code: StatusCode::BAD_REQUEST,
            //         message: e.to_string()
            //     }
            // })?;
            // let db_service_channel = data.db_service_channel.clone();
            // let mut db_service_channel = db_service_channel.lock().await;
            info!("will send email");
            //channel.send(emal_req).await;
            
            // let confirm = db_service_channel.channel
            //     .basic_publish(
            //         "",
            //         &db_service_channel.name,
            //         BasicPublishOptions::default(),
            //         email_req_string.as_bytes(),
            //         BasicProperties::default(),
            //     ).await.map_err(|e| {
            //         CustomError{
            //             status_code: StatusCode::INTERNAL_SERVER_ERROR,
            //             message: e.to_string()
            //         }
            //     })?
            //     .await.map_err(|e| {
            //         CustomError{
            //             status_code: StatusCode::INTERNAL_SERVER_ERROR,
            //             message: e.to_string()
            //         }
            //     })?;
            Ok("Hello, world!")
        },
        Err(e) => {
            let err_msg = format!("{}",e.to_string());
            Err(CustomError{
                status_code: StatusCode::BAD_REQUEST,
                message: e.to_string()
            })
            //Err(HttpResponse::BadRequest().body(err_msg))
        }
    }

}

// pub async fn post_hello(req_body: String) -> impl Responder {
//     let req: HelloRequest = serde_json::from_str(&req_body).unwrap();
//     HttpResponse::Ok().body(format!("Hello, {}!", req.name))
// }
mod user_proto {
    tonic::include_proto!("user"); // The string specified here must match the proto package name
}

use tracing_subscriber::fmt::format;
pub use user_proto::user_service_client::UserServiceClient;
use user_proto::{CreateUserRequest};
pub use user_proto::{User,UserRole,UserStatus,CreateUserResponse,OperationStatus};
use tonic::{Request, Response, Status};
pub use tonic::transport::Channel;

#[derive(Debug,Clone)]
pub struct GrpcMessenger{
   client: UserServiceClient<Channel> ,
}

impl GrpcMessenger {
    pub fn new(client: UserServiceClient<Channel>) -> GrpcMessenger{
        GrpcMessenger{
            client: client 
        }
    }
    pub async fn create_user(&mut self,user:User)->Result<Response<CreateUserResponse>,Box<dyn std::error::Error>>{
        let request = Request::new(CreateUserRequest{
            user:Some(user)
        });
        let result=self.client.create_user(request).await.map_err(|e| {
            format!("create user erorr {}",e)
        })?;
        Ok(result)
    }
}
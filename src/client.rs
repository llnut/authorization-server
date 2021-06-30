use tonic::{metadata::MetadataValue, transport::Channel, Request};
use tracing::info;
use user_server::pb_user_client::PbUserClient;
use user_server::{
    LoginRequest, Message, PasswordUpdateRequest, RefreshTokenRequest, Request as PbRequest,
    UserIndexRequest, UserProfileUpdateRequest, UserShowRequest, UserStoreRequest,
};

pub mod user_server {
    tonic::include_proto!("jcsora.user_server");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;

    let token = MetadataValue::from_str("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJncmFudF90eXBlIjoibm9ybWFsIiwiZW1haWwiOiJzb3JhQG91dGxvb2suY29tIiwic3ViIjo2LCJleHAiOjE2MjI3MTcxOTQsImlhdCI6MTYyMjcxMzU5NH0.KVwVeetM3s-IOGHZ_Aa3s0FP_6YNl-pKo8FeUfv3Ff0")?;

    let mut client = PbUserClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(Message {
        msg_type: 1001,
        sequence: 1,
        request: Some(PbRequest {
            user_index: Some(UserIndexRequest {
                id: "1".to_string(),
                email: "sora".to_string(),
                nickname: "sora".to_string(),
                limit: 2,
                page: 1,
            }),
            user_show: Some(UserShowRequest { id: 20 }),
            user_store: Some(UserStoreRequest {
                email: "sora@outlook.com".to_string(),
                password: "123456".to_string(),
            }),
            login: Some(LoginRequest {
                email: "sora@outlook.com".to_string(),
                password: "123456".to_string(),
            }),
            refresh_token: Some(RefreshTokenRequest {
                refresh_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJncmFudF90eXBlIjoicmVmcmVzaCIsImVtYWlsIjoic29yYUBvdXRsb29rLmNvbSIsInN1YiI6NiwiZXhwIjoxNjIyNDUwNDUzLCJpYXQiOjE2MjI0NDY4NTN9.2h8dcvLGeYwTU9kDCSEoW_uENb7dymrAjM64R4UZxHw".to_string(),
            }),
            user_profile_update: Some(UserProfileUpdateRequest {
                id: 1,
                nickname: "".to_string(),
                gender: 2,
                birthday: "2021-06-02 11:24:00".to_string(),
            }),
            password_update: Some(PasswordUpdateRequest {
                email: "sora@outlook.com".to_string(),
                old_password: "123456".to_string(),
                new_password: "1234567".to_string(),
            }),
        }),
        response: None,
    });

    //let response = client.user_index(request).await?;
    //let response = client.user_show(request).await?;
    let response = client.user_store(request).await?;
    //let response = client.login(request).await?;
    //let response = client.refresh_token(request).await?;
    //let response = client.user_profile_update(request).await?;
    //let response = client.password_update(request).await?;

    info!("RESPONSE={:?}", response);
    Ok(())
}

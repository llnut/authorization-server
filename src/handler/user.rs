use crate::config::DbPool;
use crate::model::response::{Meta, Page, Token};
use crate::service::user as user_service;
use crate::user_server::pb_user_server::PbUser;
use crate::user_server::{
    LoginResponse, Message as PbMessage, PaginationMeta, PasswordUpdateResponse,
    RefreshTokenResponse, Request as PbRequest, Response as PbResponse, UserIndexResponse,
    UserIndexResponseRecord, UserProfileUpdateResponse, UserShowResponse, UserStoreResponse,
};
use crate::util::jwt;
use chrono::Local;
use tonic::{Request, Response, Status};
use tracing::info;

trait Middleware {
    fn auth_check(&self) -> Result<bool, Status>;
}

impl<T> Middleware for Request<T> {
    fn auth_check(&self) -> Result<bool, Status> {
        let token = self.metadata().get("authorization");
        if let Some(t) = token {
            let token_info = jwt::verify(t.to_str().unwrap_or(""))?;
            if token_info.grant_type != "normal".to_string() {
                info!("invalid auth token");
                return Err(Status::unauthenticated("invalid auth token"));
            }
            let now: usize = Local::now().timestamp() as usize;
            if token_info.exp > now {
                let refresh_token = self.metadata().get("refresh_token");
                if let Some(t) = refresh_token {
                    let refresh_token_info = jwt::verify(t.to_str().unwrap_or(""))?;
                    if refresh_token_info.grant_type != "refresh".to_string()
                        || refresh_token_info.exp < now
                    {
                        info!("invalid auth token");
                        return Err(Status::unauthenticated("invalid auth token"));
                    }
                }
            }
        } else {
            info!("no valid auth token");
            return Err(Status::unauthenticated("no valid auth token"));
        }
        Ok(true)
    }
}

pub struct PbUserServer {
    pub db_pool: DbPool,
}

pub async fn build_server(db_pool: DbPool) -> PbUserServer {
    PbUserServer { db_pool }
}

impl From<Meta> for PaginationMeta {
    fn from(meta: Meta) -> PaginationMeta {
        PaginationMeta {
            current_page: meta.current_page,
            total_page: meta.total_page,
            limit: meta.limit,
            total: meta.total,
        }
    }
}

impl From<Page<user_service::User>> for UserIndexResponse {
    fn from(user: Page<user_service::User>) -> UserIndexResponse {
        let mut response_record: Vec<UserIndexResponseRecord> = vec![];
        for row in user.record {
            response_record.push(UserIndexResponseRecord {
                id: row.id as i64,
                email: row.email.unwrap(),
                nickname: row.nickname.unwrap_or("".to_string()),
            });
        }
        UserIndexResponse {
            record: response_record,
            meta: Some(user.meta.into()),
        }
    }
}

impl From<user_service::User> for UserIndexResponseRecord {
    fn from(user: user_service::User) -> UserIndexResponseRecord {
        UserIndexResponseRecord {
            id: user.id as i64,
            email: user.email.unwrap_or("asd".to_string()),
            nickname: user.nickname.unwrap_or("".to_string()),
        }
    }
}

impl From<user_service::User> for UserShowResponse {
    fn from(user: user_service::User) -> UserShowResponse {
        UserShowResponse {
            id: user.id as i64,
            email: user.email.unwrap_or("adsf".to_string()),
            nickname: user.nickname.unwrap_or("".to_string()),
        }
    }
}

impl From<user_service::UserProfile> for UserProfileUpdateResponse {
    fn from(user: user_service::UserProfile) -> UserProfileUpdateResponse {
        let birthday: String;
        if let Some(date) = user.birthday {
            birthday = date.format("%Y-%m-%d %H:%M-%S").to_string();
        } else {
            birthday = "".to_string();
        }
        UserProfileUpdateResponse {
            id: user.user_id as i64,
            nickname: user.nickname,
            gender: user.gender as i32,
            birthday,
        }
    }
}

impl From<String> for UserStoreResponse {
    fn from(email: String) -> UserStoreResponse {
        UserStoreResponse { email }
    }
}

impl From<Token> for LoginResponse {
    fn from(token: Token) -> LoginResponse {
        LoginResponse {
            token: token.token,
            refresh_token: token.refresh_token,
        }
    }
}

impl From<Token> for RefreshTokenResponse {
    fn from(token: Token) -> RefreshTokenResponse {
        RefreshTokenResponse {
            token: token.token,
            refresh_token: token.refresh_token,
        }
    }
}

impl From<bool> for PasswordUpdateResponse {
    fn from(result: bool) -> PasswordUpdateResponse {
        PasswordUpdateResponse { result }
    }
}

impl From<UserIndexResponse> for PbMessage {
    fn from(response: UserIndexResponse) -> PbMessage {
        PbMessage {
            msg_type: 2001,
            sequence: 1,
            response: Some(PbResponse {
                result: true,
                error_description: vec![0u8],
                last_block: false,
                block_index: 0,
                user_index: Some(response),
                user_show: None,
                user_store: None,
                login: None,
                refresh_token: None,
                user_profile_update: None,
                password_update: None,
            }),
            request: None,
        }
    }
}

impl From<UserShowResponse> for PbMessage {
    fn from(response: UserShowResponse) -> PbMessage {
        PbMessage {
            msg_type: 2002,
            sequence: 1,
            response: Some(PbResponse {
                result: true,
                error_description: vec![0u8],
                last_block: false,
                block_index: 0,
                user_index: None,
                user_show: Some(response),
                user_store: None,
                login: None,
                refresh_token: None,
                user_profile_update: None,
                password_update: None,
            }),
            request: None,
        }
    }
}

impl From<UserStoreResponse> for PbMessage {
    fn from(response: UserStoreResponse) -> PbMessage {
        PbMessage {
            msg_type: 2003,
            sequence: 1,
            response: Some(PbResponse {
                result: true,
                error_description: vec![0u8],
                last_block: false,
                block_index: 0,
                user_index: None,
                user_show: None,
                user_store: Some(response),
                login: None,
                refresh_token: None,
                user_profile_update: None,
                password_update: None,
            }),
            request: None,
        }
    }
}

impl From<LoginResponse> for PbMessage {
    fn from(response: LoginResponse) -> PbMessage {
        PbMessage {
            msg_type: 2004,
            sequence: 1,
            response: Some(PbResponse {
                result: true,
                error_description: vec![0u8],
                last_block: false,
                block_index: 0,
                user_index: None,
                user_show: None,
                user_store: None,
                login: Some(response),
                refresh_token: None,
                user_profile_update: None,
                password_update: None,
            }),
            request: None,
        }
    }
}

impl From<RefreshTokenResponse> for PbMessage {
    fn from(response: RefreshTokenResponse) -> PbMessage {
        PbMessage {
            msg_type: 2005,
            sequence: 1,
            response: Some(PbResponse {
                result: true,
                error_description: vec![0u8],
                last_block: false,
                block_index: 0,
                user_index: None,
                user_show: None,
                user_store: None,
                login: None,
                refresh_token: Some(response),
                user_profile_update: None,
                password_update: None,
            }),
            request: None,
        }
    }
}

impl From<UserProfileUpdateResponse> for PbMessage {
    fn from(response: UserProfileUpdateResponse) -> PbMessage {
        PbMessage {
            msg_type: 2006,
            sequence: 1,
            response: Some(PbResponse {
                result: true,
                error_description: vec![0u8],
                last_block: false,
                block_index: 0,
                user_index: None,
                user_show: None,
                user_store: None,
                login: None,
                refresh_token: None,
                user_profile_update: Some(response),
                password_update: None,
            }),
            request: None,
        }
    }
}

impl From<PasswordUpdateResponse> for PbMessage {
    fn from(response: PasswordUpdateResponse) -> PbMessage {
        PbMessage {
            msg_type: 2007,
            sequence: 1,
            response: Some(PbResponse {
                result: true,
                error_description: vec![0u8],
                last_block: false,
                block_index: 0,
                user_index: None,
                user_show: None,
                user_store: None,
                login: None,
                refresh_token: None,
                user_profile_update: None,
                password_update: Some(response),
            }),
            request: None,
        }
    }
}

#[tonic::async_trait]
impl PbUser for PbUserServer {
    async fn user_index(&self, request: Request<PbMessage>) -> Result<Response<PbMessage>, Status> {
        let pb_request = PbRequest::from(request);
        let db_result =
            user_service::user_index(pb_request.user_index.unwrap(), self.db_pool.clone()).await?;

        Ok(Response::new(PbMessage::from(UserIndexResponse::from(
            db_result,
        ))))
    }

    async fn user_show(&self, request: Request<PbMessage>) -> Result<Response<PbMessage>, Status> {
        let pb_request = PbRequest::from(request);
        let db_result =
            user_service::user_show(pb_request.user_show.unwrap(), self.db_pool.clone()).await?;
        let pb_response = UserShowResponse::from(db_result);
        Ok(Response::new(PbMessage::from(pb_response)))
    }

    async fn user_store(&self, request: Request<PbMessage>) -> Result<Response<PbMessage>, Status> {
        let pb_request = PbRequest::from(request);
        let db_result =
            user_service::user_store(pb_request.user_store.unwrap(), self.db_pool.clone()).await?;
        let pb_response = UserStoreResponse::from(db_result);
        Ok(Response::new(PbMessage::from(pb_response)))
    }

    async fn login(&self, request: Request<PbMessage>) -> Result<Response<PbMessage>, Status> {
        let pb_request = PbRequest::from(request);
        let token = user_service::login(pb_request.login.unwrap(), self.db_pool.clone()).await?;
        let pb_response = LoginResponse::from(token);
        Ok(Response::new(PbMessage::from(pb_response)))
    }

    async fn refresh_token(
        &self,
        request: Request<PbMessage>,
    ) -> Result<Response<PbMessage>, Status> {
        let pb_request = PbRequest::from(request);
        let token = user_service::refresh_token(pb_request.refresh_token.unwrap()).await?;
        let pb_response = RefreshTokenResponse::from(token);
        Ok(Response::new(PbMessage::from(pb_response)))
    }

    async fn user_profile_update(
        &self,
        request: Request<PbMessage>,
    ) -> Result<Response<PbMessage>, Status> {
        let pb_request = PbRequest::from(request);
        let db_result = user_service::user_profile_update(
            pb_request.user_profile_update.unwrap(),
            self.db_pool.clone(),
        )
        .await?;
        let pb_response = UserProfileUpdateResponse::from(db_result);
        Ok(Response::new(PbMessage::from(pb_response)))
    }

    async fn password_update(
        &self,
        request: Request<PbMessage>,
    ) -> Result<Response<PbMessage>, Status> {
        request.auth_check()?;
        let pb_request = PbRequest::from(request);
        let db_result = user_service::password_update(
            pb_request.password_update.unwrap(),
            self.db_pool.clone(),
        )
        .await?;
        let pb_response = PasswordUpdateResponse::from(db_result);
        Ok(Response::new(PbMessage::from(pb_response)))
    }
}

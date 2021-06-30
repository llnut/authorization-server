use crate::config::DbPool;
use crate::error::UserServerError;
use crate::model::request::ListOption;
use crate::model::response::{Page, Token};
use crate::schema::{user_profile, users};
use crate::user_server::{
    LoginRequest, PasswordUpdateRequest, RefreshTokenRequest, UserIndexRequest,
    UserProfileUpdateRequest, UserShowRequest, UserStoreRequest,
};
use crate::util::{jwt, pagination::*, password, random};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types;
use tracing::info;

no_arg_sql_function!(last_insert_id, sql_types::Unsigned<sql_types::Integer>);

#[derive(Queryable, Debug)]
pub struct User {
    pub id: u32,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub gender: Option<i8>,
    pub birthday: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct UserProfile {
    pub user_id: u32,
    pub nickname: String,
    pub gender: i8,
    pub birthday: Option<NaiveDateTime>,
}

impl From<UserIndexRequest> for ListOption {
    fn from(request: UserIndexRequest) -> ListOption {
        let option = ListOption {
            limit: request.limit,
            page: request.page,
        };
        option.apply_default()
    }
}

pub async fn user_index(
    params: UserIndexRequest,
    db_pool: DbPool,
) -> Result<Page<User>, UserServerError> {
    let list_option = ListOption::from(params.clone());
    info!("params:{:?}", params);
    let conn = &db_pool.get().unwrap();

    let mut query = users::table.left_join(user_profile::table).into_boxed();
    if params.id != "".to_string() {
        let vec: Vec<u32> = params
            .id
            .split(',')
            .map(|item| item.parse::<u32>().unwrap_or(0))
            .collect();
        query = query.filter(users::id.eq_any(vec));
    }
    if params.email != "".to_string() {
        query = query.filter(users::email.eq(params.email));
    }

    let result = query
        .select((
            users::id,
            users::email,
            user_profile::nickname.nullable(),
            user_profile::gender.nullable(),
            user_profile::birthday.nullable(),
        ))
        .page(list_option.page)
        .limit(list_option.limit)
        .paginate::<User>(conn)?;

    info!("index查询的内容:{:?}", result);
    Ok(result)
}

pub async fn user_show(params: UserShowRequest, db_pool: DbPool) -> Result<User, UserServerError> {
    let conn = &db_pool.get().unwrap();

    let mut query = users::table.left_join(user_profile::table).into_boxed();
    if params.id != 0 {
        query = query.filter(users::id.eq(params.id as u32));
    }

    let result = query
        .select((
            users::id,
            users::email,
            user_profile::nickname.nullable(),
            user_profile::gender.nullable(),
            user_profile::birthday.nullable(),
        ))
        .get_result::<User>(conn)?;
    info!("show查询的内容:{:?}", result);
    Ok(result)
}

pub async fn user_store(
    params: UserStoreRequest,
    db_pool: DbPool,
) -> Result<String, UserServerError> {
    let conn = &db_pool.get().unwrap();

    if params.email == "".to_string() || params.password.len() < 6 {
        return Err(UserServerError::ArgumentError("参数不合法".to_string()));
    }
    info!("password.len():{:?}", params.password.len());

    let hash = password::hash_password(&params.password)?;
    let new_user = (users::email.eq(&params.email), users::hash.eq(hash));
    let result = conn.transaction::<u32, UserServerError, _>(|| {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(conn)?;

        let user_id: u32 = diesel::select(last_insert_id).first(conn)?;

        diesel::insert_into(user_profile::table)
            .values((
                user_profile::user_id.eq(user_id),
                user_profile::nickname.eq("新用户".to_string() + &random::random_string(16)),
                user_profile::gender.eq(0),
            )).execute(conn)?;
        Ok(user_id)
    })?;

    info!("store结果:{:?}", result);
    Ok(params.email)
}

pub async fn login(params: LoginRequest, db_pool: DbPool) -> Result<Token, UserServerError> {
    let conn = &db_pool.get().unwrap();

    let result = users::table
        .select((users::id, users::email.nullable(), users::hash))
        .filter(users::email.eq(params.email))
        .get_result::<(u32, Option<String>, String)>(conn)?;

    info!("login查询的内容:{:?}", result);
    let login_result = password::verify(&result.2, &params.password)?;
    if !login_result {
        return Err(UserServerError::PasswordUnauthorizedError(
            "密码错误".to_string(),
        ));
    }
    let token = Token {
        token: jwt::get_token(
            "normal".to_string(),
            3600,
            result.0,
            result.1.clone().unwrap_or("".to_string()),
        )?,
        refresh_token: jwt::get_token(
            "refresh".to_string(),
            86400 * 7,
            result.0,
            result.1.unwrap_or("".to_string()),
        )?,
    };
    let token_info = jwt::verify(&token.token)?;
    info!("{:?}", token_info);
    Ok(token)
}

pub async fn refresh_token(params: RefreshTokenRequest) -> Result<Token, UserServerError> {
    let token_info = jwt::verify(&params.refresh_token)?;
    info!("{:?}", token_info);
    let token = Token {
        token: jwt::get_token("normal".to_string(), 3600, token_info.sub, token_info.email)?,
        refresh_token: params.refresh_token,
    };
    Ok(token)
}

pub async fn user_profile_update(
    params: UserProfileUpdateRequest,
    db_pool: DbPool,
) -> Result<UserProfile, UserServerError> {
    let conn = &db_pool.get().unwrap();

    if params.id == 0 {
        return Err(UserServerError::ArgumentError("id 参数不合法".to_string()));
    }
    let mut result = user_profile::table
        .filter(user_profile::id.eq(params.id as u32))
        .select((
            user_profile::user_id,
            user_profile::nickname,
            user_profile::gender,
            user_profile::birthday.nullable(),
        ))
        .get_result::<UserProfile>(conn)?;

    let mut update_value = (
        user_profile::nickname.eq(result.nickname.clone()),
        user_profile::gender.eq(result.gender),
        user_profile::birthday.eq(result.birthday),
    );

    if params.nickname != "".to_string() {
        update_value.0 = user_profile::nickname.eq(params.nickname.clone());
        result.nickname = params.nickname;
    }
    if params.gender != 0 {
        update_value.1 = user_profile::gender.eq(params.gender as i8);
        result.gender = params.gender as i8;
    }
    if params.birthday != "".to_string() {
        update_value.2 = user_profile::birthday.eq(Some(NaiveDateTime::parse_from_str(
            &params.birthday,
            "%Y-%m-%d %H:%M:%S",
        )?));
        result.birthday = Some(NaiveDateTime::parse_from_str(
            &params.birthday,
            "%Y-%m-%d %H:%M:%S",
        )?);
    }

    let update_row =
        diesel::update(user_profile::table.filter(user_profile::id.eq(params.id as u32)))
            .set(update_value)
            .execute(conn)?;

    info!("update结果:{:?}", update_row);
    Ok(result)
}

pub async fn password_update(
    params: PasswordUpdateRequest,
    db_pool: DbPool,
) -> Result<bool, UserServerError> {
    let conn = &db_pool.get().unwrap();

    let result = users::table
        .select((users::id, users::email.nullable(), users::hash))
        .filter(users::email.eq(&params.email))
        .get_result::<(u32, Option<String>, String)>(conn)?;

    info!("login查询的内容:{:?}", result);
    let verify = password::verify(&result.2, &params.old_password)?;
    if !verify {
        return Err(UserServerError::PasswordUnauthorizedError(
            "密码错误".to_string(),
        ));
    } else {
        let new_hash = password::hash_password(&params.new_password)?;
        diesel::update(users::table.filter(users::email.eq(params.email)))
            .set(users::hash.eq(new_hash))
            .execute(conn)?;
    }
    info!("密码修改成功");
    Ok(true)
}

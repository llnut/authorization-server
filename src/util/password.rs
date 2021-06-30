use crate::error::UserServerError;
use argon2::{self, Config, ThreadMode, Variant, Version};
use once_cell::sync::Lazy;
use tracing::error;

static PASSWORD_SECRET_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("PASSWORD_SECRET_KEY").expect("未设置 PASSWORD_SECRET_KEY"));

const SALT: &'static [u8] = b"sorasupersecuresalt";

pub fn hash_password(password: &str) -> Result<String, UserServerError> {
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: PASSWORD_SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
        error!("{}", err.to_string());
        UserServerError::PasswordHashError(err.to_string())
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, UserServerError> {
    argon2::verify_encoded_ext(
        hash,
        password.as_bytes(),
        PASSWORD_SECRET_KEY.as_bytes(),
        &[],
    )
    .map_err(|_| UserServerError::PasswordUnauthorizedError("密码认证失败".to_string()))
}

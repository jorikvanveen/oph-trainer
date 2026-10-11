use axum::http::{HeaderMap, header};
use rand::{TryRngCore, rngs::OsRng};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

use crate::{
    dto::{AccountDto, ApiErrorCodeDto, ApiErrorDto},
    entity::accounts as account,
};

pub struct AccountService {
    db: DatabaseConnection,
}

impl AccountService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn database(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn current(&self, headers: &HeaderMap) -> Result<Option<String>, ApiErrorDto> {
        let mut credentials = headers.get_all(header::AUTHORIZATION).iter();
        let Some(value) = credentials.next() else {
            return Ok(None);
        };
        let Some((scheme, key)) = value.to_str().ok().and_then(|value| value.split_once(' '))
        else {
            return Err(invalid_key());
        };
        if credentials.next().is_some()
            || !scheme.eq_ignore_ascii_case("Bearer")
            || !valid_secret(key)
        {
            return Err(invalid_key());
        }
        let account = account::Entity::find_by_id(key)
            .one(&self.db)
            .await
            .map_err(database_error)?;
        account
            .map(|account| Some(account.key))
            .ok_or_else(invalid_key)
    }

    pub async fn require(&self, headers: &HeaderMap) -> Result<String, ApiErrorDto> {
        self.current(headers).await?.ok_or_else(|| {
            error(
                ApiErrorCodeDto::Unauthenticated,
                "Send your account key in the Authorization bearer header.",
            )
        })
    }

    pub async fn create(&self) -> Result<AccountDto, ApiErrorDto> {
        let key = secret()?;
        account::ActiveModel {
            key: Set(key.clone()),
        }
        .insert(&self.db)
        .await
        .map_err(database_error)?;
        tracing::info!("Account created");
        Ok(AccountDto { account_key: key })
    }
}

fn valid_secret(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn secret() -> Result<String, ApiErrorDto> {
    let mut bytes = [0; 32];
    OsRng.try_fill_bytes(&mut bytes).map_err(|_| {
        error(
            ApiErrorCodeDto::AccountUnavailable,
            "Could not create an account key. Try again.",
        )
    })?;
    Ok(hex::encode(bytes))
}

fn invalid_key() -> ApiErrorDto {
    error(
        ApiErrorCodeDto::InvalidAccountKey,
        "Account key not recognized.",
    )
}

pub(crate) fn database_error(error: DbErr) -> ApiErrorDto {
    tracing::error!(%error, "Account or progress database operation failed");
    self::error(
        ApiErrorCodeDto::DatabaseUnavailable,
        "Could not load or save account progress. Try again.",
    )
}

pub(crate) fn error(code: ApiErrorCodeDto, message: impl Into<String>) -> ApiErrorDto {
    ApiErrorDto {
        code,
        message: message.into(),
    }
}

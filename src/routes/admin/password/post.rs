use actix_web::{HttpResponse, web};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{authentication::{validate_credentials, AuthError, Credentials}, routes::admin::dashboard::get_username, session_state::TypedSession, utils::{e500, see_other}};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    session: TypedSession,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, actix_web::Error> {
    if form.new_password.expose_secret().len() < 12 { //no need checking new password check because it must match
        FlashMessage::error(
            "New password must be at least 12 characters long."
        )
        .send();
        return Ok(see_other("/admin/password"))
    } else if form.new_password.expose_secret().len() > 128 {
        FlashMessage::error(
            "New password must be at most 128 characters long."
        )
        .send();
        return Ok(see_other("/admin/password"))
    }

    let user_id = session.get_user_id().map_err(e500)?;
    if user_id.is_none() {
        return Ok(see_other("/login"))
    };
    let user_id = user_id.unwrap();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();
        return Ok(see_other("/admin/password"));
    }
    let username = get_username(user_id, &pool).await.map_err(e500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/admin/password"))
            },
            AuthError::UnexpectedError(_) => Err(e500(e).into()),
        }
    }
    todo!()
}
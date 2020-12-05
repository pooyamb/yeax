use std::fmt;
use std::rc::Rc;

use actix_web::{get, http::StatusCode, web, HttpResponse, ResponseError};
use yeax::App;

type Hook<T> = Rc<dyn Fn(&str, &str) -> Result<T, AuthError>>;

#[derive(Default, Clone)]
struct AuthConfig {
    pre_auth: Vec<Hook<()>>,
    post_auth: Vec<Hook<AuthHookResponse>>,
}

#[derive(Default)]
pub struct AuthApp {
    config: AuthConfig,
}

pub enum AuthHookResponse {
    Respond(HttpResponse),
    Continue,
}

impl AuthApp {
    pub fn on_pre_auth<F>(&mut self, f: F)
    where
        F: Fn(&str, &str) -> Result<(), AuthError> + 'static,
    {
        self.config.pre_auth.push(Rc::new(f))
    }

    pub fn on_post_auth<F>(&mut self, f: F)
    where
        F: Fn(&str, &str) -> Result<AuthHookResponse, AuthError> + 'static,
    {
        self.config.post_auth.push(Rc::new(f))
    }
}

impl App for AuthApp {
    fn configure_web(&self, cfg: &mut web::ServiceConfig) {
        cfg.service(login).data(self.config.clone());
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

const USERNAMES: &[&str] = &["admin", "manager", "user", "modir"];

#[get("/{username}/{password}")]
async fn login(
    web::Path((username, password)): web::Path<(String, String)>,
    cfg: web::Data<AuthConfig>,
) -> Result<HttpResponse, AuthError> {
    for hook in cfg.pre_auth.iter() {
        hook(username.as_str(), password.as_str())?;
    }

    if username == password && USERNAMES.contains(&username.as_str()) {
        for hook in cfg.post_auth.iter() {
            match hook(username.as_str(), password.as_str())? {
                AuthHookResponse::Continue => {}
                AuthHookResponse::Respond(response) => return Ok(response),
            }
        }
        Ok(HttpResponse::Ok().body(String::from("Success")))
    } else {
        Err(AuthError::InvalidCredentials)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum AuthError {
    NotAuthrized,
    InvalidCredentials,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AuthError")
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::NotAuthrized => StatusCode::FORBIDDEN,
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
        }
    }
}

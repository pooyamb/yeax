use actix_web::{http::header::LOCATION, HttpResponse};
use yeax::{App, Registry};

use crate::auth::{AuthApp, AuthHookResponse};

#[derive(Default)]
pub struct SecondFactorApp {}

impl App for SecondFactorApp {
    fn init(&mut self, r: &mut Registry) {
        r.register_di(|auth_app: &mut AuthApp| {
            // After we authenticate the user
            auth_app.on_post_auth(|username, _| {
                if username == "manager" {
                    // do stuff needed and redirect
                    Ok(AuthHookResponse::Respond(
                        HttpResponse::Found()
                            .header(LOCATION, "/login/token")
                            .finish(),
                    ))
                } else {
                    Ok(AuthHookResponse::Continue)
                }
            })
        })
    }
}

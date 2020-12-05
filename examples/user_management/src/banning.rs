use yeax::{App, Registry};

use crate::auth::{AuthApp, AuthError};

#[derive(Default)]
pub struct BanningApp {}

impl App for BanningApp {
    fn init(&mut self, r: &mut Registry) {
        r.register_di(|auth_app: &mut AuthApp| {
            auth_app.on_pre_auth(|username, _| {
                if username == "admin" {
                    Err(AuthError::NotAuthrized)
                } else {
                    Ok(())
                }
            });
        })
    }
}

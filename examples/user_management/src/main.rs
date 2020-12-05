use actix_web::{App, HttpServer};
use yeax::{ActixReactorExt, Reactor};

mod auth;
mod banning;
mod secondfactor;

use auth::AuthApp;
use banning::BanningApp;
use secondfactor::SecondFactorApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let r = Reactor::default()
            .add(AuthApp::default())
            .add(BanningApp::default())
            .add(SecondFactorApp::default())
            .build();
        App::new().configure_app(&r)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

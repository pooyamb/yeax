use actix_web::web::ServiceConfig;
use downcast_rs::{impl_downcast, Downcast};

use crate::reactor::Registry;

pub trait App: Downcast {
    fn pre_init(&mut self) {}
    fn init(&mut self, _: &mut Registry) {}
    fn post_init(&mut self) {}

    fn configure_web(&self, _: &mut ServiceConfig) {}

    fn finish(&mut self) {}
}

impl_downcast!(App);

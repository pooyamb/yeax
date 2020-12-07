use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;

use actix_service::ServiceFactory;
use actix_web::dev::{MessageBody, ServiceRequest, ServiceResponse};
use actix_web::error::Error;
use actix_web::App as ActixApp;

use crate::app::App;
use crate::di::{InjectFactory, Injectable, Injector};

#[derive(Default)]
pub struct Reactor {
    pub(crate) apps: HashMap<TypeId, Box<dyn App>>,
    registry: Registry,
}

impl Reactor {
    pub fn add<T>(mut self, app: T) -> Self
    where
        T: App,
    {
        self.apps.insert(TypeId::of::<T>(), Box::new(app));
        self
    }

    pub fn build(mut self) -> Self {
        for (_, app) in self.apps.iter_mut() {
            app.init(&mut self.registry);
        }

        self.run_hooks()
    }

    fn run_hooks(mut self) -> Self {
        for injector in self.registry.di.clone().iter() {
            injector.run(&mut self)
        }
        self
    }
}

#[derive(Default)]
pub struct Registry {
    // different hooks can be defined here, like on config change, on new app register etc
    di: Vec<Rc<dyn Injector>>,
}

impl Registry {
    pub fn register_di<F, P>(&mut self, inject_fn: F)
    where
        F: InjectFactory<P> + 'static,
        P: 'static,
    {
        self.di.push(Rc::new(Injectable::new(inject_fn)))
    }
}

pub trait ActixReactorExt {
    fn configure_app(self, r: &Reactor) -> Self;
}

impl<T, B> ActixReactorExt for ActixApp<T, B>
where
    B: MessageBody,
    T: ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
        InitError = (),
    >,
{
    fn configure_app(self, r: &Reactor) -> Self {
        self.configure(|cfg| {
            for (_, app) in r.apps.iter() {
                app.configure_web(cfg);
            }
        })
    }
}

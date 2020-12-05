#![feature(get_mut_unchecked)]

mod app;
mod di;
mod reactor;

pub use app::App;
pub use reactor::{ActixReactorExt, Reactor, Registry};

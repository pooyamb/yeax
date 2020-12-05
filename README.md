# Yeax

This repo contains my personal experiments with `Rust` to make a higher level framework. It is incomplete and it lacks the support for async functions, but it should be good enough for experimenting.

The base idea of `Yeax` is to provide a way to represent a multi app architecture for a web application, think of it like django's apps but with rust's taste or HMVC kind of architecture if you're familiar with that term.

The goal is to provide a more convinient way to develop bigger applications in `Rust`, and provide the basic needs a web application has, and also make it easier to write simple, minimal and reusable tiny apps that are configurable as their own and as part of a bigger picture.

## Example

There is a minimal example in examples folder, and I'll try to update it or add new ones later, but to get a picture of what it looks like, see below:

Defining an application:

```rust
use yeax::App;

struct FirstApp;

impl App for FirstApp{}
```

Depending another application on `FirstApp`:

```rust
use yeax::{App, Registry};

struct SecondApp;

impl App for SecondApp{
    fn init(&mut self, r: &mut Registry) {
        r.register_di(|first_app: &mut FirstApp| {
            // Do something with the first app, possibly adding some config
            // or hooks, or reading some data
        })

        // We can have multiple dependencies(less than 12)
        // Note that all these closures will be called, in the order they are defined.
        r.register_di(|first_app: &mut FirstApp, other: &mut AnotherApp| {
            // ...body
        })

        // We can also depend on our self if we need to.
        // ex: changing internal state
        r.register_di(|me: &mut SecondApp, first_app: &mut FirstApp| {
            // ...body
        })

        // Having 2 of the same dependency will result in error(2 mutable refrence)
        r.register_di(|first_app: &mut FirstApp, yafa: &mut FirstApp| {
            // ...body
        })
    }
}
```

And later in your actix-web's main:

```rust
use actix_web::{App, HttpServer};
use yeax::{ActixReactorExt, Reactor};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let reactor = Reactor::default()
            .add(FirstApp)
            .add(SecondApp)
            .build();
        App::new().configure_app(&reactor)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
```

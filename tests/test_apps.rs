//! Tests are not complete yet

use yeax::{App, Reactor, Registry};

struct A {
    a: i32,
}

impl App for A {
    fn init(&mut self, r: &mut Registry) {
        fn inject(a: &mut A, b: &mut B, c: &mut C, d: &mut D) {
            a.a = 10;
            println!("b: {} c: {} d:{}", b.b, c.c, d.d)
        }

        r.register_di(inject);
    }
}

struct B {
    b: i32,
}

impl App for B {}

struct C {
    c: i32,
}

impl App for C {}

struct D {
    d: i32,
}

impl App for D {}

#[allow(clippy::many_single_char_names)]
#[test]
fn test_it() {
    let a = A { a: 10 };
    let b = B { b: 10 };
    let c = C { c: 10 };
    let d = D { d: 10 };
    let _ = Reactor::default().add(a).add(b).add(c).add(d).build();
}

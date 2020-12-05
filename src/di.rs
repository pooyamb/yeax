use std::marker::PhantomData;

use crate::reactor::Reactor;

trait Extractable<'a> {
    type Result;

    fn extract(r: &'a mut Reactor) -> Option<Self::Result>;
}

pub trait InjectFactory<P> {
    fn run(&self, r: &mut Reactor);
}

pub struct Injectable<F, P> {
    f: F,
    _marker: PhantomData<P>,
}

impl<F, P> Injectable<F, P> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            _marker: PhantomData,
        }
    }
}

pub trait Injector {
    fn run(&self, r: &mut Reactor);
}

impl<F, P> Injector for Injectable<F, P>
where
    F: InjectFactory<P>,
{
    fn run(&self, r: &mut Reactor) {
        self.f.run(r)
    }
}

mod private {
    #![allow(non_snake_case)]

    use std::any::{type_name, TypeId};

    use super::Extractable;
    use super::InjectFactory;
    use crate::app::App;
    use crate::reactor::Reactor;

    macro_rules! impl_factory_for_fn {
        ($($param:ident),*) => {
            impl<F, $($param),*> InjectFactory<($($param),*,)> for F
            where
                F: Fn($(&mut $param),*),
                $($param: App),*
            {
                fn run(&self, r: &mut Reactor) {
                    let ($($param),*,) = <($($param),*,)>::extract(r).unwrap();
                    self($($param),*)
                }
            }
        };
    }

    impl_factory_for_fn!(P1);
    impl_factory_for_fn!(P1, P2);
    impl_factory_for_fn!(P1, P2, P3);
    impl_factory_for_fn!(P1, P2, P3, P4);
    impl_factory_for_fn!(P1, P2, P3, P4, P5);
    impl_factory_for_fn!(P1, P2, P3, P4, P5, P6);
    impl_factory_for_fn!(P1, P2, P3, P4, P5, P6, P7);
    impl_factory_for_fn!(P1, P2, P3, P4, P5, P6, P7, P8);
    impl_factory_for_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9);
    impl_factory_for_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
    impl_factory_for_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
    impl_factory_for_fn!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);

    macro_rules! impl_extractable {
        ($num:expr, $gen1:ident, $($gen:ident => $count:expr),*) => {
            impl<'a, $gen1, $($gen),*> Extractable<'a> for ($gen1, $($gen),*)
            where
                $gen1: App + 'static,
                $($gen : App + 'static),*
            {
                type Result = (&'a mut $gen1, $(&'a mut $gen),*);

                fn extract(r: &'a mut Reactor) -> Option<Self::Result> {
                    let mut res: [Option<&mut Box<dyn App>>; $num] = Default::default();
                    for (id, app) in r.apps.iter_mut() {
                        if *id == TypeId::of::<$gen1>() {
                            res[0] = Some(app);
                        }
                        $(
                            else if *id == TypeId::of::<$gen>() {
                                res[$count] = Some(app);
                            }
                        )*
                    }

                    let [$gen1, $($gen),*] = res;

                    Some((
                        $gen1
                            .unwrap_or_else(||
                                panic!(format!("Dependency {:?} is not registered in app or is used twice!", type_name::<$gen1>()))
                            )
                            .downcast_mut()
                            .unwrap(),
                        $(
                            $gen
                                .unwrap_or_else(||
                                    panic!(format!("Dependency {:?} is not registered in app or is used twice!", type_name::<$gen>()))
                                )
                                .downcast_mut()
                                .unwrap()
                        ),*
                    ))
                }
            }
        };
    }

    impl_extractable!(1, P1,);
    impl_extractable!(2, P1, P2 => 1);
    impl_extractable!(3, P1, P2 => 1, P3 => 2);
    impl_extractable!(4, P1, P2 => 1, P3 => 2, P4 => 3);
    impl_extractable!(5, P1, P2 => 1, P3 => 2, P4 => 3, P5 => 4);
    impl_extractable!(6, P1, P2 => 1, P3 => 2, P4 => 3, P5 => 4, P6 => 5);
    impl_extractable!(7, P1, P2 => 1, P3 => 2, P4 => 3, P5 => 4, P6 => 5, P7 => 6);
    impl_extractable!(8, P1, P2 => 1, P3 => 2, P4 => 3, P5 => 4, P6 => 5, P7 => 6, P8 => 7);
    impl_extractable!(9, P1, P2 => 1, P3 => 2, P4 => 3, P5 => 4, P6 => 5, P7 => 6, P8 => 7, P9 => 8);
    impl_extractable!(10, P1, P2 => 1, P3 => 2, P4 => 3, P5 => 4, P6 => 5, P7 => 6, P8 => 7, P9 => 8, P10 => 9);
    impl_extractable!(11, P1, P2 => 1, P3 => 2, P4 => 3, P5 => 4, P6 => 5, P7 => 6, P8 => 7, P9 => 8, P10 => 9, P11 => 10);
    impl_extractable!(12, P1, P2 => 1, P3 => 2, P4 => 3, P5 => 4, P6 => 5, P7 => 6, P8 => 7, P9 => 8, P10 => 9, P11 => 10, P12 => 11);
}

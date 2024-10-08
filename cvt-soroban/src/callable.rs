use soroban_sdk::Env;

pub trait Call {
    fn call(self, env: &Env);
}

#[macro_export]
macro_rules! make_callable {
    ($contract:ident, $call:ident) => {
        paste::paste!(
            #[allow(non_camel_case_types)]
            pub struct [< call_ $call >];

            impl Call for [< call_ $call >] {
                fn call(self, env: &Env) {
                    $contract::$call(env.clone());
                }
            }
        );
    };

    ($contract:ident, $call:ident, $($arg:ident : $ty:ty),*) => {
        paste::paste!(
            #[allow(non_camel_case_types)]
            pub struct [< call_ $call >] { $($arg: $ty,)* }

            impl Call for [< call_ $call >] {
                fn call(self, env: &Env) {
                    $contract::$call(env.clone(), $(self.$arg,)*);
                }
            }
        );
    }
}

#[macro_export]
macro_rules! parametric_rule {
    ($f:ident, ($($call:ident),+)) => {
        $(paste::paste!(
            #[no_mangle]
            pub fn [< $f _ $call >](e: Env, c: [< call_ $call >]) {
                $f::<[< call_ $call >]>(e, c);
            }
        );)+
    };
}

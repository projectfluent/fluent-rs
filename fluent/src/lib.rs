pub use fluent_bundle::{FluentArgs, FluentBundle, FluentMessage, FluentResource, FluentValue};

#[macro_export]
macro_rules! fluent_args {
    ( $($key:expr => $value:expr),* ) => {
        {
            let mut args: ::fluent_bundle::FluentArgs = ::fluent_bundle::FluentArgs::new();
            $(
                args.insert($key, $value.into());
            )*
            args
        }
    };
}

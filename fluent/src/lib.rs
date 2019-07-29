pub use fluent_bundle::{FluentBundle, FluentResource, FluentValue, FluentArgs, FluentMessage};

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

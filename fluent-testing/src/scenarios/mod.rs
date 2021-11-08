mod browser;
mod preferences;
mod simple;
pub mod structs;

use structs::*;

#[macro_export]
macro_rules! queries {
    ( $( $x:expr ),* ) => {
        {
            Queries(vec![
                $(
                    $x.into(),
                )*
            ])
        }
    };
}

pub fn get_scenarios() -> Vec<Scenario> {
    vec![
        simple::get_scenario(),
        preferences::get_scenario(),
        browser::get_scenario(),
    ]
}

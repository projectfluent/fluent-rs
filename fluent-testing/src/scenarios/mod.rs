mod browser;
mod preferences;
mod simple;

use crate::*;

#[macro_export]
macro_rules! queries {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Query> = Vec::new();
            $(
                temp_vec.push($x.into());
            )*
            Queries(temp_vec)
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

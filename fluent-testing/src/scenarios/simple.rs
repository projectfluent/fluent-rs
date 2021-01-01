use super::structs::*;
use crate::files::FileSource;
use crate::queries;

pub fn get_scenario() -> Scenario {
    Scenario::new(
        "simple",
        vec![FileSource::new(
            "browser",
            "browser/{locale}/",
            vec!["en-US"],
        )],
        vec!["en-US"],
        vec!["browser/sanitize.ftl"],
        queries![("history-section-label", "History")],
    )
}

use crate::*;

pub fn get_scenario() -> Scenario {
    Scenario::new(
        "simple",
        vec![FileSource::new(
            "browser",
            "browser/{locale}/",
            vec!["en-US"],
            None,
        )],
        vec!["en-US"],
        vec!["browser/sanitize.ftl"],
        queries![("history-section-label", "History")],
    )
}

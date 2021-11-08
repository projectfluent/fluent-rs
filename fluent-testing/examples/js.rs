use fluent_testing::scenarios::get_scenarios;

fn main() {
    for scenario in get_scenarios() {
        if scenario.name != "browser" {
            continue;
        }

        println!("let keys = [");
        for query in scenario.queries.iter() {
            println!("  {{id: \"{}\", args: null}},", query.input.id);
        }
        println!("];");
    }
}

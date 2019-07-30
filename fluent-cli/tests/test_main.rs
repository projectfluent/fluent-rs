use fluent_cli::parse_file;

#[test]
fn test_main() {
    parse_file("./tests/fixture.ftl", false);
}

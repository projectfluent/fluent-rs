mod helpers;

use self::helpers::{
    assert_format_no_errors, assert_get_bundle_no_errors, assert_get_resource_from_str_no_errors,
};

#[test]
fn variant_expression() {
    let res = assert_get_resource_from_str_no_errors(
        "
-foo = Foo
-bar =
    {
       *[nominative] Bar
        [genitive] Bar's
    }
baz = { -bar }

use-foo = { -foo }
use-foo-missing = { -foo[missing] }

use-bar = { -bar }
use-bar-nominative = { -bar[nominative] }
use-bar-genitive = { -bar[genitive] }
use-bar-missing = { -bar[missing] }

missing-missing = { -missing[missing] }
    ",
    );
    let bundle = assert_get_bundle_no_errors(&res, None);

    assert_format_no_errors(bundle.format("baz", None), "Bar");

    assert_format_no_errors(bundle.format("use-foo", None), "Foo");

    assert_format_no_errors(bundle.format("use-foo-missing", None), "Foo");

    assert_format_no_errors(bundle.format("use-bar", None), "Bar");

    assert_format_no_errors(bundle.format("use-bar-nominative", None), "Bar");

    assert_format_no_errors(bundle.format("use-bar-genitive", None), "Bar's");

    assert_format_no_errors(bundle.format("use-bar-missing", None), "Bar");

    assert_format_no_errors(bundle.format("missing-missing", None), "___");
}

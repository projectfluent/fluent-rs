/// This is a mock implementation of Unicode Plural Rules.
/// It's meant to be replaced with a real implementation as soon as possible.
extern crate fluent_locale;

use self::fluent_locale::negotiate_languages;
use self::fluent_locale::NegotiationStrategy;

static LOCALES: &[&'static str] = &[
    "af", "ak", "am", "ar", "asa", "az", "be", "bem", "bez", "bg", "bh", "bm", "bn", "bo", "br",
    "brx", "bs", "ca", "cgg", "chr", "cs", "cy", "da", "de", "dv", "dz", "ee", "el", "en", "eo",
    "es", "et", "eu", "fa", "ff", "fi", "fil", "fo", "fr", "fur", "fy", "ga", "gd", "gl", "gsw",
    "gu", "guw", "gv", "ha", "haw", "he", "hi", "hr", "hu", "id", "ig", "ii", "is", "it", "iu",
    "ja", "jmc", "jv", "ka", "kab", "kaj", "kcg", "kde", "kea", "kk", "kl", "km", "kn", "ko",
    "ksb", "ksh", "ku", "kw", "lag", "lb", "lg", "ln", "lo", "lt", "lv", "mas", "mg", "mk", "ml",
    "mn", "mo", "mr", "ms", "mt", "my", "nah", "naq", "nb", "nd", "ne", "nl", "nn", "no", "nr",
    "nso", "ny", "nyn", "om", "or", "pa", "pap", "pl", "ps", "pt", "rm", "ro", "rof", "ru", "rwk",
    "sah", "saq", "se", "seh", "ses", "sg", "sh", "shi", "sk", "sl", "sma", "smi", "smj", "smn",
    "sms", "sn", "so", "sq", "sr", "ss", "ssy", "st", "sv", "sw", "syr", "ta", "te", "teo", "th",
    "ti", "tig", "tk", "tl", "tn", "to", "tr", "ts", "tzm", "uk", "ur", "ve", "vi", "vun", "wa",
    "wae", "wo", "xh", "xog", "yo", "zh", "zu",
];

fn is_between(n: f32, start: f32, end: f32) -> bool {
    start <= n && n <= end
}

fn is_in(n: f32, list: &[f32]) -> bool {
    list.contains(&n)
}

static PLURAL_RULES: &[fn(f32) -> &'static str] = &[
    /* 0 */
    |_| "other",
    /* 1 */
    |n| {
        if is_between(n % 100.0, 3.0, 10.0) {
            return "few";
        }
        if n == 0.0 {
            return "zero";
        }
        if is_between(n % 100.0, 11.0, 99.0) {
            return "many";
        }
        if n == 2.0 {
            return "two";
        }
        if n == 1.0 {
            return "one";
        }
        "other"
    },
    /* 2 */
    |n| {
        if n != 0.0 && (n % 10.0) == 0.0 {
            return "many";
        }
        if n == 2.0 {
            return "two";
        }
        if n == 1.0 {
            return "one";
        }
        "other"
    },
    /* 3 */
    |n| if n == 1.0 { "one" } else { "other" },
    /* 4 */
    |n| {
        if is_between(n, 0.0, 1.0) {
            "one"
        } else {
            "other"
        }
    },
    /* 5 */
    |n| {
        if is_between(n, 0.0, 2.0) {
            "one"
        } else {
            "other"
        }
    },
    /* 6 */
    |n| {
        if n == 0.0 {
            return "zero";
        }
        if (n % 10.0) == 1.0 && (n % 100.0) != 11.0 {
            return "one";
        }
        "other"
    },
    /* 7 */
    |n| {
        if n == 2.0 {
            return "two";
        }
        if n == 1.0 {
            return "one";
        }
        "other"
    },
    /* 8 */
    |n| {
        if is_between(n, 3.0, 6.0) {
            return "few";
        }
        if is_between(n, 7.0, 10.0) {
            return "many";
        }
        if n == 2.0 {
            return "two";
        }
        if n == 1.0 {
            return "one";
        }
        "other"
    },
    /* 9 */
    |n| {
        if n == 0.0 || n != 1.0 && is_between(n % 100.0, 1.0, 19.0) {
            return "few";
        }
        if n == 1.0 {
            return "one";
        }
        "other"
    },
    /* 10 */
    |n| {
        if is_between(n % 10.0, 2.0, 9.0) && !is_between(n % 100.0, 11.0, 19.0) {
            return "few";
        }
        if n % 10.0 == 1.0 && !is_between(n % 100.0, 11.0, 19.0) {
            return "one";
        }
        "other"
    },
    /* 11 */
    |n| {
        if is_between(n % 10.0, 2.0, 4.0) && !is_between(n % 100.0, 12.0, 14.0) {
            return "few";
        }
        if n % 10.0 == 0.0 || is_between(n % 10.0, 5.0, 9.0) || is_between(n % 100.0, 11.0, 14.0) {
            return "many";
        }
        if n % 10.0 == 1.0 && n % 100.0 != 11.0 {
            return "one";
        }
        "other"
    },
    /* 12 */
    |n| {
        if is_between(n, 2.0, 4.0) {
            return "few";
        }
        if n == 1.0 {
            return "one";
        }
        "other"
    },
    /* 13 */
    |n| {
        if is_between(n % 10.0, 2.0, 4.0) && !is_between(n % 100.0, 12.0, 14.0) {
            return "few";
        }
        if n != 1.0 && is_between(n % 10.0, 0.0, 1.0)
            || is_between(n % 10.0, 5.0, 9.0)
            || is_between(n % 100.0, 12.0, 14.0)
        {
            return "many";
        }
        if n == 1.0 {
            return "one";
        }
        "other"
    },
    /* 14 */
    |n| {
        if is_between(n % 100.0, 3.0, 4.0) {
            return "few";
        }
        if n % 100.0 == 2.0 {
            return "two";
        }
        if n % 100.0 == 1.0 {
            return "one";
        }
        "other"
    },
    /* 15 */
    |n| {
        if n == 0.0 || is_between(n % 100.0, 2.0, 10.0) {
            return "few";
        }
        if is_between(n % 100.0, 11.0, 19.0) {
            return "many";
        }
        if n == 1.0 {
            return "one";
        }
        "other"
    },
    /* 16 */
    |n| {
        if n % 10.0 == 1.0 && n != 11.0 {
            "one"
        } else {
            "other"
        }
    },
    /* 17 */
    |n| {
        if n == 3.0 {
            return "few";
        }
        if n == 0.0 {
            return "zero";
        }
        if n == 6.0 {
            return "many";
        }
        if n == 2.0 {
            return "two";
        }
        if n == 1.0 {
            return "one";
        }
        "other"
    },
    /* 18 */
    |n| {
        if n == 0.0 {
            return "zero";
        }
        if is_between(n, 0.0, 2.0) && n != 0.0 && n != 2.0 {
            return "one";
        }
        "other"
    },
    /* 19 */
    |n| {
        if is_between(n, 2.0, 10.0) {
            return "few";
        }
        if is_between(n, 0.0, 1.0) {
            return "one";
        }
        "other"
    },
    /* 20 */
    |n| {
        if (is_between(n % 10.0, 3.0, 4.0) || n % 10.0 == 9.0)
            && !(is_between(n % 100.0, 10.0, 19.0)
                || is_between(n % 100.0, 70.0, 79.0)
                || is_between(n % 100.0, 90.0, 99.0))
        {
            return "few";
        }
        if n % 1000000.0 == 0.0 && n != 0.0 {
            return "many";
        }
        if n % 10.0 == 2.0 && !is_in(n % 100.0, &[12.0, 72.0, 92.0]) {
            return "two";
        }
        if n % 10.0 == 1.0 && !is_in(n % 100.0, &[11.0, 71.0, 91.0]) {
            return "one";
        }
        "other"
    },
    /* 21 */
    |n| {
        if n == 0.0 {
            return "zero";
        }
        if n == 1.0 {
            return "one";
        }
        "other"
    },
    /* 22 */
    |n| {
        if is_between(n, 0.0, 1.0) || is_between(n, 11.0, 99.0) {
            return "one";
        }
        "other"
    },
    /* 23 */
    |n| {
        if is_between(n, 0.0, 1.0) || is_between(n, 11.0, 99.0) {
            "one"
        } else {
            "other"
        }
    },
    /* 24 */
    |n| {
        if is_between(n, 3.0, 10.0) || is_between(n, 13.0, 19.0) {
            return "few";
        }
        if is_in(n, &[2.0, 12.0]) {
            return "two";
        }
        if is_in(n, &[1.0, 11.0]) {
            return "one";
        }
        "other"
    },
];

fn get_plural_rule(loc: &str) -> Option<usize> {
    let num = match loc {
        "af" => 3,
        "ak" => 4,
        "am" => 4,
        "ar" => 1,
        "asa" => 3,
        "az" => 0,
        "be" => 11,
        "bem" => 3,
        "bez" => 3,
        "bg" => 3,
        "bh" => 4,
        "bm" => 0,
        "bn" => 3,
        "bo" => 0,
        "br" => 20,
        "brx" => 3,
        "bs" => 11,
        "ca" => 3,
        "cgg" => 3,
        "chr" => 3,
        "cs" => 12,
        "cy" => 17,
        "da" => 3,
        "de" => 3,
        "dv" => 3,
        "dz" => 0,
        "ee" => 3,
        "el" => 3,
        "en" => 3,
        "eo" => 3,
        "es" => 3,
        "et" => 3,
        "eu" => 3,
        "fa" => 0,
        "ff" => 5,
        "fi" => 3,
        "fil" => 4,
        "fo" => 3,
        "fr" => 5,
        "fur" => 3,
        "fy" => 3,
        "ga" => 8,
        "gd" => 24,
        "gl" => 3,
        "gsw" => 3,
        "gu" => 3,
        "guw" => 4,
        "gv" => 23,
        "ha" => 3,
        "haw" => 3,
        "he" => 2,
        "hi" => 4,
        "hr" => 11,
        "hu" => 0,
        "id" => 0,
        "ig" => 0,
        "ii" => 0,
        "is" => 3,
        "it" => 3,
        "iu" => 7,
        "ja" => 0,
        "jmc" => 3,
        "jv" => 0,
        "ka" => 0,
        "kab" => 5,
        "kaj" => 3,
        "kcg" => 3,
        "kde" => 0,
        "kea" => 0,
        "kk" => 3,
        "kl" => 3,
        "km" => 0,
        "kn" => 0,
        "ko" => 0,
        "ksb" => 3,
        "ksh" => 21,
        "ku" => 3,
        "kw" => 7,
        "lag" => 18,
        "lb" => 3,
        "lg" => 3,
        "ln" => 4,
        "lo" => 0,
        "lt" => 10,
        "lv" => 6,
        "mas" => 3,
        "mg" => 4,
        "mk" => 16,
        "ml" => 3,
        "mn" => 3,
        "mo" => 9,
        "mr" => 3,
        "ms" => 0,
        "mt" => 15,
        "my" => 0,
        "nah" => 3,
        "naq" => 7,
        "nb" => 3,
        "nd" => 3,
        "ne" => 3,
        "nl" => 3,
        "nn" => 3,
        "no" => 3,
        "nr" => 3,
        "nso" => 4,
        "ny" => 3,
        "nyn" => 3,
        "om" => 3,
        "or" => 3,
        "pa" => 3,
        "pap" => 3,
        "pl" => 13,
        "ps" => 3,
        "pt" => 3,
        "rm" => 3,
        "ro" => 9,
        "rof" => 3,
        "ru" => 11,
        "rwk" => 3,
        "sah" => 0,
        "saq" => 3,
        "se" => 7,
        "seh" => 3,
        "ses" => 0,
        "sg" => 0,
        "sh" => 11,
        "shi" => 19,
        "sk" => 12,
        "sl" => 14,
        "sma" => 7,
        "smi" => 7,
        "smj" => 7,
        "smn" => 7,
        "sms" => 7,
        "sn" => 3,
        "so" => 3,
        "sq" => 3,
        "sr" => 11,
        "ss" => 3,
        "ssy" => 3,
        "st" => 3,
        "sv" => 3,
        "sw" => 3,
        "syr" => 3,
        "ta" => 3,
        "te" => 3,
        "teo" => 3,
        "th" => 0,
        "ti" => 4,
        "tig" => 3,
        "tk" => 3,
        "tl" => 4,
        "tn" => 3,
        "to" => 0,
        "tr" => 0,
        "ts" => 3,
        "tzm" => 22,
        "uk" => 11,
        "ur" => 3,
        "ve" => 3,
        "vi" => 0,
        "vun" => 3,
        "wa" => 4,
        "wae" => 3,
        "wo" => 0,
        "xh" => 3,
        "xog" => 3,
        "yo" => 0,
        "zh" => 0,
        "zu" => 3,
        _ => return None,
    };
    Some(num)
}

pub struct PluralRules {
    pub locale: String,
    selector: Box<Fn(f32) -> &'static str>,
}

impl PluralRules {
    pub fn new(locales: &[&str]) -> PluralRules {
        let supported =
            negotiate_languages(locales, LOCALES, Some("en"), &NegotiationStrategy::Lookup);

        let locale = supported[0].to_owned();
        let f = match get_plural_rule(supported[0]) {
            Some(n) => Box::new(PLURAL_RULES[n]),
            None => unimplemented!("Plural rule for this language is not available"),
        };
        PluralRules {
            locale: locale,
            selector: f,
        }
    }

    pub fn select(&self, num: f32) -> &'static str {
        (self.selector)(num)
    }
}

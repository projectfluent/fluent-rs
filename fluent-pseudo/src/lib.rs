use regex::Captures;
use regex::Regex;
use std::borrow::Cow;

static TRANSFORM_SMALL_MAP: &[char] = &[
    'ȧ', 'ƀ', 'ƈ', 'ḓ', 'ḗ', 'ƒ', 'ɠ', 'ħ', 'ī', 'ĵ', 'ķ', 'ŀ', 'ḿ', 'ƞ', 'ǿ',
    'ƥ', 'ɋ', 'ř', 'ş', 'ŧ', 'ŭ', 'ṽ', 'ẇ', 'ẋ', 'ẏ', 'ẑ',
];
static TRANSFORM_CAPS_MAP: &[char] = &[
    'Ȧ', 'Ɓ', 'Ƈ', 'Ḓ', 'Ḗ', 'Ƒ', 'Ɠ', 'Ħ', 'Ī', 'Ĵ', 'Ķ', 'Ŀ', 'Ḿ', 'Ƞ', 'Ǿ',
    'Ƥ', 'Ɋ', 'Ř', 'Ş', 'Ŧ', 'Ŭ', 'Ṽ', 'Ẇ', 'Ẋ', 'Ẏ', 'Ẑ',
];

pub fn transform(s: &str) -> Cow<str> {
    let re_az = Regex::new(r"[a-zA-Z]").unwrap();

    re_az.replace_all(s, |caps: &Captures| {
        let ch = caps[0].chars().nth(0).unwrap();
        let cc = ch as u8;
        if cc >= 97 && cc <= 122 {
            let pos = cc - 97;
            let new_char = TRANSFORM_SMALL_MAP[pos as usize];
            new_char.to_string()
        } else if cc >= 65 && cc <= 90 {
            let pos = cc - 65;
            let new_char = TRANSFORM_CAPS_MAP[pos as usize];
            new_char.to_string()
        } else {
            ch.to_string()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn it_works() {
        let x = transform("Hello World");
        assert_eq!(x, "Ħḗŀŀǿ Ẇǿřŀḓ");
    }
}

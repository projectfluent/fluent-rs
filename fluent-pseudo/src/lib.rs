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

static FLIPPED_SMALL_MAP: &[char] = &[
    'ɐ', 'q', 'ɔ', 'p', 'ǝ', 'ɟ', 'ƃ', 'ɥ', 'ı', 'ɾ', 'ʞ', 'ʅ', 'ɯ', 'u', 'o', 'd', 'b',
    'ɹ', 's', 'ʇ', 'n', 'ʌ', 'ʍ', 'x', 'ʎ', 'z',
];
static FLIPPED_CAPS_MAP: &[char] = &[
    '∀', 'Ԑ', 'Ↄ', 'ᗡ', 'Ǝ', 'Ⅎ', '⅁', 'H', 'I', 'ſ', 'Ӽ', '⅂', 'W', 'N', 'O',
    'Ԁ', 'Ò', 'ᴚ', 'S', '⊥', '∩', 'Ʌ', 'M', 'X', '⅄', 'Z',
];

pub fn transform(s: &str, flipped: bool, elongate: bool) -> Cow<str> {
    let re_az = Regex::new(r"[a-zA-Z]").unwrap();

    let (small_map, caps_map) = if flipped {
        (FLIPPED_SMALL_MAP, FLIPPED_CAPS_MAP)
    } else {
        (TRANSFORM_SMALL_MAP, TRANSFORM_CAPS_MAP)
    };

    re_az.replace_all(s, |caps: &Captures| {
        let ch = caps[0].chars().nth(0).unwrap();
        let cc = ch as u8;
        if cc >= 97 && cc <= 122 {
            let pos = cc - 97;
            let new_char = small_map[pos as usize];
            if elongate && (cc == 97 || cc == 101 || cc == 111 || cc == 117) {
                let mut s = new_char.to_string();
                s.push(new_char);
                s
            } else {
                new_char.to_string()
            }
        } else if cc >= 65 && cc <= 90 {
            let pos = cc - 65;
            let new_char = caps_map[pos as usize];
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
        let x = transform("Hello World", false, true);
        assert_eq!(x, "Ħḗḗŀŀǿǿ Ẇǿǿřŀḓ");

        let x = transform("Hello World", false, false);
        assert_eq!(x, "Ħḗŀŀǿ Ẇǿřŀḓ");

        let x = transform("Hello World", true, false);
        assert_eq!(x, "Hǝʅʅo Moɹʅp");
    }
}

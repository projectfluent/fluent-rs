use std::char;

fn encode_unicode(s: &str) -> char {
    u32::from_str_radix(s, 16)
        .ok()
        .and_then(char::from_u32)
        .unwrap_or('�')
}

pub fn unescape_unicode(string: &str) -> Option<String> {
    let bytes = string.as_bytes();
    let mut result: Option<String> = None;

    let mut ptr = 0;

    while let Some(b) = bytes.get(ptr) {
        if b != &b'\\' {
            if let Some(ref mut result) = result {
                result.push(*b as char);
            }
            ptr += 1;
            continue;
        }

        let new_string = result.get_or_insert_with(|| String::from(&string[0..ptr]));

        ptr += 1;

        let new_char = match bytes.get(ptr) {
            Some(b'\\') => '\\',
            Some(b'"') => '"',
            Some(u @ b'u') | Some(u @ b'U') => {
                let start = ptr + 1;
                let len = if u == &b'u' { 4 } else { 6 };
                ptr += len;
                string
                    .get(start..(start + len))
                    .map(|slice| encode_unicode(slice))
                    .unwrap_or('�')
            }
            _ => '�',
        };
        new_string.push(new_char);
        ptr += 1;
    }
    return result;
}

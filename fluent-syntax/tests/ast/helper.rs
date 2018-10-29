use std::char;
use std::collections::VecDeque;

fn encode_unicode(s: &str, l: usize) -> char {
    if s.len() != l {
        return '�';
    }
    let u = match u32::from_str_radix(s, 16) {
        Ok(u) => u,
        Err(_) => return '�',
    };
    char::from_u32(u).unwrap_or('�')
}

pub fn unescape_unicode(s: &str) -> String {
    let mut queue: VecDeque<_> = String::from(s).chars().collect();
    let mut result = String::new();

    while let Some(c) = queue.pop_front() {
        if c != '\\' {
            result.push(c);
            continue;
        }

        match queue.pop_front() {
            Some('u') => {
                let mut buffer = String::new();
                for _ in 0..4 {
                    if let Some(c) = queue.pop_front() {
                        match c {
                            '0'...'9' | 'a'...'f' | 'A'...'F' => {
                                buffer.push(c);
                            }
                            _ => break,
                        }
                    } else {
                        break;
                    }
                }
                let new_char = encode_unicode(&buffer, 4);
                result.push(new_char);
            }
            Some('U') => {
                let mut buffer = String::new();
                for _ in 0..6 {
                    if let Some(c) = queue.pop_front() {
                        match c {
                            '0'...'9' | 'a'...'f' | 'A'...'F' => {
                                buffer.push(c);
                            }
                            _ => break,
                        }
                    } else {
                        break;
                    }
                }
                let new_char = encode_unicode(&buffer, 6);
                result.push(new_char);
            }
            Some(c) => {
                result.push(c);
            }
            None => break,
        }
    }
    result
}

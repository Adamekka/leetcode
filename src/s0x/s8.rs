pub fn solution() {
    assert_eq!(my_atoi("42".to_string()), 42);
    assert_eq!(my_atoi("   -42".to_string()), -42);
    assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(my_atoi("words and 987".to_string()), 0);
    assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(my_atoi("-+12".to_string()), 0);
    assert_eq!(my_atoi("+1".to_string()), 1);
    assert_eq!(my_atoi("".to_string()), 0);
}

fn my_atoi(s: String) -> i32 {
    let s = s.trim_start();

    let mut result = "".to_string();

    let mut chars = s.chars();

    let first_char = chars.clone().next().unwrap_or_default();
    match first_char {
        '-' => {
            result.push('-');
            chars.next();
        }
        '+' => {
            chars.next();
        }
        _ => (),
    }

    for c in chars {
        if c.is_ascii_digit() {
            result.push(c);
        } else {
            break;
        }
    }

    match result.parse::<i32>() {
        Ok(result) => result,
        Err(err) => match err.kind() {
            std::num::IntErrorKind::PosOverflow => i32::MAX,
            std::num::IntErrorKind::NegOverflow => i32::MIN,
            _ => 0,
        },
    }
}

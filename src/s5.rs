pub fn solution() {
    let result = longest_palindrome("babad".to_string());
    assert!(result == "aba" || result == "bab");

    assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
    assert_eq!(longest_palindrome("a".to_string()), "a");
    assert_eq!(longest_palindrome("bb".to_string()), "bb");
    assert_eq!(longest_palindrome("abb".to_string()), "bb");
}

fn longest_palindrome(s: String) -> String {
    let mut window_size = s.len();
    while window_size > 0 {
        match s.as_bytes().windows(window_size).find(|slice| {
            let iter = slice.iter();
            iter.clone().eq(iter.clone().rev())
        }) {
            Some(slice) => return String::from_utf8(slice.to_vec()).unwrap_or_default(),
            None => window_size -= 1,
        }
    }

    "".to_string()
}

/// Too slow
fn _longest_palindrome(s: String) -> String {
    if s.len() == 1 || is_palindrome(&s) {
        return s;
    }

    let mut longest_palindrome = "";

    for i in 0..s.len() {
        let split = s.split_at(i).1;

        if longest_palindrome.len() > split.len() {
            continue;
        }

        if is_palindrome(split) {
            longest_palindrome = split;
        }

        for j in 0..split.len() {
            let split = split.split_at(j).0;

            if longest_palindrome.len() > split.len() {
                continue;
            }

            if split.len() > longest_palindrome.len() && is_palindrome(split) {
                longest_palindrome = split;
            }
        }
    }

    fn is_palindrome(s: &str) -> bool {
        s == s.chars().rev().collect::<String>()
    }

    longest_palindrome.to_string()
}

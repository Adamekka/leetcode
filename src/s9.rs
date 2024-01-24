pub fn solution() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(-121));
    assert!(!is_palindrome(10));
}

fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }

    x.to_string() == x.to_string().chars().rev().collect::<String>()
}

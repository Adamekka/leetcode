pub fn solution() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
    assert_eq!(reverse(1534236469), 0);
}

fn reverse(x: i32) -> i32 {
    let mut is_negative = false;

    let mut x_str = x.to_string().chars().rev().collect::<String>();

    if x.is_negative() {
        is_negative = true;
        x_str.pop();
    }

    let x = x_str.parse::<i32>().unwrap_or_default();

    if is_negative {
        -x
    } else {
        x
    }
}

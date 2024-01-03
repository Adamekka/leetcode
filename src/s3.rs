pub fn solution() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring("aab".to_string()), 2);
    assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    assert_eq!(length_of_longest_substring("aabaab!bb".to_string()), 3);
}

fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = 0;

    let mut occurred = "".to_string();

    for c in s.chars() {
        if occurred.contains(c) {
            let len = occurred.len();

            if len > longest {
                longest = len
            }

            let idx = occurred.find(c).unwrap();
            occurred = occurred.split_at(idx).1.to_string();
            occurred.remove(0);

            occurred.push(c);
        } else {
            occurred.push(c);
        }
    }

    let len = occurred.len();

    if len > longest {
        longest = len
    }

    longest as i32
}

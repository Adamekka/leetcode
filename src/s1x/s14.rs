pub fn solution() {
    assert_eq!(
        longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        "".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec!["c".to_string(), "acc".to_string(), "ccc".to_string()]),
        "".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec!["ab".to_string(), "a".to_string()]),
        "a".to_string()
    );
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut strs = strs;
    strs.sort();

    let shortest = strs.first().unwrap().as_str();
    let mut prefix = "";

    for i in 1..=shortest.len() {
        let slice = &shortest[0..i];

        if !strs.iter().all(|s| &s[0..i] == slice) {
            break;
        }

        prefix = slice;
    }

    prefix.to_owned()
}

pub fn solution() {
    assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut arr = nums1;
    arr.extend_from_slice(&nums2);
    arr.sort();

    if arr.len() % 2 == 0 {
        let split = arr.split_at(arr.len() / 2);
        let first = *split.0.last().unwrap() as f64;
        let second = *split.1.first().unwrap() as f64;

        (first + second) / 2.0
    } else {
        let idx = arr.len() / 2;

        arr[idx] as f64
    }
}

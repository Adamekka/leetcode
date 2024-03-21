pub fn solution() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (idx_i, i) in nums.iter().enumerate() {
        for (idx_j, j) in nums.iter().enumerate() {
            if idx_i == idx_j {
                continue;
            } else if i + j == target {
                return vec![idx_i as i32, idx_j as i32];
            }
        }
    }

    unreachable!();
}

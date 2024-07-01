impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut arr: Vec<i32> = nums.clone();
        arr.sort();
        let n: usize = arr.len();
        let mut min_diff: i32 = i32::MAX;
        let mut min_sum: i32 = i32::MAX;
        for i in 0..n - 2 {
            let mut j = i + 1;
            let mut k = n - 1;
            while k > j {
                let _sum = arr[i] + arr[j] + arr[k];
                let _diff = _sum - target;
                if _diff == 0 {
                    return _sum;
                } else if _diff < 0 {
                    if min_diff > { -_diff } {
                        min_diff = { -_diff };
                        min_sum = _sum;
                    }
                    j += 1;
                } else {
                    if min_diff > _diff {
                        min_diff = _diff;
                        min_sum = _sum;
                    }
                    k -= 1;
                }
            }
        }
        min_sum
    }
}

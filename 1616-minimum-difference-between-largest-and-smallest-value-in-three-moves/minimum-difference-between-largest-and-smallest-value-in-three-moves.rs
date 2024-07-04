impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        if nums.len() <= 3 {
            return 0;
        }

        let mut min = [nums[0], nums[1], nums[2], nums[3]];
        let mut max = [nums[0], nums[1], nums[2], nums[3]];
        min.sort_unstable();
        max.sort_unstable_by(|a, b| b.cmp(a));

        for num in nums.into_iter().skip(4) {
            if num <= min[3] {
                for i in 0..4 {
                    if num <= min[i] {
                        for j in (i+1..4).rev() {
                            min[j] = min[j-1];
                        }
                        min[i] = num;
                        break;
                    }
                }
            }
            if num >= max[3] {
                for i in 0..4 {
                    if num >= max[i] {
                        for j in (i+1..4).rev() {
                            max[j] = max[j-1];
                        }
                        max[i] = num;
                        break;
                    }
                }
            }
        }
        let mut ans = i32::MAX;
        for l in 0..4 {
            let r = 3 - l;
            ans = ans.min(max[r] - min[l])
        }
        ans
    }
}
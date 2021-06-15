pub fn find(nums: &mut Vec<i32>) -> Vec<[i32; 3]> {
    nums.sort();
    // nums.dedup();

    let len = nums.len();
    if len < 3 {
        return vec![];
    }

    assert!(len < 3000);

    assert!((nums[0] >= -100000) && (nums[len - 1] <= 100000));

    let mut result = vec![];
    nums.iter()
        .filter(|x| **x <= 0)
        .enumerate()
        .for_each(|(i, x)| {
            // skip duplicate data start from second item
            if i == 0 || (i > 0 && nums[i] != nums[i - 1]) {
                let mut j = i + 1;
                let mut k = len - 1;

                // skip duplicate data
                while j < k && nums[j] == nums[j + 1] {
                    j += 1;
                }

                // skip duplicate data
                while j < k && nums[k] == nums[k - 1] {
                    k -= 1;
                }

                while j < k {
                    let sum = nums[j] + nums[k];
                    if sum > -x {
                        k -= 1
                    } else if sum < -x {
                        j += 1
                    } else {
                        result.push([*x, nums[j], nums[k]]);
                        j += 1;
                        k -= 1;
                    }
                }
            }
        });
    result
}

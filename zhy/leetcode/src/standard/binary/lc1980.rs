// 1980. Find Unique Binary String
// Given an array of strings nums containing n unique binary strings each of length n, return a binary string of length n that does not appear in nums. If there are multiple answers, you may return any of them.

 

// Example 1:

// Input: nums = ["01","10"]
// Output: "11"
// Explanation: "11" does not appear in nums. "00" would also be correct.
// Example 2:

// Input: nums = ["00","01"]
// Output: "11"
// Explanation: "11" does not appear in nums. "10" would also be correct.
// Example 3:

// Input: nums = ["111","011","001"]
// Output: "101"
// Explanation: "101" does not appear in nums. "000", "010", "100", and "110" would also be correct.
 

// Constraints:

// n == nums.length
// 1 <= n <= 16
// nums[i].length == n
// nums[i] is either '0' or '1'.
// All the strings of nums are unique.

use std::collections::HashSet;
use std::iter::FromIterator;
pub struct Solution;
impl Solution {
    ///
    /// ```
    /// 
    /// use leetcode::standard::binary::lc1980::{Solution,str_vec_to_string_vec};
    /// assert_eq!("000",Solution::find_different_binary_string(str_vec_to_string_vec(["111","011","001"].to_vec())));
    /// ```
    /// 
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums[0].len();
        let numset :HashSet<String> =HashSet::from_iter(nums.into_iter());
        for i in 0..1<<n{
            let mut istr = format!("{:b}",i);
            istr = format!("{:0width$}",istr.parse::<i32>().unwrap(),width = n);
            if !numset.contains(&istr){
                return istr.to_string();
            }
        }
        return "".to_string();
    }
}
pub fn str_vec_to_string_vec(v:Vec<&str>)->Vec<String>{
    return v.iter().fold(Vec::new(), |mut v, &x| {v.push(x.to_string());v});
}
#[cfg(test)]
mod tests {
    use super::str_vec_to_string_vec;
    use super::Solution;
    #[test]
    fn lc1980() {
        assert_eq!("101",Solution::find_different_binary_string(str_vec_to_string_vec(["111","011","001"].to_vec())));
        assert_eq!("11",Solution::find_different_binary_string(str_vec_to_string_vec(["01","10"].to_vec())));
        assert_eq!("11",Solution::find_different_binary_string(str_vec_to_string_vec(["00","01"].to_vec())));
    }
}
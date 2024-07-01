pub struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut output = vec![];
        let mut sum = 0;

        for value in nums {
            sum += value;
            output.push(sum);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = vec![1, 2, 3, 4];
        let expected = [1, 3, 6, 10];
        assert_eq!(Solution::running_sum(input), expected);
    }

    #[test]
    fn case_2() {
        let input = vec![1, 1, 1, 1, 1];
        let expected = [1, 2, 3, 4, 5];
        assert_eq!(Solution::running_sum(input), expected);
    }

    #[test]
    fn case_3() {
        let input = vec![3, 1, 2, 10, 1];
        let expected = [3, 4, 6, 16, 17];
        assert_eq!(Solution::running_sum(input), expected);
    }
}

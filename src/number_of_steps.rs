pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut n = num;
        let mut steps = 0;

        loop {
            if n == 0 {
                break;
            }

            if n % 2 == 0 {
                n /= 2;
            } else {
                n -= 1;
            };

            steps += 1;
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = 14;
        let expected = 6;
        assert_eq!(Solution::number_of_steps(input), expected);
    }

    #[test]
    fn case_2() {
        let input = 8;
        let expected = 4;
        assert_eq!(Solution::number_of_steps(input), expected);
    }

    #[test]
    fn case_3() {
        let input = 123;
        let expected = 12;
        assert_eq!(Solution::number_of_steps(input), expected);
    }
}

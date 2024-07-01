pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut arr = vec![];
        let mut x = 1;
        let n_plus_1 = n + 1;

        loop {
            let is_divisible_by3 = x % 3 == 0;
            let is_divisible_by5 = x % 5 == 0;

            if is_divisible_by3 && is_divisible_by5 {
                arr.push("FizzBuzz".to_string());
            } else if is_divisible_by3 {
                arr.push("Fizz".to_string());
            } else if is_divisible_by5 {
                arr.push("Buzz".to_string());
            } else {
                arr.push(format!("{x}"));
            }

            x += 1;

            if x == n_plus_1 {
                break;
            }
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = 15;
        let expected = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ];
        assert_eq!(Solution::fizz_buzz(input), expected);
    }

    #[test]
    fn case_2() {
        let input = 5;
        let expected = vec!["1", "2", "Fizz", "4", "Buzz"];
        assert_eq!(Solution::fizz_buzz(input), expected);
    }

    #[test]
    fn case_3() {
        let input = 3;
        let expected = vec!["1", "2", "Fizz"];
        assert_eq!(Solution::fizz_buzz(input), expected);
    }
}

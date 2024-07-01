pub struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut richest = 0;

        for balances in accounts {
            let mut total_balance = 0;
            for balance in balances {
                total_balance += balance;
            }

            if total_balance > richest {
                richest = total_balance
            }
        }

        richest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = vec![vec![1, 2, 3], vec![3, 2, 1]];
        let expected = 6;
        assert_eq!(Solution::maximum_wealth(input), expected);
    }

    #[test]
    fn case_2() {
        let input = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        let expected = 10;
        assert_eq!(Solution::maximum_wealth(input), expected);
    }

    #[test]
    fn case_3() {
        let input = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        let expected = 17;
        assert_eq!(Solution::maximum_wealth(input), expected);
    }
}

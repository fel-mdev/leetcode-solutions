pub struct Solution;
impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut strings = Vec::with_capacity(target.len());

        let mut expected_next = 1;
        for num in target.into_iter() {
            let diff = num - expected_next;
            for _ in 0..diff {
                strings.push("Push".to_owned());
                strings.push("Pop".to_owned());
            }

            strings.push("Push".to_owned());
            expected_next = num + 1;
        }

        strings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = vec![1, 3];
        let mut expected = vec![];

        let output = Solution::build_array(input.clone(), 3);

        let mut n = 1;
        for action in output {
            if action == "Push" {
                expected.push(n);
                n += 1;
            } else if action == "Pop" {
                expected.pop();
            } else {
                assert!(false, "invalid action");
            }
        }

        assert_eq!(input, expected);
    }

    #[test]
    fn case_2() {
        let input = vec![1, 2, 3];
        let mut expected = vec![];

        let output = Solution::build_array(input.clone(), 3);

        let mut n = 1;
        for action in output {
            if action == "Push" {
                expected.push(n);
                n += 1;
            } else if action == "Pop" {
                expected.pop();
            } else {
                assert!(false, "invalid action");
            }
        }

        assert_eq!(input, expected);
    }

    #[test]
    fn case_3() {
        let input = vec![1, 2];
        let mut expected = vec![];

        let output = Solution::build_array(input.clone(), 2);

        let mut n = 1;
        for action in output {
            if action == "Push" {
                expected.push(n);
                n += 1;
            } else if action == "Pop" {
                expected.pop();
            } else {
                assert!(false, "invalid action");
            }
        }

        assert_eq!(input, expected);
    }
}

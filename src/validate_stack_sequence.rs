pub struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut temp = vec![];
        let mut popped_index = 0;

        for n in pushed.iter() {
            temp.push(n);

            for p in popped_index..popped.len() {
                match temp.last() {
                    Some(x) if **x == popped[p] => {
                        temp.pop();
                        popped_index += 1;
                    }
                    _ => break,
                }
            }
        }

        return temp.len() == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input_0 = vec![1, 2, 3, 5, 4];
        let input_1 = vec![1, 2, 4, 3, 5];
        let expected = false;

        assert_eq!(
            Solution::validate_stack_sequences(input_0, input_1),
            expected
        );
    }

    #[test]
    fn case_2() {
        let input_0 = vec![1, 2, 4, 5, 3];
        let input_1 = vec![1, 2, 5, 3, 4];
        let expected = true;

        assert_eq!(
            Solution::validate_stack_sequences(input_0, input_1),
            expected
        );
    }

    #[test]
    fn case_3() {
        let input_0 = vec![1, 2, 5, 4, 3];
        let input_1 = vec![1, 3, 2, 4, 5];
        let expected = false;

        assert_eq!(
            Solution::validate_stack_sequences(input_0, input_1),
            expected
        );
    }

    #[test]
    fn case_4() {
        let input_0 = vec![1, 3, 2, 5, 4];
        let input_1 = vec![1, 3, 4, 2, 5];
        let expected = false;

        assert_eq!(
            Solution::validate_stack_sequences(input_0, input_1),
            expected
        );
    }

    #[test]
    fn case_5() {
        let input_0 = vec![1, 3, 4, 5, 2];
        let input_1 = vec![1, 3, 5, 2, 4];
        let expected = true;

        assert_eq!(
            Solution::validate_stack_sequences(input_0, input_1),
            expected
        );
    }

    #[test]
    fn case_6() {
        let input_0 = vec![1, 3, 5, 4, 2];
        let input_1 = vec![1, 4, 2, 3, 5];
        let expected = false;

        assert_eq!(
            Solution::validate_stack_sequences(input_0, input_1),
            expected
        );
    }

    #[test]
    fn case_7() {
        let input_0 = vec![1, 4, 2, 5, 3];
        let input_1 = vec![1, 4, 3, 2, 5];
        let expected = false;

        assert_eq!(
            Solution::validate_stack_sequences(input_0, input_1),
            expected
        );
    }
}

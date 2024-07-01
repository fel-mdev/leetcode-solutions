use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut char_map: HashMap<char, u32> = HashMap::new();

        for c in magazine.chars() {
            *char_map.entry(c).or_default() += 1;
        }

        for c in ransom_note.chars() {
            match char_map.get_mut(&c) {
                Some(n) if *n > 0 => *n -= 1,
                _ => return false,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input_a = "a".to_owned();
        let input_b = "b".to_owned();
        let expected = false;
        assert_eq!(Solution::can_construct(input_a, input_b), expected);
    }

    #[test]
    fn case_2() {
        let input_a = "aa".to_owned();
        let input_b = "ab".to_owned();
        let expected = false;
        assert_eq!(Solution::can_construct(input_a, input_b), expected);
    }

    #[test]
    fn case_3() {
        let input_a = "aa".to_owned();
        let input_b = "aab".to_owned();
        let expected = true;
        assert_eq!(Solution::can_construct(input_a, input_b), expected);
    }
}

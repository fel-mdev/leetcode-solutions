// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: Option::None,
            val,
        }
    }

    pub fn insert(&mut self, val: i32) {
        let mut next = &mut self.next;
        loop {
            match next {
                None => {
                    *next = Option::Some(Box::new(ListNode::new(val)));
                    break;
                }
                Some(next_node) => {
                    next = &mut next_node.next;
                }
            }
        }
    }
}

pub struct Solution;
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;

        while let (Some(node), Some(next_node)) = (slow, fast) {
            if let Some(ref next_fast_node) = next_node.next {
                slow = &node.next;
                fast = &next_fast_node.next;
            } else {
                break;
            }
        }

        slow.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut input = ListNode::new(1);
        input.insert(2);
        input.insert(3);
        input.insert(4);
        input.insert(5);

        let mut expected = ListNode::new(3);
        expected.insert(4);
        expected.insert(5);

        assert_eq!(
            Solution::middle_node(Option::Some(Box::new(input))),
            Option::Some(Box::new(expected))
        );
    }

    #[test]
    fn case_2() {
        let mut input = ListNode::new(1);
        input.insert(2);
        input.insert(3);
        input.insert(4);
        input.insert(5);
        input.insert(6);

        let mut expected = ListNode::new(4);
        expected.insert(5);
        expected.insert(6);

        assert_eq!(
            Solution::middle_node(Option::Some(Box::new(input))),
            Option::Some(Box::new(expected))
        );
    }
}

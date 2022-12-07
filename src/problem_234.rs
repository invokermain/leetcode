// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_values(values: &[i32]) -> Self {
        if values.is_empty() {
            panic!("values must not be empty");
        }

        let mut current_node = Some(Box::new(ListNode::new(*values.last().unwrap())));
        for val in values[..values.len() - 1].iter().rev() {
            current_node = Some(Box::new(ListNode {
                val: *val,
                next: current_node.clone(),
            }));
        }

        *current_node.unwrap()
    }
}

fn recursive_extract(vec: &mut Vec<i32>, val: Option<Box<ListNode>>) {
    if let Some(boxed_node) = val {
        vec.push(boxed_node.val);
        recursive_extract(vec, boxed_node.next);
    }
}

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return false;
    }

    let mut values = Vec::new();

    recursive_extract(&mut values, head);

    let mut iter = values.iter();

    while let (Some(left), Some(right)) = (iter.next(), iter.next_back()) {
        if left != right {
            return false;
        };
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let linked_list = ListNode::from_values(&[1, 2, 2, 1]);
        assert!(is_palindrome(Some(Box::new(linked_list))));
    }

    #[test]
    fn test_case_2() {
        let linked_list = ListNode::from_values(&[3, 2, 5, 2, 3]);
        assert!(is_palindrome(Some(Box::new(linked_list))));
    }

    #[test]
    fn test_case_3() {
        let linked_list = ListNode::from_values(&[1, 2, 3]);
        assert!(!is_palindrome(Some(Box::new(linked_list))));
    }
}

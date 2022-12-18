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

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut even_count = false;
    let mut current_node = head.clone();
    let mut middle_node = head;

    loop {
        let next_node = current_node.clone().unwrap().next;
        if next_node.is_none() {
            return middle_node;
        }
        even_count = !even_count;
        if even_count {
            middle_node = middle_node.unwrap().next.clone();
        }

        current_node = next_node.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let linked_list = ListNode::from_values(&[1, 2, 3, 4, 5]);
        assert_eq!(
            middle_node(Some(Box::new(linked_list))),
            Some(Box::new(ListNode::from_values(&[3, 4, 5])))
        );
    }

    #[test]
    fn test_case_2() {
        let linked_list = ListNode::from_values(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(
            middle_node(Some(Box::new(linked_list))),
            Some(Box::new(ListNode::from_values(&[4, 5, 6])))
        );
    }
}

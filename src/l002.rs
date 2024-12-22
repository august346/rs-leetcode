#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl From<Vec<i32>> for Box<ListNode> {
    fn from(v: Vec<i32>) -> Self {
        let mut head = ListNode::new(0);
        let mut current = &mut head;

        for value in v {
            current.next = Some(Box::new(ListNode::new(value)));
            current = current.next.as_mut().unwrap();
        }

        head.next.unwrap()
    }
}

impl Into<Vec<i32>> for ListNode {
    fn into(self) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = Some(Box::new(self));

        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }

        vec
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;

        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;

        while l1.is_some() || l2.is_some() || current.val > 10 {
            let mut next = Box::new(ListNode::new(current.val / 10));
            current.val = current.val % 10;

            if let Some(l) = l1.take() {
                next.val += l.val;
                l1 = l.next;
            }
            if let Some(l) = l2.take() {
                next.val += l.val;
                l2 = l.next;
            }

            current.next = Some(next);
            current = current.next.as_mut().unwrap();
        }

        if current.val >= 10 {
            current.next = Some(Box::new(ListNode::new(current.val / 10)));
            current.val = current.val % 10;
        }

        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case(vec![2,4,3], vec![5,6,4], vec![7,0,8])]
    #[case(vec![9,9,9,9,9,9,9], vec![9,9,9,9], vec![8,9,9,9,0,0,0,1])]
    fn test(#[case] l1: Vec<i32>, #[case] l2: Vec<i32>, #[case] expected: Vec<i32>) {
        let l1 = if !l1.is_empty() {Some(l1.into())} else {None};
        let l2 = if !l2.is_empty() {Some(l2.into())} else {None};
        let result: Vec<i32> = match Solution::add_two_numbers(l1, l2) {
            None => vec![],
            Some(bln) => (*bln).into(),
        };
        assert_eq!(result, expected);
    }
}
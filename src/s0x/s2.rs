pub fn solution() {
    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        ),
        Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None }))
            }))
        })),
    );

    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode::new(0))),
            Some(Box::new(ListNode::new(0)))
        ),
        Some(Box::new(ListNode::new(0)))
    );

    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode::new(9)))
                                }))
                            }))
                        }))
                    }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode::new(9)))
                    }))
                }))
            })),
        ),
        Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode::new(1)))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
    );

    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode::new(5))),
            Some(Box::new(ListNode::new(5))),
        ),
        Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode::new(1)))
        }))
    )
}

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list1 = vec![l1.clone().unwrap().val];
    let mut list2 = vec![l2.clone().unwrap().val];

    let mut node1 = l1.clone();
    let mut node2 = l2.clone();

    let mut list1_end = false;
    let mut list2_end = false;

    loop {
        if list1_end && list2_end {
            break;
        }

        match node1.clone().unwrap().next {
            Some(node) => {
                list1.push(node.val);
                node1 = Some(node);
            }

            None => {
                list1_end = true;
            }
        }

        match node2.clone().unwrap().next {
            Some(node) => {
                list2.push(node.val);
                node2 = Some(node);
            }

            None => {
                list2_end = true;
            }
        }
    }

    let mut len = std::cmp::max(list1.len(), list2.len());

    let mut carrying: bool = false;
    let mut val = list1[0] + list2[0];
    if val > 9 {
        val -= 10;
        carrying = true;
    }

    let mut sum: Box<ListNode> = Box::new(ListNode::new(val));

    if carrying && len == 1 {
        len += 1;
    }

    for i in 1..len {
        let mut val = list1.get(i).unwrap_or(&0) + list2.get(i).unwrap_or(&0);
        if carrying {
            val += 1;
            carrying = false;
        }

        if val > 9 {
            val -= 10;
            carrying = true;
        }

        sum.append(val);

        if carrying && i == len - 1 {
            sum.append(1);
        }
    }

    impl ListNode {
        fn append(&mut self, val: i32) {
            match self.next {
                Some(ref mut next) => next.append(val),
                None => {
                    let node = ListNode::new(val);
                    self.next = Some(Box::new(node));
                }
            }
        }
    }

    Some(sum)
}

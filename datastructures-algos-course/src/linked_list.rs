#[derive(Debug)]
pub struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T: std::ops::AddAssign> ListNode<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

pub fn linked_list_example() {
    let mut linked_list = ListNode {
        data: 3,
        next: Some(Box::new(ListNode {
            data: 2,
            next: None,
        })),
    };

    if let Some(ref mut v) = linked_list.next {
        v.add_up(10)
    }

    println!("{:?}", linked_list);

    let mut v: Vec<String> = Vec::with_capacity(100);
    v.push("hello".to_string());
    v.push("goodbye".to_string());

    for i in 0..105 {
            v.push(i.to_string());
    }

    println!("v.len = {}, v.capacity = {}", v.len(), v.capacity());
}

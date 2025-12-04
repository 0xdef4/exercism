pub struct SimpleLinkedList<T: Copy> {
    head: Option<Box<Node<T>>>,
    len: usize
}

#[derive(PartialEq)]
struct Node<T: Copy> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T: Copy> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            len: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        let node = Node {
            data: _element,
            next: self.head.take()
        };
        
        self.head = Some(Box::new(node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();

        match head {
            Some(head) => {
                self.head = head.next;
                self.len -= 1;
                return Some(head.data);
            },
            None => {return None}
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|s| &s.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut head = self.head;

        while let Some(_head) = head {
            list.push(_head.data);
            head = _head.next
        }
        list
    }
}

impl<T: Copy> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for e in _iter {
            list.push(e);
        }

        list
    }
}

impl<T: Copy> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut output = Vec::new();

       let mut cur_node = _linked_list.rev().head;
        while let Some(node) = cur_node {
            output.push(node.data);
            cur_node = node.next;
        }

        output
    }
}

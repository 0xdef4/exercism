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

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
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

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T: Copy> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut output = Vec::new();

        for _ in 0.._linked_list.len {
            output.push(_linked_list.pop().unwrap());
        }

        output.reverse();
        output
    }
}

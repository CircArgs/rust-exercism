use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    len: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { len: 0, head: None }
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
        self.len += 1;
        let head = self.head.take();
        self.head = Some(Box::new(Node {
            data: _element,
            next: head,
        }));
    }
    fn push_box_node(&mut self, mut _box_node: Box<Node<T>>) {
        self.len += 1;
        let head = self.head.take();
        _box_node.next = head;
        self.head = Some(_box_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let mut head = self.head.take();
        self.head = head.as_mut().and_then(|h| h.next.take());
        head.map(|h| h.data)
    }
    fn pop_box_node(&mut self) -> Option<Box<Node<T>>> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let mut head = self.head.take();
        self.head = head.as_mut().and_then(|h| h.next.take());
        head.take()
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|h| &h.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut ret = Self::new();
        while let Some(bn) = self.pop_box_node() {
            ret.push_box_node(bn);
        }
        ret
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ret = Self::new();
        for e in _iter {
            ret.push(e);
        }
        ret
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut ret = vec![];
        let mut curr = self.head.as_ref();
        while let Some(n) = curr {
            ret.push(n.data.clone());
            curr = n.next.as_ref();
        }
        ret.reverse();
        ret
    }
}

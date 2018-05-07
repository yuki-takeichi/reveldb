use std::mem::swap;

struct Node<Key> {
    key: Key,
    // XXX Change it AtomicPtr<Node> later.
    next: Option<Box<Node<Key>>>,
}

impl<Key> Node<Key> {
    pub fn new(key: Key, height: u8) -> Box<Node<Key>> {
        Box::new(Node {
            key: key,
            next: None,
        })
    }

    fn next(&self, height: u8) -> Option<&Node<Key>> {
        match self.next {
            Some(ref next) => Some(next),
            None => None,
        }
    }
    fn set_next(&mut self, height: u8, node: Box<Node<Key>>) {
        self.next = Some(node);
    }
}

// Use TypedArena inside
pub struct SkipList<Key> {
    head: Box<Node<Key>>,
    maxHeight: u8,
}

impl<Key> SkipList<Key>
where
    Key: Default,
{
    pub fn new(maxHeight: u8) -> SkipList<Key> {
        // XXX currently limit maxHeight is 1 so that SkipList has the same implementation as naive linked list.
        let maxHeight = 1;

        // XXX ensure Key::default() is -Infinity.
        let smallest = Key::default();
        SkipList {
            head: Node::new(smallest, maxHeight),
            maxHeight: 1,
        }
    }

    pub fn insert(&mut self, key: Key) {
        // XXX stub impl
        // TODO: impl FindGreaterOrEqual()-like fn
        let prev = &mut self.head;
        if prev.next.is_some() {
            let mut x = Node::new(key, self.maxHeight);
            swap(&mut x.next, &mut prev.next);
            prev.next = Some(x);
        } else {
            prev.next = Some(Node::new(key, self.maxHeight));
        }
    }
}

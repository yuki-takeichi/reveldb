use std::cell::RefCell;
use std::collections::LinkedList;

// Allocator

pub trait Allocator {
    fn allocate(&mut self) -> &mut Node;
    fn new() -> Self
    where
        Self: Sized;
}

pub struct NaiveAllocator {
    list: LinkedList<Node>,
}

impl Allocator for NaiveAllocator {
    fn allocate(&mut self) -> &mut Node {
        let range = Node::new();
        self.list.push_back(range);
        self.list.back_mut().expect("not reach here")
    }

    fn new() -> Self {
        NaiveAllocator {
            list: LinkedList::new(),
        }
    }
}

// Skip list

pub struct Node {
    key: i32,
    next: RefCell<[Box<Option<Node>>; 4]>,
}

impl Node {
    fn new() -> Self {
        Node {
            key: 0, // XXX default
            next: RefCell::new([
                Box::new(None),
                Box::new(None),
                Box::new(None),
                Box::new(None),
            ]),
        }
    }
}

pub trait SkipList {
    type Allocator: Allocator;

    fn new(allocator: Self::Allocator) -> Self
    where
        Self: Sized;
    fn add(&mut self, key: i8);
}

pub struct NaiveSkipList {
    allocator: NaiveAllocator,
    head: Box<Node>,
}

impl SkipList for NaiveSkipList {
    type Allocator = NaiveAllocator;

    fn new(allocator: NaiveAllocator) -> Self {
        let node = Node::new();
        NaiveSkipList {
            allocator: allocator,
            head: Box::new(node),
        }
    }

    fn add(&mut self, key: i8) {
        let mut next = self.head.next.borrow_mut();
        next[0] = Box::new(Some(Node::new()));
    }
}

// memtable

pub struct MemTable<S: SkipList> {
    skip_list: S,
}

impl<S> MemTable<S>
where
    S: SkipList,
{
    fn new() -> Self {
        let allocator = <S as SkipList>::Allocator::new();
        let skip_list = <S>::new(allocator);
        MemTable {
            skip_list: skip_list,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn allocator() {
        let mut allocator = NaiveAllocator::new();
        let mem = allocator.allocate();
        mem.key = 777;
    }

    #[test]
    fn memtable() {
        let mut memtable = MemTable::<NaiveSkipList>::new();
    }
}

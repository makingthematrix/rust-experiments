use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashSet;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut};

struct MyData {
    id: usize,
    number: usize,
    graph: Weak<Graph<MyData>>,
}

static mut FIRST_FREE_ID: AtomicUsize = AtomicUsize::new(1);

unsafe fn get_id() -> usize {
    FIRST_FREE_ID.fetch_add(1, Ordering::SeqCst)
}

impl MyData {
    pub fn new(number: usize, refs: Weak<Graph<MyData>>) -> MyData {
        let id = unsafe { get_id() };
        MyData { id, number, graph: refs }
    }
}

struct Graph<T>(RefCell<Vec<Weak<T>>>);

impl Graph<MyData> {
    pub fn new(capacity: usize) -> Self {
       Graph(RefCell::new(Vec::with_capacity(capacity)))
    }

    pub fn init(&self, arena: &Arena<MyData>) {
        let mut ref_mut = self.0.borrow_mut();
        arena.for_each(|d| { ref_mut.insert(d.number, Rc::downgrade(d)); });
    }
}

struct Arena<T> {
    vec: Vec<Rc<T>>,
    dim: usize
}

impl Arena<MyData> {
    pub fn new(dim: usize) -> Self {
        Arena { vec: Vec::with_capacity(dim * dim), dim }
    }

    pub fn init(&mut self, graph: &Rc<Graph<MyData>>) {
        let dim = self.dim;

        for i in 0..dim {
            for j in 0..dim {
                self.vec.push(Rc::new(MyData::new((i*dim + j), Rc::downgrade(graph))));
            }
        }
    }

    pub fn for_each<F>(&self, mut f: F) where
        Self: Sized, F: FnMut(&Rc<MyData>),
    {
        self.vec.iter().for_each(|d| f(d));
    }

}

struct World {
    arena: Arena<MyData>,
    graph: Rc<Graph<MyData>>
}

impl World {
    pub fn new(dim: usize) -> Self {
        let mut arena = Arena::<MyData>::new(dim);
        let graph = Rc::new(Graph::<MyData>::new(dim * dim));

        arena.init(&graph);
        graph.init(&arena);

        World { arena, graph }
    }
}

#[cfg(test)]
mod arena_tests {
    use std::collections::HashSet;

    use quickcheck::TestResult;
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn set_from_and_to_vec() {
        let world = World::new(100);

        assert_that!(true);
    }
}
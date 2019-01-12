use std::cell::{RefCell, Ref, RefMut};
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Node<T: Clone> {
    value: RefCell<T>,
    next: Option<Reference<T>>,
    prev: Option<WeakReference<T>>
}

type Reference<T> = Rc<RefCell<Node<T>>>;
type WeakReference<T> = Weak<RefCell<Node<T>>>;

impl <T: Clone> Node<T> {
    fn new(val: T) -> Node<T> {
        Node{value: RefCell::new(val), next: None, prev: None}
    }

    pub fn value(&self) -> Ref<T> {
        self.value.borrow()
    }

    pub fn value_mut(&self) -> RefMut<T> {
        self.value.borrow_mut()
    }

    pub fn next(&self) -> Option<Reference<T>> {

        let n = match self.next {
            Some(ref v) => Some(v),
            None => None
        };
        
        clone_opt_ref(n)
    }

    fn set_next(&mut self, node: Option<&Reference<T>>) {
        let nd = clone_opt_ref(node);
        self.next = nd;
    }

    fn set_prev(&mut self, node: Option<&Reference<T>>) {
        let wk = clone_opt_weak(node);
        self.prev = wk;
    }

    pub fn prev(&self) -> Option<Reference<T>> {

        let p = match self.prev {
            Some(ref v) => Some(v),
            None => None
        };

        clone_opt_ref_from_weak(p)
    }

}

fn clone_opt_ref<T: Clone>(node : Option<&Reference<T>>) -> Option<Reference<T>> {
        match node {
            Some(v) => Some(Rc::clone(v)),
            None => None
        }
}

fn clone_opt_ref_from_weak<T: Clone>(node: Option<&WeakReference<T>>) -> Option<Reference<T>> {
    if let Some(v) = node {
        if let Some(s) = v.upgrade() {
            return Some(s);
        }
    }

    None
}

fn clone_opt_weak<T: Clone>(node: Option<&Reference<T>>) -> Option<WeakReference<T>> {
    match node {
        Some(v) => Some(Rc::downgrade(v)),
        None => None
    }
}

pub struct LinkedList<T: Clone> {
    first: Option<Reference<T>>,
    last: Option<Reference<T>>
}

impl <T:Clone> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {first: None, last: None}
    }

    pub fn first(&self) -> Option<Reference<T>> {
        let node = match self.first {
            Some(ref n) => Some(n),
            None => None
        };

        clone_opt_ref(node)
    }

    pub fn last(&self) -> Option<Reference<T>> {
        let node = match self.last {
            Some(ref n) => Some(n),
            None => None
        };

        clone_opt_ref(node)
    }

    pub fn insert_first(&mut self, val: T) {
        let nd = Node::new(val);
        let nd_ref = Rc::new(RefCell::new(nd));
        let mut nd_mut = nd_ref.borrow_mut();
        //if there is already a first node, then we need to replace it
        if let Some(ref f) = self.first {
            nd_mut.set_next(Some(f));

            let mut f_mut = f.borrow_mut();
            f_mut.set_prev(Some(&nd_ref));
        }
        else { //otherwise we are both first and last
            self.last = Some(Rc::clone(&nd_ref));
        }

        self.first = Some(Rc::clone(&nd_ref));
    }

    pub fn insert_last(&mut self, val: T) {
        let nd = Node::new(val);
        let n = Rc::new(RefCell::new(nd));
        let mut mut_nd = n.borrow_mut();
       if let Some(ref l) = self.last {
           mut_nd.set_prev(Some(l));

           let mut l_mut = l.borrow_mut();
           l_mut.set_next(Some(&n));
       }
       else { //otherwise ware are both first and last
            self.first = Some(Rc::clone(&n));
       }

       self.last = Some(Rc::clone(&n));
    }

    pub fn insert_after(&mut self, val: T, after: &Reference<T>) {
        let nd = Node::new(val);
        let rf = Rc::new(RefCell::new(nd));
        let mut mut_rf = rf.borrow_mut();

        let mut aft = after.borrow_mut();
        //if the thing we are going after was last
        //we are last. Otherwise steal its next
        match aft.next() {
            Some(n) => {
                let mut nm = n.borrow_mut();
                nm.set_prev(Some(&rf));
                mut_rf.set_next(Some(&n));
            },
            None => self.last = Some(Rc::clone(&rf))
        };

        mut_rf.set_prev(Some(&after));
        aft.set_next(Some(&rf));
    }

    pub fn insert_before(&mut self, val: T, before: &Reference<T>) {
        let nd = Node::new(val);
        let rf = Rc::new(RefCell::new(nd));
        let mut mut_rf = rf.borrow_mut();

        let mut bef = before.borrow_mut();

        match bef.prev() {
            Some(p) => {
                let mut pm = p.borrow_mut();
                pm.set_next(Some(&rf));
                mut_rf.set_prev(Some(&p));
            },
            None => self.first = Some(Rc::clone(&rf))
        };

        mut_rf.set_next(Some(before));
        bef.set_prev(Some(&rf));
    }

    pub fn remove(&mut self, to_remove: &Reference<T>) {
        let rm = to_remove.borrow_mut();
        let pr = rm.prev();
        let nx = rm.next();

        match pr {
            Some(p) => {
                match nx {
                    Some(n) => {
                        let mut pm = p.borrow_mut();
                        let mut nm = n.borrow_mut();
                        pm.set_next(Some(&n));
                        nm.set_prev(Some(&p));
                    },
                    None => self.last = Some(Rc::clone(&p))
                }
            },
            None => {
                match nx {
                    Some(n) => {
                        let mut nm = n.borrow_mut();
                        nm.set_prev(None);
                        self.first = Some(Rc::clone(&n))
                    },
                    None => {
                        self.first = None;
                        self.last = None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_last() {
        let mut lst = LinkedList::new();

        lst.insert_last(1);
        lst.insert_last(2);

        if let Some(f) = lst.first() {
            let fb = f.borrow();
            assert_eq!(*fb.value(), 1);

            if let Some(n) = fb.next() {
                assert!(Rc::ptr_eq(&n, &lst.last().unwrap()));
                let nx = n.borrow();
                assert!(Rc::ptr_eq(&nx.prev().unwrap(), &f));
                assert_eq!(*nx.value(), 2);
            }
            else {
                panic!("next is missing");
            }
        }
        else {
            panic!("first is missing");
        }
    }

    #[test]
    fn test_insert_first() {
        let mut lst = LinkedList::new();

        lst.insert_first(1);
        lst.insert_first(2);

        if let Some(f) = lst.first() {
            let fb = f.borrow();
            assert_eq!(*fb.value(), 2);

            if let Some(n) = fb.next() {
                assert!(Rc::ptr_eq(&n, &lst.last().unwrap()));
                let nx = n.borrow();
                assert!(Rc::ptr_eq(&nx.prev().unwrap(), &f));
                assert_eq!(*nx.value(), 1);
            }
            else {
                panic!("next is missing");
            }
        }
        else {
            panic!("first is missing");
        }
    }

    #[test]
    fn test_insert_after_middle() {
        let mut lst = LinkedList::new();

        lst.insert_last(1);
        let fst = lst.first().unwrap();
        lst.insert_after(2, &fst);
        lst.insert_after(3, &fst);

        if let Some(f) = lst.first() {
            let fb = f.borrow();
            assert_eq!(*fb.value(), 1);

            if let Some(n) = fb.next() {
                
                let nx = n.borrow();
                assert!(Rc::ptr_eq(&nx.prev().unwrap(), &f));
                assert_eq!(*nx.value(), 3);

                if let Some(l) = nx.next() {
                    let ls = l.borrow();
                    assert_eq!(*ls.value(), 2);
                    assert!(Rc::ptr_eq(&ls.prev().unwrap(), &n));
                    assert!(Rc::ptr_eq(&l, &lst.last().unwrap()));
                }
                else {
                    panic!("last is missing");
                }
            }
            else {
                panic!("next is missing");
            }
        }
        else {
            panic!("first is missing");
        }
    }

    #[test]
    fn test_insert_after_last() {
        let mut lst = LinkedList::new();

        lst.insert_last(1);
        let fst = lst.first().unwrap();
        lst.insert_after(2, &fst);
        let cur = lst.last().unwrap();
        lst.insert_after(3, &cur);

        if let Some(f) = lst.first() {
            let fb = f.borrow();
            assert_eq!(*fb.value(), 1);

            if let Some(n) = fb.next() {
                
                let nx = n.borrow();
                assert!(Rc::ptr_eq(&nx.prev().unwrap(), &f));
                assert_eq!(*nx.value(), 2);

                if let Some(l) = nx.next() {
                    let ls = l.borrow();
                    assert_eq!(*ls.value(), 3);
                    assert!(Rc::ptr_eq(&ls.prev().unwrap(), &n));
                    assert!(Rc::ptr_eq(&l, &lst.last().unwrap()));
                }
                else {
                    panic!("last is missing");
                }
            }
            else {
                panic!("next is missing");
            }
        }
        else {
            panic!("first is missing");
        }
    }
}
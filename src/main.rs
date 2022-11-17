use std::{rc::Rc, cell::RefCell, ops::Deref};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    pub value: i32,
    pub parent: NodeOption,
    pub next: NodeOption,
}
pub type NodeOption = Option<Rc<RefCell<Node>>>;


impl Node {
    fn new(value: i32, parent: NodeOption, next: NodeOption) -> Node {
        Node {
            value,
            parent,
            next,
        }
    }

    fn print(&self) {
        println!("Value: {}", self.value);
    }

    fn print_all(&self) {
        self.print();
        if let Some(ref next) = self.next {
            let n = next.deref();
            //let n = &**next;
            let next: &Node = &n.borrow();
            next.print_all();
        }
    }

    fn print_all_reverse(&self) {
        self.print();
        if let Some(ref parent) = self.parent {
            let p = parent.deref();
            //let p = &**parent;
            let parent: &Node = &p.borrow();
            parent.print_all_reverse();
        }
    }


}

fn main() {

    let root = Node::new(1, None, None);
    let root_rc_refcell = Rc::new(RefCell::new(root));

    let child = Node::new(2, Some(Rc::clone(&root_rc_refcell)), None);
    let child_rc_refcell = Rc::new(RefCell::new(child));

    let child_2 = Node::new(3, Some(Rc::clone(&child_rc_refcell)), None);
    let child_2_rc_refcell = Rc::new(RefCell::new(child_2));

    let child_3 = Node::new(4, Some(Rc::clone(&child_2_rc_refcell)), None);
    let child_3_rc_refcell = Rc::new(RefCell::new(child_3));

    
    {
        let mut r = root_rc_refcell.borrow_mut();
        let mut c = child_rc_refcell.borrow_mut();
        let mut c2 = child_2_rc_refcell.borrow_mut();

        r.next = Some(Rc::clone(&child_rc_refcell));
        c.next = Some(Rc::clone(&child_2_rc_refcell));
        c2.next = Some(Rc::clone(&child_3_rc_refcell));
    }

    
    let r: &Node = &root_rc_refcell.deref().borrow();
    r.print_all();

    println!("------------------");

    let c3: &Node = &child_3_rc_refcell.deref().borrow();
    c3.print_all_reverse();
}

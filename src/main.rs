use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct State {
    pub value: i32,
    pub parent: OptionRcRefCellState,
    pub children: OptionChildren,
}

pub type RcRefCellState = Rc<RefCell<State>>;
pub type OptionRcRefCellState = Option<RcRefCellState>;
pub type Children = HashMap<String, RcRefCellState>;
pub type OptionChildren = Option<Children>;

impl State {
    fn new(value: i32, parent: OptionRcRefCellState, children: OptionChildren) -> State {
        State {
            value,
            parent,
            children,
        }
    }

    fn add_child(&mut self, name: String, child: RcRefCellState) {
        //the child must have this as a parent
        if let Some(children) = &mut self.children {
            children.insert(name, child);
        } else {
            let mut children = HashMap::new();
            children.insert(name, child);
            self.children = Some(children);
        }
    }

    fn get_child(&self, name: &str) -> OptionRcRefCellState {
        if let Some(children) = &self.children {
            if let Some(child) = children.get(name) {
                return Some(Rc::clone(&child));
            }
        }
        None
    }

    fn get_parent(&self) -> OptionRcRefCellState {
        if let Some(parent) = &self.parent {
            return Some(Rc::clone(&parent));
        }
        None
    }

    fn to_string(&self, offset: u32) -> String {
        //if self is a child of its own children, then we have a loop :D => StackOverflow!
        let mut s = String::new();
        for _ in 0..offset {
            s.push_str("\t");
        }
        let mut result = format!("Value: {}", self.value);
        if let Some(children) = &self.children {
            result = format!("{} | Children:", result);

            for (name, child) in children {
                let child: &State = &*child.borrow();
                result = format!(
                    "{}\n{s}\tName: {} | {}",
                    result,
                    name,
                    child.to_string(offset + 1)
                );
            }
        }
        result
    }
}

fn main() {
    let root = Rc::new(RefCell::new(State::new(0, None, None)));

    {
        let child1 = Rc::new(RefCell::new(State::new(1, Some(root.clone()), None)));
        let child2 = Rc::new(RefCell::new(State::new(2, Some(root.clone()), None)));
        let child3 = Rc::new(RefCell::new(State::new(3, Some(root.clone()), None)));
        let child4 = Rc::new(RefCell::new(State::new(4, Some(root.clone()), None)));

        //add children to child4
        let mut child4_mut = child4.borrow_mut();
        let child41 = Rc::new(RefCell::new(State::new(41, Some(child4.clone()), None)));
        let child42 = Rc::new(RefCell::new(State::new(42, Some(child4.clone()), None)));
        let child43 = Rc::new(RefCell::new(State::new(43, Some(child4.clone()), None)));
        let child44 = Rc::new(RefCell::new(State::new(44, Some(child4.clone()), None)));

        child4_mut.add_child("child41".to_string(), child41);
        child4_mut.add_child("child42".to_string(), child42);
        child4_mut.add_child("child43".to_string(), child43);
        child4_mut.add_child("child44".to_string(), child44);

        //add children to child3
        let mut child3_mut = child3.borrow_mut();
        let child31 = Rc::new(RefCell::new(State::new(31, Some(child3.clone()), None)));
        let child32 = Rc::new(RefCell::new(State::new(32, Some(child3.clone()), None)));
        let child33 = Rc::new(RefCell::new(State::new(33, Some(child3.clone()), None)));

        child3_mut.add_child("child31".to_string(), child31.clone());
        child3_mut.add_child("child32".to_string(), child32);
        child3_mut.add_child("child33".to_string(), child33);

        //add children to child31
        let mut child31_mut = child31.borrow_mut();
        let child311 = Rc::new(RefCell::new(State::new(311, Some(child31.clone()), None)));
        let child312 = Rc::new(RefCell::new(State::new(312, Some(child31.clone()), None)));
        let child313 = Rc::new(RefCell::new(State::new(313, Some(child31.clone()), None)));

        child31_mut.add_child("child311".to_string(), child311.clone());
        child31_mut.add_child("child312".to_string(), child312);
        child31_mut.add_child("child313".to_string(), child313);

        //add children to child311
        let mut child311_mut = child311.borrow_mut();
        let child3111 = Rc::new(RefCell::new(State::new(3111, Some(child311.clone()), None)));
        let child3112 = Rc::new(RefCell::new(State::new(3112, Some(child311.clone()), None)));
        let child3113 = Rc::new(RefCell::new(State::new(3113, Some(child311.clone()), None)));

        child311_mut.add_child("child3111".to_string(), child3111);
        child311_mut.add_child("child3112".to_string(), child3112);
        child311_mut.add_child("child3113".to_string(), child3113);

        //add children to root
        let mut root_mut = root.borrow_mut();
        root_mut.add_child("child1".to_string(), child1.clone());
        root_mut.add_child("child2".to_string(), child2.clone());
        root_mut.add_child("child3".to_string(), child3.clone());
        root_mut.add_child("child4".to_string(), child4.clone());
    }

    let root: &State = &*root.borrow();

    println!("{}", root.to_string(0));

    println!("--------------");
    // println!(
    //     "Child 1: {}",
    //     root.get_child("child1").unwrap().borrow().to_string(0)
    // );
}

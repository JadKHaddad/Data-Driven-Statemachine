use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Status {
    state_changed: bool,
    state: OptionRcRefCellDynStateLike,
    output: String,
    submit: bool,
    input_recognized: bool,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name = String::new();
        if self.state_changed {
            if let Some(state) = &self.state {
                name = state.borrow().get_name();
            }
        }
        write!(
            f,
            "state_changed: {}\nsubmit: {}\noutput: {}\nstate name: {}",
            self.state_changed, self.submit, self.output, name
        )
    }
}

type RcRefCellDynStateLike = Rc<RefCell<dyn StateLike>>;
type OptionRcRefCellDynStateLike = Option<RcRefCellDynStateLike>;

trait StateLike {
    fn get_name(&self) -> String;
    fn input(&mut self, input: String) -> Status;
    fn back(&mut self) -> Status;
    fn to_string(&self, offset: u32) -> String;
    fn get_parent(&self) -> OptionRcRefCellDynStateLike;
}

struct Context {
    name: String,
    value: String,
}

pub struct ContextState {
    name: String,
    value: i32,
    index: u32,
    parent: OptionRcRefCellDynStateLike,
    next: OptionRcRefCellDynStateLike,
    contexts: Vec<Context>,
}

impl ContextState {
    fn new(
        name: String,
        value: i32,
        parent: OptionRcRefCellDynStateLike,
        next: OptionRcRefCellDynStateLike,
        contexts: Vec<Context>,
    ) -> ContextState {
        ContextState {
            name,
            value,
            index: 0,
            parent,
            next,
            contexts,
        }
    }
}

impl StateLike for ContextState {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn input(&mut self, input: String) -> Status {
        let mut status = Status {
            state_changed: false,
            state: None,
            output: String::new(),
            submit: false,
            input_recognized: true,
        };

        if self.index < self.contexts.len() as u32 {
            self.index += 1;
        }

        if let Some(mut context) = self.contexts.get_mut(self.index as usize) {
            context.value = input;
        }

        if self.index == self.contexts.len() as u32 {
            status.state_changed = true;
            status.state = self.next.clone();
        }

        return status;
    }

    fn back(&mut self) -> Status {
        let mut status = Status {
            state_changed: false,
            state: None,
            output: String::new(),
            submit: false,
            input_recognized: true,
        };

        if self.index > 0 {
            self.index -= 1;
        }

        if self.index == 0 {
            if self.parent.is_some() {
                status.state_changed = true;
                status.state = self.parent.clone();
            }
        }

        return status;
    }

    fn to_string(&self, offset: u32) -> String {
        String::from("ContextState")
    }

    fn get_parent(&self) -> OptionRcRefCellDynStateLike {
        if let Some(parent) = &self.parent {
            return Some(Rc::clone(&parent));
        }
        None
    }
}

trait OptionLike {
    fn input(&self, input: &String) -> bool;
    fn get_state(&mut self) -> OptionRcRefCellDynStateLike;
    fn get_submit(&self) -> bool;
}
pub struct StateOption {
    name: String,
    state: OptionRcRefCellDynStateLike,
    submit: bool,
}

impl StateOption {
    fn new(name: String, state: OptionRcRefCellDynStateLike, submit: bool) -> StateOption {
        StateOption {
            name,
            state,
            submit,
        }
    }
}

impl OptionLike for StateOption {
    fn input(&self, input: &String) -> bool {
        if &self.name == input {
            return true;
        }
        false
    }

    fn get_state(&mut self) -> OptionRcRefCellDynStateLike {
        self.state.clone()
    }

    fn get_submit(&self) -> bool {
        self.submit
    }
}

pub struct StateLambdaOption {
    name: String,
    lambda_state: fn() -> OptionRcRefCellDynStateLike, //parent should be passed in
    submit: bool,
    state_created: bool,
    state: OptionRcRefCellDynStateLike,
}

impl StateLambdaOption {
    fn new(
        name: String,
        lambda_state: fn() -> OptionRcRefCellDynStateLike,
        submit: bool,
    ) -> StateLambdaOption {
        StateLambdaOption {
            name,
            lambda_state,
            submit,
            state_created: false,
            state: None,
        }
    }
}

impl OptionLike for StateLambdaOption {
    fn input(&self, input: &String) -> bool {
        if &self.name == input {
            return true;
        }
        false
    }

    fn get_state(&mut self) -> OptionRcRefCellDynStateLike {
        if !self.state_created {
            self.state = (self.lambda_state)();
            self.state_created = true;
        }
       
        self.state.clone()
    }

    fn get_submit(&self) -> bool {
        self.submit
    }
}

type OptionVecBoxDynOptionLike = Option<Vec<Box<dyn OptionLike>>>;

pub struct OptionsState {
    name: String,
    value: i32,
    parent: OptionRcRefCellDynStateLike,
    options: OptionVecBoxDynOptionLike,
}

impl OptionsState {
    fn new(
        name: String,
        value: i32,
        parent: OptionRcRefCellDynStateLike,
        options: OptionVecBoxDynOptionLike,
    ) -> OptionsState {
        OptionsState {
            name,
            value,
            parent,
            options,
        }
    }

    fn add_option(&mut self, option: Box<dyn OptionLike>) {
        //every option state should have this as a parent
        if let Some(options) = &mut self.options {
            options.push(option);
        } else {
            self.options = Some(vec![option]);
        }
    }

    // fn add_child(&mut self, name: String, child: RcRefCellDynState) {
    //     //the child must have this as a parent
    //     if let Some(children) = &mut self.options {
    //         children.insert(name, child);
    //     } else {
    //         let mut children = HashMap::new();
    //         children.insert(name, child);
    //         self.options = Some(children);
    //     }
    // }

    // fn get_child(&self, name: &str) -> OptionRcRefCellDynState {
    //     if let Some(children) = &self.options {
    //         if let Some(child) = children.get(name) {
    //             return Some(Rc::clone(&child));
    //         }
    //     }
    //     None
    // }
}

impl StateLike for OptionsState {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn input(&mut self, input: String) -> Status {
        let mut status = Status {
            state_changed: false,
            state: None,
            output: String::new(),
            submit: false,
            input_recognized: false,
        };

        if let Some(options) = self.options.as_mut() {
            for option in options.iter_mut() {
                if option.input(&input) {
                    status.state_changed = true;
                    status.state = option.get_state();
                    status.submit = option.get_submit();
                    status.input_recognized = true;
                    break;
                }
            }
        }

        return status;
    }

    fn back(&mut self) -> Status {
        let mut status = Status {
            state_changed: false,
            state: None,
            output: String::new(),
            submit: false,
            input_recognized: true,
        };
        if self.parent.is_some() {
            status.state_changed = true;
            status.state = self.parent.clone();
        }
        return status;
    }

    fn to_string(&self, offset: u32) -> String {
        //if self is a child of its own children, then we have a loop :D => StackOverflow!
        // let mut s = String::new();
        // for _ in 0..offset {
        //     s.push_str("\t");
        // }
        // let mut result = format!("Value: {}", self.value);
        // if let Some(children) = &self.options {
        //     result = format!("{} | Children:", result);

        //     for (name, child) in children {
        //         let child = child.borrow();
        //         result = format!(
        //             "{}\n{s}\tName: {} | {}",
        //             result,
        //             name,
        //             child.to_string(offset + 1)
        //         );
        //     }
        // }
        // result
        String::from("OptionsState")
    }

    fn get_parent(&self) -> OptionRcRefCellDynStateLike {
        if let Some(parent) = &self.parent {
            return Some(Rc::clone(&parent));
        }
        None
    }
}
fn main() {
    let states: Vec<Box<dyn StateLike>> = vec![];

    let root = Rc::new(RefCell::new(OptionsState::new(
        String::from("root"),
        0,
        None,
        None,
    )));

    {
        //create children
        let child1 = Rc::new(RefCell::new(OptionsState::new(
            String::from("child1"),
            1,
            Some(root.clone()),
            None,
        )));
        let child2 = Rc::new(RefCell::new(OptionsState::new(
            String::from("child2"),
            2,
            Some(root.clone()),
            None,
        )));
        let child3 = Rc::new(RefCell::new(OptionsState::new(
            String::from("child3"),
            3,
            Some(root.clone()),
            None,
        )));

        //create a context state
        let context_state = Rc::new(RefCell::new(ContextState::new(
            String::from("context_state"),
            4,
            Some(root.clone()),
            Some(child1.clone()),
            vec![
                Context {
                    name: String::from("context1"),
                    value: String::new(),
                },
                Context {
                    name: String::from("context2"),
                    value: String::new(),
                },
                Context {
                    name: String::from("context3"),
                    value: String::new(),
                },
            ],
        )));

        //create options
        let option1 = StateOption::new(String::from("option1"), Some(child1.clone()), false);
        let option2 = StateOption::new(String::from("option2"), Some(child2.clone()), false);
        let option3 = StateOption::new(String::from("option3"), Some(child3.clone()), false);
        let option4 = StateOption::new(String::from("option4"), Some(context_state.clone()), false);

        //create options vector
        let options: Vec<Box<dyn OptionLike>> = vec![
            Box::new(option1),
            Box::new(option2),
            Box::new(option3),
            Box::new(option4),
        ];

        //add options to root
        root.borrow_mut().options = Some(options);
    }

    let status: Status;
    {
        let mut root = root.borrow_mut();
        status = root.input(String::from("option4"));
    }

    println!("{}", status);

    // {
    //     let child1 = Rc::new(RefCell::new(State::new(1, Some(root.clone()), None)));
    //     let child2 = Rc::new(RefCell::new(State::new(2, Some(root.clone()), None)));
    //     let child3 = Rc::new(RefCell::new(State::new(3, Some(root.clone()), None)));
    //     let child4 = Rc::new(RefCell::new(State::new(4, Some(root.clone()), None)));

    //     //add children to child4
    //     let mut child4_mut = child4.borrow_mut();
    //     let child41 = Rc::new(RefCell::new(State::new(41, Some(child4.clone()), None)));
    //     let child42 = Rc::new(RefCell::new(State::new(42, Some(child4.clone()), None)));
    //     let child43 = Rc::new(RefCell::new(State::new(43, Some(child4.clone()), None)));
    //     let child44 = Rc::new(RefCell::new(State::new(44, Some(child4.clone()), None)));

    //     child4_mut.add_child("child41".to_string(), child41);
    //     child4_mut.add_child("child42".to_string(), child42);
    //     child4_mut.add_child("child43".to_string(), child43);
    //     child4_mut.add_child("child44".to_string(), child44);

    //     //add children to child3
    //     let mut child3_mut = child3.borrow_mut();
    //     let child31 = Rc::new(RefCell::new(State::new(31, Some(child3.clone()), None)));
    //     let child32 = Rc::new(RefCell::new(State::new(32, Some(child3.clone()), None)));
    //     let child33 = Rc::new(RefCell::new(State::new(33, Some(child3.clone()), None)));

    //     child3_mut.add_child("child31".to_string(), child31.clone());
    //     child3_mut.add_child("child32".to_string(), child32);
    //     child3_mut.add_child("child33".to_string(), child33);

    //     //add children to child31
    //     let mut child31_mut = child31.borrow_mut();
    //     let child311 = Rc::new(RefCell::new(State::new(311, Some(child31.clone()), None)));
    //     let child312 = Rc::new(RefCell::new(State::new(312, Some(child31.clone()), None)));
    //     let child313 = Rc::new(RefCell::new(State::new(313, Some(child31.clone()), None)));

    //     child31_mut.add_child("child311".to_string(), child311.clone());
    //     child31_mut.add_child("child312".to_string(), child312);
    //     child31_mut.add_child("child313".to_string(), child313);

    //     //add children to child311
    //     let mut child311_mut = child311.borrow_mut();
    //     let child3111 = Rc::new(RefCell::new(State::new(3111, Some(child311.clone()), None)));
    //     let child3112 = Rc::new(RefCell::new(State::new(3112, Some(child311.clone()), None)));
    //     let child3113 = Rc::new(RefCell::new(State::new(3113, Some(child311.clone()), None)));

    //     child311_mut.add_child("child3111".to_string(), child3111);
    //     child311_mut.add_child("child3112".to_string(), child3112);
    //     child311_mut.add_child("child3113".to_string(), child3113);

    //     //add children to root
    //     let mut root_mut = root.borrow_mut();
    //     root_mut.add_child("child1".to_string(), child1.clone());
    //     root_mut.add_child("child2".to_string(), child2.clone());
    //     root_mut.add_child("child3".to_string(), child3.clone());
    //     root_mut.add_child("child4".to_string(), child4.clone());
    // }

    // let root = root.borrow();

    // println!("{}", root.to_string(0));

    // println!("--------------");
    // println!(
    //     "Child 1: {}",
    //     root.get_child("child1").unwrap().borrow().to_string(0)
    // );
}

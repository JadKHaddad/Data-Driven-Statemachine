use statemachine::{
    context_like::StateContext,
    option_like::{OptionLike, StateClosureOption, StateOption},
    state_like::{ContextState, OptionsState, StateLike},
    status::Status,
};
use std::{cell::RefCell, rc::Rc};

fn main() {
    let root = Rc::new(RefCell::new(OptionsState::new(
        String::from("root"),
        None,
        None,
    )));

    {
        //create children
        let child1 = Rc::new(RefCell::new(OptionsState::new(
            String::from("child1"),
            Some(root.clone()),
            None,
        )));
        let child2 = Rc::new(RefCell::new(OptionsState::new(
            String::from("child2"),
            Some(root.clone()),
            None,
        )));
        let child3 = Rc::new(RefCell::new(OptionsState::new(
            String::from("child3"),
            Some(root.clone()),
            None,
        )));

        //create a context state
        let context_state = Rc::new(RefCell::new(ContextState::new(
            String::from("context_state"),
            Some(root.clone()),
            None,
            vec![
                StateContext {
                    name: String::from("context1"),
                    value: String::new(),
                },
                StateContext {
                    name: String::from("context2"),
                    value: String::new(),
                },
                StateContext {
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

        //create a clousure option
        let root_clone = root.clone();
        let option5 = StateClosureOption::new(
            String::from("option5"),
            move || {
                println!("Creating option5");
                Some(Rc::new(RefCell::new(OptionsState::new(
                    String::from("child5"),
                    Some(root_clone.clone()),
                    None,
                ))))
            },
            false,
        );

        //create options vector
        let options: Vec<Box<dyn OptionLike>> = vec![
            Box::new(option1),
            Box::new(option2),
            Box::new(option3),
            Box::new(option4),
            Box::new(option5),
        ];

        //add options to root
        root.borrow_mut().options = Some(options);
    }

    let mut current_state: Rc<RefCell<dyn StateLike>> = root.clone();
    loop {
        let status: Status;
        {
            let mut current_state_ref = current_state.borrow_mut();
            println!("{}", current_state_ref.output());

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("error: unable to read user input");
            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }
            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }
            if input == "back" {
                status = current_state_ref.back();
            } else {
                status = current_state_ref.input(input);
            }
            println!("------------");
            println!("{}", status);
            println!("------------");
        }

        if status.state_changed {
            if let Some(state) = status.state {
                current_state = state;
            }
            if status.submit {
                println!("submitting\n");
                let collections = current_state.borrow().collect_contexts();
                for collection in collections {
                    println!("{}:", collection.name);
                    for context in collection.contexts {
                        println!("{}: {}", context.name, context.value);
                    }
                }
                break;
            }
        }
    }

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

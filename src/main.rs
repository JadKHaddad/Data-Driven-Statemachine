use statemachine::{
    context_like::StateContext,
    option_like::{OptionLike, StateOption},
    serde_state_like::*,
    state_like::{ContextState, OptionsState, StateHolder, StateLike},
    status::Status,
};
use std::{cell::RefCell, rc::Rc};

fn t() {
    let root = Rc::new(RefCell::new(OptionsState::new(
        String::from("root"),
        String::from("root description"),
        None,
        vec![],
    )));

    {
        //create children
        let child1 = Rc::new(RefCell::new(OptionsState::new(
            String::from("child1"),
            String::from("child1 description"),
            Some(root.clone()),
            vec![],
        )));
        let child2 = Rc::new(RefCell::new(OptionsState::new(
            String::from("child2"),
            String::from("child2 description"),
            Some(root.clone()),
            vec![],
        )));
        let child3 = Rc::new(RefCell::new(OptionsState::new(
            String::from("child3"),
            String::from("child3 description"),
            Some(root.clone()),
            vec![],
        )));

        //create a context state
        let context_state = Rc::new(RefCell::new(ContextState::new(
            String::from("context_state"),
            String::from("context_state description"),
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
            false,
        )));

        //create options
        let option1 = StateOption::new(String::from("option1"), Box::new(child1.clone()), false);
        let option2 = StateOption::new(String::from("option2"), Box::new(child2.clone()), false);
        let option3 = StateOption::new(String::from("option3"), Box::new(child3.clone()), false);
        let option4 = StateOption::new(
            String::from("option4"),
            Box::new(context_state.clone()),
            false,
        );

        //create a clousure option
        let root_clone = root.clone();
        let option5 = StateOption::new(
            String::from("option5"),
            Box::new(StateHolder::new(
                move || {
                    println!("Creating option5");
                    Rc::new(RefCell::new(OptionsState::new(
                        String::from("child5"),
                        String::from("child5 description"),
                        Some(root_clone.clone()),
                        vec![],
                    )))
                },
                false,
            )),
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
        root.borrow_mut().options = options;
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
}

fn main() {
    t();
    //create a state creator
    let ser_into_state = SerDeIntoStateLike::Path("/opt/t".to_string(), true);

    let opt_path = SerDeOption {
        name: "option1".to_string(),
        submit: false,
        state: ser_into_state,
    };

    let opt_inline = SerDeOption {
        name: "option2".to_string(),
        submit: false,
        state: SerDeIntoStateLike::Inline(SerDeState {
            name: "child2".to_string(),
            description: "child2 description".to_string(),
            r#type: StateType::Context(
                vec![
                    SerDeContext {
                        name: "context1".to_string(),
                    },
                    SerDeContext {
                        name: "context2".to_string(),
                    },
                    SerDeContext {
                        name: "context3".to_string(),
                    },
                ],
                None,
                false,
            ),
        }),
    };

    let state_type = StateType::Options(vec![opt_path, opt_inline]);

    let state_c = SerDeState {
        name: "root".to_string(),
        description: "root description".to_string(),
        r#type: state_type,
    };

    // let opt_t = SerDeIntoStateLike::Inline(state_c);
    // let opt_c = SerDeOption{
    //     name: "option1".to_string(),
    //     submit: false,
    //     state: opt_t,
    // };
    // let state_t = StateType::Options(vec![opt_c]);
    // let state_c = SerDeState {
    //     name: "root".to_string(),
    //     description: "root description".to_string(),
    //     r#type: state_t,
    // };

    //state_c to yaml
    let yaml = serde_yaml::to_string(&state_c).unwrap();
    //save yaml to file
    std::fs::write("state.yaml", yaml).unwrap();
}

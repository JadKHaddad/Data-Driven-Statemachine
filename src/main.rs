use statemachine::{
    context_like::StateContext,
    option_like::{OptionLike, StateClosureOption, StateOption},
    state_like::{ContextState, OptionsState, StateLike},
    status::Status,
    state_creator::*,
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
                    String::from("child5 description"),
                    Some(root_clone.clone()),
                    vec![],
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
    //create a state creator
    let opt_t = OptionType::Closure("/opt/t".to_string());

    let opt_c = OptionCreator  {
        name: "option1".to_string(),
        submit: false,
        r#type: opt_t,
    };

    let state_t = StateType::Options(vec![opt_c]);

    let state_c = StateCreator {
        name: "root".to_string(),
        description: "root description".to_string(),
        r#type: state_t,
    };


    let opt_t = OptionType::State(state_c);
    let opt_c = OptionCreator  {
        name: "option1".to_string(),
        submit: false,
        r#type: opt_t,
    };
    let state_t = StateType::Options(vec![opt_c]);
    let state_c = StateCreator {
        name: "root".to_string(),
        description: "root description".to_string(),
        r#type: state_t
    };


    //state_c to yaml
    let yaml = serde_yaml::to_string(&state_c).unwrap();
    //save yaml to file
    std::fs::write("state.yaml", yaml).unwrap();
}


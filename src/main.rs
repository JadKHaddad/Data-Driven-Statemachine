use statemachine::{
    context_like::{StateContext, StateOptionsContext},
    option_like::{OptionLike, StateOption},
    serde_state_like::*,
    state_like::{ContextState, OptionsState, StateHolder, StateLike},
    status::{InputStatus, OutputStatus},
};
use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

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

        //create valid options state
        let state_for_valid_options = Rc::new(RefCell::new(OptionsState::new(
            String::from("valid options"),
            String::from("valid options"),
            None,
            vec![],
        )));

        //create a context state
        let context_state = Rc::new(RefCell::new(ContextState::new(
            String::from("context_state"),
            String::from("context_state description"),
            Some(root.clone()),
            None,
            vec![
                Box::new(StateContext {
                    name: String::from("normal"),
                    value: String::new(),
                }),
                Box::new(StateOptionsContext {
                    name: String::from("options"),
                    value: String::new(),
                    state: Box::new(state_for_valid_options.clone()),
                }),
            ],
            false,
        )));

        state_for_valid_options.borrow_mut().parent = Some(context_state.clone());

        //create the valid options
        let option1 = StateOption::new(
            String::from("Telekom"),
            Box::new(context_state.clone()),
            false,
        );
        let option2 = StateOption::new(
            String::from("Vodafone"),
            Box::new(context_state.clone()),
            false,
        );

        //create an context state with only one context
        let state_for_context = Rc::new(RefCell::new(ContextState::new(
            String::from("others"),
            String::from("others"),
            Some(state_for_valid_options.clone()),
            Some(Box::new(context_state.clone())),
            vec![Box::new(StateContext {
                name: String::from("so tell me what you want"),
                value: String::new(),
            })],
            false,
        )));

        let option3 = StateOption::new(
            String::from("others"),
            Box::new(state_for_context.clone()),
            false,
        );

        let options: Vec<Box<dyn OptionLike>> =
            vec![Box::new(option1), Box::new(option2), Box::new(option3)];
        state_for_valid_options.borrow_mut().options = options;

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
}

fn run(root: Rc<RefCell<dyn StateLike>>) {
    let mut current_state: Rc<RefCell<dyn StateLike>> = root.clone();
    loop {
        let output_status: OutputStatus;
        let input_status: InputStatus;
        {
            {
                let mut current_state_ref = current_state.borrow_mut();
                output_status = current_state_ref.output();
                println!("{}", output_status);
                println!("------------");
            }

            if output_status.state_changed {
                if let Some(state) = output_status.state {
                    current_state = state;
                    continue;
                }
                if output_status.submit {
                    println!("submitting\n");
                    let collections = current_state.borrow_mut().collect().unwrap();
                    for collection in collections {
                        println!("{}:", collection.state_name);
                        for context in collection.context_collections {
                            println!("{}: {}", context.name, context.value);
                        }
                    }
                    break;
                }
            }

            println!("{}", output_status.output);

            let mut current_state_ref = current_state.borrow_mut();

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
                input_status = current_state_ref.back();
            } else {
                input_status = current_state_ref.input(input);
            }
            println!("------------");
            println!("{}", input_status);
            println!("------------");
        }

        if input_status.state_changed {
            if let Some(state) = input_status.state {
                current_state = state;
            }
            if input_status.submit {
                println!("submitting\n");
                let collections = current_state.borrow_mut().collect().unwrap();
                println!("{:?}", collections);
                for collection in collections {
                    println!("{}:", collection.state_name);
                    for context in collection.context_collections {
                        println!("{}: {}", context.name, context.value);
                    }
                }
                break;
            }
        }
    }
}
fn main() {
    let how_to_get_string = |name: String| {
        let mut file = File::open(&name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Ok(contents)
    };

    let state = SerDeState::create(Box::new(how_to_get_string), String::from("state.yaml")).unwrap().unwrap();
    run(state);

}

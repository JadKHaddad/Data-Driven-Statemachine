use parking_lot::RwLock;
use statemachine::{
    serde_state_like::*,
    state_like::StateLike,
    status::{InputStatus, OutputStatus},
};

use std::{fs::File, io::Read, sync::Arc};

fn run(root: Arc<RwLock<dyn StateLike>>) {
    let mut current_state: Arc<RwLock<dyn StateLike>> = root.clone();
    loop {
        let output_status: OutputStatus;
        let input_status: InputStatus;
        {
            {
                let mut current_state_ref = current_state.write();
                output_status = current_state_ref.output().unwrap();
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
                    let collections = current_state.write().collect().unwrap().unwrap();
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

            let mut current_state_ref = current_state.write();

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
                input_status = current_state_ref.input(input).unwrap();
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
                let collections = current_state.write().collect().unwrap().unwrap();
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
use std::error::Error as StdError;
fn main() {
    let how_to_get_string_local = |name: String| {
        let mut file = File::open(&name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Ok(contents)
    };

    let how_to_get_string_from_api = |name: String| {
        //sleep 3 seconds to simulate api call
        std::thread::sleep(std::time::Duration::from_secs(3));

        let mut file = File::open(&name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Ok(contents)
    };

    let functions: Vec<fn(String) -> Result<String, Box<dyn StdError>>> =
        vec![how_to_get_string_local, how_to_get_string_from_api];

    let state = SerDeState::create_from_yaml_str(functions, String::from("states/state.yaml"), 0)
        .unwrap()
        .unwrap();
    run(state);
}

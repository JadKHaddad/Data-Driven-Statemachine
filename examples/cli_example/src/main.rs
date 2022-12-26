use parking_lot::RwLock;
use statemachine::{
    serde_state::*,
    state::State,
    status::{InputStatus, OutputStatus},
};

use std::{fs::File, io::Read, sync::Arc};

fn run(root: Arc<RwLock<State>>) {
    let mut current_state: Arc<RwLock<State>> = root.clone();
    loop {
        let output_status: OutputStatus;
        let input_status: InputStatus;
        {
            {
                let mut current_state_ref = current_state.write();
                output_status = current_state_ref.output().unwrap();
            }

            if output_status.state_changed {
                if let Some(state) = output_status.state {
                    current_state = state;
                    continue;
                }
                if output_status.submit {
                    println!(
                        "submitting on output from state {}\n",
                        current_state.read().get_name()
                    );
                    let collections = current_state.write().collect().unwrap().unwrap();
                    println!("{:?}", collections);
                    break;
                }
            }

            println!("{:?}", output_status.output);

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
        }

        if input_status.state_changed {
            if let Some(state) = input_status.state {
                current_state = state;
            }
            if input_status.submit {
                println!(
                    "submitting on input from state {}\n",
                    current_state.read().get_name()
                );
                let collections = current_state.write().collect().unwrap().unwrap();
                println!("{:?}", collections);
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

    let state = SerDeState::create_from_yaml_str(functions, String::from("../states/state.yaml"), 0)
        .unwrap()
        .unwrap();
    run(state.clone());
    
    state.write().destroy();
}

pub struct A {
    pub name : String,
    pub parent: Option<Arc<RwLock<A>>>,
    pub next: Option<Arc<RwLock<A>>>,
}

impl A {
    pub fn destroy(&mut self) {
        println!("destroying {}", self.name);
        self.parent = None;
        if let Some(next) = &self.next {
            next.write().destroy();
        }
    }
}

impl Drop for A {
    fn drop(&mut self) {
        println!("dropping {}", self.name);
    }
}
    
use statemachine::{
    serde_state::*,
    status::{InputStatus, OutputStatus},
};

use std::{error::Error as StdError};
use std::{fs::File, io::Read};

use futures_util::{SinkExt, StreamExt};
use poem::{
    get, handler,
    listener::TcpListener,
    web::{
        websocket::{Message, WebSocket},
        Html, Path,
    },
    IntoResponse, Route, Server,
};

#[handler]
fn index() -> Html<&'static str> {
    Html(
        r###"
    <body>
        <form id="loginForm">
            Name: <input id="nameInput" type="text" />
            <button type="submit">Login</button>
        </form>
        
        <form id="sendForm" hidden>
            Text: <input id="msgInput" type="text" />
            <button type="submit">Send</button>
        </form>
        
        <textarea id="msgsArea" cols="50" rows="30" hidden></textarea>
    </body>
    <script>
        let ws;
        const loginForm = document.querySelector("#loginForm");
        const sendForm = document.querySelector("#sendForm");
        const nameInput = document.querySelector("#nameInput");
        const msgInput = document.querySelector("#msgInput");
        const msgsArea = document.querySelector("#msgsArea");
        
        nameInput.focus();
        loginForm.addEventListener("submit", function(event) {
            event.preventDefault();
            loginForm.hidden = true;
            sendForm.hidden = false;
            msgsArea.hidden = false;
            msgInput.focus();
            ws = new WebSocket("ws://127.0.0.1:3000/ws/" + nameInput.value);
            ws.onmessage = function(event) {
                msgsArea.value += event.data + "\r\n";
            }
        });
        
        sendForm.addEventListener("submit", function(event) {
            event.preventDefault();
            ws.send(msgInput.value);
            msgInput.value = "";
        });
    </script>
    "###,
    )
}

#[handler]
fn ws(Path(name): Path<String>, ws: WebSocket) -> impl IntoResponse {
    println!("{} connected", name);
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
    //user is connected
    //create the welcome state
    let how_to_get_string_local = |name: String| {
        let mut file = File::open(&name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Ok(contents)
    };
    let functions: Vec<fn(String) -> Result<String, Box<dyn StdError>>> =
        vec![how_to_get_string_local];

    let state = SerDeState::create_from_yaml_str(functions, String::from("states/state.yaml"), 0)
        .unwrap()
        .unwrap();

    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();
        tokio::spawn(async move {
            let mut current_state = state.clone();
            loop {
                let output_status: OutputStatus;
                let input_status: InputStatus;
                {
                    let mut current_state_g = current_state.write();
                    output_status = current_state_g.output().unwrap();
                }
                if output_status.state_changed {
                    if let Some(state) = output_status.state {
                        current_state = state;
                        continue;
                    }
                    if output_status.submit {
                        let collections = current_state.write().collect().unwrap().unwrap();
                        println!("{:?}", collections);
                        let _ = sender.send(format!("Thank you for your input!"));
                        break;
                    }
                }

                if sender.send(output_status.output.clone()).is_err() {
                    break;
                }

                if let Some(Ok(msg)) = stream.next().await {
                    if let Message::Text(input) = msg {
                        {
                            let mut current_state_g = current_state.write();
                            if input == "back" {
                                input_status = current_state_g.back();
                            } else {
                                input_status = current_state_g.input(input).unwrap();
                            }
                        }
                        if input_status.state_changed {
                            if let Some(state) = input_status.state {
                                current_state = state;
                            }
                            if input_status.submit {
                                let collections = current_state.write().collect().unwrap().unwrap();
                                println!("{:?}", collections);
                                let _ = sender.send(format!("Thank you for your input!"));
                                break;
                            }
                        }
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                if sink.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
            println!("dropped");
        });
    })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/", get(index)).at("/ws/:name", get(ws));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}

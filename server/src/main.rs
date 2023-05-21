mod client;
mod msg;
mod server;
mod session;

use std::io::stdin;

#[derive(Clone)]
struct State {}

#[tokio::main]
async fn main() {
    let adr = "localhost:1234";
    let mut app = tide::with_state(State {});
    app.at("/session/list").get(session::list);
    app.at("/session/create").post(session::create);
    app.at("/session/join").post(session::join);
    let _join = tokio::spawn(app.listen(adr));
    let mut input = String::new();
    loop {
        input.clear();
        stdin()
            .read_line(&mut input)
            .expect("failed to read line from stdin");
        match input.as_str().trim() {
            "q" | "quit" => break,
            cmd => eprintln!("invalid command '{cmd}'"),
        }
    }
}

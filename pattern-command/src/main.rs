use std::collections::VecDeque;

trait Command {
    fn execute(&mut self);
}

struct Invoker {
    history: VecDeque<Box<dyn Command>>
}

impl Invoker {
    fn execute(&mut self, command: Box<dyn Command>) {
        self.history.push_back(command);
        self.history.back_mut().unwrap().execute();
    }
}

#[derive(Clone, Copy)]
enum LightState {
    ON,
    OFF
}

#[derive(Clone, Copy)]
struct Light {
    state: LightState
}

impl Light {
    fn turn_on(&mut self) {
        self.state = LightState::ON;
        println!("State on");
    }

    fn turn_off(&mut self) {
        self.state = LightState::OFF;
        println!("State off");
    }
}

struct TurnOnCommand {
    light: Light
}

impl Command for TurnOnCommand {
    fn execute(&mut self) {
        self.light.turn_on();
    }
}

struct TurnOffCommand {
    light: Light
}

impl Command for TurnOffCommand {
    fn execute(&mut self) {
        self.light.turn_off();
    }
}

struct Client {
    light: Light,
    invoker: Invoker
}

impl Client {
    fn press(&mut self, state: LightState) {
        match state {
            LightState::ON => self.invoker.execute(Box::new(TurnOnCommand {light: self.light})),
            LightState::OFF => self.invoker.execute(Box::new(TurnOffCommand {light: self.light}))
        }
    }
}

fn main() {
    let mut _client = Client {
        light: Light {state: LightState::OFF},
        invoker: Invoker {history: VecDeque::new()}
    };
    _client.press(LightState::ON);
    _client.press(LightState::OFF);
    _client.press(LightState::ON);
}

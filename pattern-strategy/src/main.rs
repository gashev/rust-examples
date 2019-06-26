trait Strategy {
    fn algorithm(&self);
}

struct Strategy1;

impl Strategy for Strategy1 {
    fn algorithm(&self) {
        println!("Strategy1");
    }
}

struct Strategy2;

impl Strategy for Strategy2 {
    fn algorithm(&self) {
        println!("Strategy2");
    }
}

struct Client {
    strategy: Box<Strategy>
}

impl Client {
    fn algorithm(&self) {
        self.strategy.algorithm();
    }

    fn set_strategy(&mut self, strategy: Box<Strategy>) {
        self.strategy = strategy;
    }
}

fn main() {
    let mut client = Client { strategy: Box::new(Strategy1) };
    client.algorithm();
    client.set_strategy(Box::new(Strategy2));
    client.algorithm();
}
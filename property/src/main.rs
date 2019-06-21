trait Value<T> {
    fn set_value(&mut self, value: T);
    fn get_value(&self) -> &T;
}

#[derive(Default)]
struct S<T> {
    value: T,
}

impl<T> Value<T> for S<T> {
    fn set_value(&mut self, value: T) {
        self.value = value;
    }

    fn get_value(&self) -> &T {
        return &self.value;
    }
}

fn main() {
    let mut s: S<String> = Default::default();
    s.set_value(String::from("test"));
    println!("{}", s.get_value());
}
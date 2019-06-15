pub trait Test {
    fn test(&self) -> String {
        String::from("Default implementation")
    }
}

struct A;
impl Test for A {}

struct B;
impl Test for B {
    fn test(&self) -> String {
        String::from("B implementation")
    }
}
fn main() {
    let a: A = A {};
    println!("{}", a.test());

    let b: B = B {};
    println!("{}", b.test());
}

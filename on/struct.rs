fn main() {
    // let age = 12;
    // println!("{}", age);

    let mut foo = Foo::new(34);
    println!("{}", foo.get_value());
    foo.set_value(32)
}

// struct
struct Foo {
    value: i32,
}

// constructor
impl Foo {
    fn new(value: i32) -> Foo {
        return Foo { value: value };
    }

    // getter
    fn get_value(&self) -> i32 {
        self.value
    }

    // setter
    fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

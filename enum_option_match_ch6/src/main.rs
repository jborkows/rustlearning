fn main() {
    using_match_on_option();
    if_let_usage();
}

fn if_let_usage() {
    using_if_let(Foo::A(10));
    using_if_let(Foo::A(20));
    using_if_let(Foo::B);
    using_if_let(Foo::C);
}

fn using_match_on_option() {
    let five = Some(5);
    let six = plus_one(five);
    println!("5 + 1 is {:?}", six);
    let none = plus_one(None);
    println!("None + 1 is {:?}", none);
}

fn plus_one(value: Option<u32>) -> Option<u32> {
    match value {
        Some(expr) => Some(expr + 1),
        // None => None,//more specific
        _ => None, //deal with all not matched
    }
}

fn using_if_let(foo: Foo) {
    if let Foo::A(value) = foo {
        println!("Success {}", value);
    }
    if let Foo::A(10) = foo {
        println!("Only 10");
    }
}

enum Foo {
    A(u32),
    B,
    C,
}

fn main() {
    println!("Hello, world!");
    enums_like_in_java();
    enum_with_same_value();
    enum_fun();
}

fn enum_fun() {
    let _enum = MultipleDifferentValues::A(SomeStruct {
        field: String::from("AAA"),
    });
    println!("{:?}", _enum);
    let _enum = MultipleDifferentValues::B(12, String::from("AAA"));
    println!("{:?}", _enum);
    let _enum = MultipleDifferentValues::C { x: 12, y: 32 };
    println!("{:?}", _enum);
}

fn enum_with_same_value() {
    let _enum = EnumWithValue::A(String::from("AAAA"));
    let _enum = EnumWithValue::B(String::from("AAAA"));
}

fn enums_like_in_java() {
    let _enum_like_in_java = SomeEnum::A;
    let _enum_like_in_java = SomeEnum::B;
    let _enum_like_in_java = SomeEnum::C;
}

enum SomeEnum {
    A,
    B,
    C,
}

enum EnumWithValue {
    A(String),
    B(String),
}

#[derive(Debug)]
struct SomeStruct {
    field: String,
}

#[derive(Debug)]
enum MultipleDifferentValues {
    A(SomeStruct),
    B(u32, String),
    C { x: u32, y: u32 },
}

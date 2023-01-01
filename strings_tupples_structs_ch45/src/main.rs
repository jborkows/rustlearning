fn main() {
    println!("String slices");
    strings_and_slices();
    non_string_slices();
    struct_and_tupples();
}

fn struct_and_tupples() {
    tupples();
    named_structure();
    unnamed_field_structure();
}

fn unnamed_field_structure() {
    struct Color(i32, i32, i32);
    let _black = Color(0, 0, 0);
}

fn named_structure() {
    //struct with named fields
    struct User {
        username: String,
        age: u64,
        active: bool,
    }
    let _initialization = User {
        username: String::from("John"),
        age: 50,
        active: true,
    };
    //initialize with variable names as field in structure
    let username = String::from("Anna");
    let age = 32;
    let _second = User {
        active: false,
        username,
        age,
    };
    // something like copy in kotlin/scala or spread operator
    let structure_updated = User {
        active: true,
        .._second
    };
    println!(
        "{}, {}, {}",
        structure_updated.username, structure_updated.age, structure_updated.active
    );
}

fn tupples() {
    //tupples
    let _this_is_a_tupple = (1, "a", "whatever");
    let _take_a_value_from_tupple = _this_is_a_tupple.2;
}

fn non_string_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice_of_a = &a[2..];
    println!("Size of {} is {}", "[3, 4, 5]", len(&slice_of_a));
}

fn strings_and_slices() {
    let my_string = String::from("hello world");
    let _word = first_word(&my_string[..]);
    let _word = first_word(&my_string);
    // let _word = first_word(my_string);//compilation error
    let my_string_literal = "hello world";
    //shadowing of _word
    //literals are already a slice
    let _word = first_word(&my_string_literal[..]);
    let _word = first_word(my_string_literal);
    let _word = first_word(&my_string_literal);
    println!("{}", _word);
}

fn len(s: &[i32]) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] //slice containing all elements
}

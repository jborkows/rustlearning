pub fn start_fun() {
    expand_string();
    concatenated_two_string_with_borrowing();
    concatenated_two_string_without_borrowing();
    iterating_through_string();
}

fn iterating_through_string() {
    println!("Iterating through text");
    let var_name = &"gąskż";
    for c in var_name.chars() {
        //will print 5 characters
        println!("{}", c)
    }
    for c in var_name.bytes() {
        //will print more than 5 bytes!
        println!("{}", c)
    }
}

fn concatenated_two_string_without_borrowing() {
    let _s1 = String::from("Hello");
    let _s2 = String::from("World!");
    let _s3 = format!("{}-{}", _s1, _s2);
}

fn concatenated_two_string_with_borrowing() {
    let _s1 = String::from("Hello");
    let _s2 = String::from("World!");
    let _s3 = _s1 + &_s2;
    // println!("{}", _s1);// _s1 is borrowed to s3
}

fn expand_string() {
    let mut _s = String::from("foo");
    _s.push_str("aaaa"); //it accept slice of String
    _s.push('a'); //accepts single character
}

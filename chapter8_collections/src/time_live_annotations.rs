use std::fmt::Display;

pub fn start_fun() {
    let _string1 = String::from("AAAAAA");
    {
        let _string2 = String::from("BBBB");
        let result = longest(_string1.as_str(), _string2.as_str());
        println!("The longest string is {}", result);

        let result3 = longest_generic(_string1.as_str(), _string2.as_str(), 2);
        println!("The longest with generics is {}", result3);
    }
}

//'a give rust compiler a hint to restrict parameters to same aliveness scope -> both of parameters
//has to live inside of scope
//If result in line 5 would be moved as declaration to scope with _string1 and printing would be
//also in scope of string1 we would get error
//saying that string2 lives to shortly
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

fn longest_generic<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Annoncement: {}", ann);
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

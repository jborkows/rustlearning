struct Point {
    x: i32,
    y: i32,
    z: i32,
}
fn main() {
    struct_destruction();
    {
        let numbers = (2, 4, 6, 8, 10);
        match numbers {
            (first, _, third, _, _) => println!("Found {} and {}", first, third),
        }
    }
    {
        let s = Some(String::from("Hello"));
        // if let Some(_s) = s { //<- will take ownership of s!!!
        // if let Some(_) = s { //<- this works
        if let Some(ref _s) = s {
            // this borrows ownership
            println!("Found a string");
        }
        println!("{:?}", s);
    }
    mutable_borrowing_with_ref_mut();
    guarded_match();
    bindings_match();
}

fn bindings_match() {
    let point = Point { x: 1, y: 2, z: 3 };
    match point {
        Point {
            x: a @ (1 | 2 | 3),
            y,
            z,
        } => println!("AAA found a={}", a),
        _ => println!("Miss..."),
    }
}

fn guarded_match() {
    {
        let something = Some(5);
        match something {
            Some(x) if x > 6 => println!("Found"),
            _ => println!("Cannot find"),
        }
    }
    {
        let something = Some(15);
        match something {
            Some(x) if x > 6 => println!("Found"),
            _ => println!("Cannot find"),
        }
    }
}

fn struct_destruction() {
    {
        let point = Point { x: 1, y: 2, z: 3 };
        let Point { x: a, y: b, z: c } = point;
        println!("a = {a}, b= {b}, c={c}", a = a, b = b, c = c);
    }
    {
        let point = Point { x: 1, y: 2, z: 3 };
        let Point { x, y, z } = point;
        println!("a = {a}, b= {b}, c={c}", a = x, b = y, c = z);
    }
    {
        let point = Point { x: 1, y: 2, z: 3 };
        let Point { x, .. } = point;
        println!("a = {a} (Ignoring params)", a = x);
    }
    {
        let point = Point { x: 1, y: 2, z: 3 };
        match point {
            Point { x, y: 2, z } => println!("Found match"),
            Point { x, y, z } => println!("Nope"),
        }
    }
}

fn mutable_borrowing_with_ref_mut() {
    let mut robot_name = Some(String::from("AI"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Skynet"),
        None => (),
    }
    println!("robot_name is {:?}", robot_name);
}

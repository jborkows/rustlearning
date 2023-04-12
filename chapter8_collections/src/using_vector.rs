pub fn start_fun() {
    new_mutating_vector();
    new_vector_with_macro();
    little_tricki_get_push();
    iterating_vector();
}

fn iterating_vector() {
    let mut _v = vec![1, 2, 5, 6, 6];
    for i in &mut _v {
        *i += 50; //dereferencing object
    }
    for i in &_v {
        println!("{}", *i)
    }
}

fn little_tricki_get_push() {
    let mut _v = vec![1, 2, 5, 6, 6];
    let _first = &_v[0];
    // _v.push(1); // compilation error due to potential reallocation of vector
    println!("{}", _first);
}

fn new_vector_with_macro() {
    let _v = vec![1, 2, 3];
    let _element: &i32 = &_v[2]; //if out of scope will raise panic
    let _element: Option<&i32> = _v.get(2);
}

fn new_mutating_vector() {
    let mut _v: Vec<i32> = Vec::new();
    _v.push(5);
    _v.push(1);
}

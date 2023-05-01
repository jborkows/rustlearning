use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping/cleaning pointer with value ")
    }
}
#[test]
fn boxing() {
    let b = Box::new(5);
    println!("b= {}", b);
    assert_ne!(1, 2);
}
#[test]
fn deref() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    // assert_eq!(x, y); <-- will not compile
    assert_eq!(x, *y);
    assert_eq!(x, *z);
    assert_eq!(*y, *z);
    // assert_eq!(y, z); <-- will not compile
}

fn add_3(value: &MyBox<i32>) -> i32 {
    value.0 + 3
}

#[test]
fn my_boxing() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
    let z = MyBox::new(y);
    let z_z = MyBox::new(z);
    println!("Can use deref to have more fun ; {})", add_3(&z_z));
}

// enum List { // <- recursive list has infinite size
//     Cons(i32, List),
//     Nil,
// }
//
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum ListWhichCanShare {
    Cons(i32, Rc<ListWhichCanShare>),
    Nil,
}

#[test]
fn sharing_list() {
    {
        use List::{Cons, Nil};
        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let b = Cons(3, Box::new(a));
        // let c = Cons(3, Box::new(a)); //<-use of moved value a
    }
    {
        use ListWhichCanShare::{Cons, Nil};

        // let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
        // let b = Cons(3, Rc::new(a));
        // let c = Cons(3, Rc::new(a)); //<-use of moved value a

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        //Borrow checker rules will be enforce on runtime
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(3, Rc::clone(&a));
        println!("A counter has count of {}", Rc::strong_count(&a))
    }
}

#[test]
fn multiple_owners_of_data() {
    let value = RefCell::new(10);
    let a = Rc::new(&value);

    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    *b.borrow_mut() += 10;
    println!("After modyfing b a = {}", *a.borrow());
    *c.borrow_mut() += 20;
    println!("After modyfing c a = {}", *a.borrow());
    *a.borrow_mut() -= 100;
    *value.borrow_mut() *= 2;
}

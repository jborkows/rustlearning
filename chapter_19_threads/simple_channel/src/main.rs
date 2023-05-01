use std::{
    sync::mpsc,
    thread,
    time::{Duration, SystemTime},
};

fn log(message: &str) {
    let binding = thread::current();
    let name = match binding.name() {
        Some(expr) => expr,
        None => "main",
    };
    println!(
        "{time:?} {name}: {message}",
        time = SystemTime::now(),
        name = name,
        message = message
    );
}
fn main() {
    let (sender, receiver) = mpsc::channel();
    // thread::spawn(move || {
    thread::Builder::new()
        .name("worker".to_string())
        .spawn(move || {
            let val = String::from("hello");
            //does not wait for receiver to pick message
            sender.send(val).unwrap();
            log("Was send...")
            // println!("val is {}", val); // compilation error ;)
        })
        .unwrap();
    log("Sleeping before receiving");
    thread::sleep(Duration::from_secs(2));
    log("Sleeping ended");
    let received = receiver.recv().unwrap(); //recv will block till message
    log(&format!("Received {value}", value = received));
}

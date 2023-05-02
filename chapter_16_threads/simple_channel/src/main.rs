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
    simple_use();
    thread::sleep(Duration::from_secs(1));
    println!("###########################");
    receiver_as_iterator();
    println!("###########################");
    multiple_senders();
}

fn simple_use() {
    let (sender, receiver) = mpsc::channel();
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
    let received = receiver.recv().unwrap();
    //recv will block till message
    log(&format!("Received {value}", value = received));
}

fn receiver_as_iterator() {
    let (sender, receiver) = mpsc::channel();
    thread::Builder::new()
        .name("worker".to_string())
        .spawn(move || {
            let val = vec![String::from("hello"), String::from("word")];
            for single in val {
                sender.send(single).unwrap();
            }
            //does not wait for receiver to pick message
            log("Was send...")
            // println!("val is {}", val); // compilation error ;)
        })
        .unwrap();
    for received in receiver {
        log(&format!("Received {value}", value = received));
    }
}

fn multiple_senders() {
    let (sender, receiver) = mpsc::channel();
    let sender_in_first = mpsc::Sender::clone(&sender);
    thread::Builder::new()
        .name("worker1".to_string())
        .spawn(move || {
            let val = vec![String::from("hello"), String::from("word")];
            for single in val {
                sender_in_first.send(single).unwrap();
            }
        })
        .unwrap();

    // If sender will not be closed/destroyed receiver will hang...
    // let sender_for_second = mpsc::Sender::clone(&sender);
    thread::Builder::new()
        .name("worker2".to_string())
        .spawn(move || {
            let val = vec![String::from("hello2"), String::from("word2")];
            for single in val {
                // sender_for_second.send(single).unwrap();
                sender.send(single).unwrap();
            }
        })
        .unwrap();
    for received in receiver {
        log(&format!("Received {value}", value = received));
    }
}

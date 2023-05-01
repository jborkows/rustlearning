use std::cell::RefCell;

trait Sender {
    fn send(&self, arg: &str);
}

struct MockSender {
    messages: RefCell<Vec<String>>,
}

impl MockSender {
    fn new() -> MockSender {
        MockSender {
            messages: RefCell::new(vec![]),
        }
    }
}

impl Sender for MockSender {
    fn send(&self, arg: &str) {
        self.messages.borrow_mut().push(String::from(arg));
    }
}

fn send_message<T>(sender: &T, msg: &str)
where
    T: Sender,
{
    sender.send(msg);
}

#[test]
fn show_mocking_in_action() {
    let sender = MockSender::new();

    send_message(&sender, "AAAA");
    send_message(&sender, "BBBB");
    assert_eq!(sender.messages.borrow().len(), 2);
}

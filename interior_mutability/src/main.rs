use interior_mutability::{LimitTracker, Messenger};
use std::cell::RefCell;

fn main() {
    #[derive(Debug)]
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message))
        }
    }

    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(93);
    limit_tracker.set_value(77);
    println!("MockMessenger: {:?}", mock_messenger.sent_messages);
}

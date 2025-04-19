use std::rc::Rc;
use std::cell::RefCell;
use ref_cell::messenger::{Logger, Tracker};

struct TestLogger {
    pub messages: RefCell<Vec<String>>,
    pub value: Rc<usize>,
}

impl Logger for TestLogger {
    fn warning(&self, msg: &str) {
        self.messages.borrow_mut().push(format!("Warning: {}", msg));
    }
    fn info(&self, msg: &str) {
        self.messages.borrow_mut().push(format!("Info: {}", msg));
    }
    fn error(&self, msg: &str) {
        self.messages.borrow_mut().push(format!("Error: {}", msg));
    }
}

fn main() {
    let val = Rc::new(100);
    let logger = TestLogger {
        messages: RefCell::new(Vec::new()),
        value: Rc::clone(&val),
    };

    let tracker = Tracker::new(&logger, 5);
    tracker.set_value(&logger.value);
    tracker.peek(&logger.value);

    for m in logger.messages.borrow().iter() {
        println!("{}", m);
    }
}

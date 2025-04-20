pub mod messenger;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use messenger::Logger;

/// Worker struct that holds value and logs.
pub struct Worker {
    pub track_value: Rc<RefCell<usize>>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(val: usize) -> Self {
        Worker {
            track_value: Rc::new(RefCell::new(val)),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        let formatted = format!("Warning: {}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert("Warning".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(formatted);
    }

    fn info(&self, msg: &str) {
        let formatted = format!("Info: {}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert("Info".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(formatted);
    }

    fn error(&self, msg: &str) {
        let formatted = format!("Error: {}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert("Error".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(formatted);
    }
}

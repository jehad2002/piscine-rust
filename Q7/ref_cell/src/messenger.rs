use std::cell::RefCell;
use std::rc::Rc;

/// Logger trait to log messages.
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

/// Tracker struct that checks how much a value is referenced.
pub struct Tracker<'a, T: Logger> {
    logger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: usize) -> Tracker<'a, T> {
        Tracker {
            logger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&self, val: &Rc<RefCell<usize>>) {
        let count = Rc::strong_count(val);
        let percentage = (count * 100) / self.max;

        if percentage >= 100 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70 {
            self.logger.warning(&format!(
                "you have used up over {}% of your quota! Proceeds with precaution",
                percentage
            ));
        }

        // self.value is not mutated because it's not marked as mutable.
        // But according to the task, we don't really need to store it.
    }

    pub fn peek(&self, val: &Rc<RefCell<usize>>) {
        let count = Rc::strong_count(val);
        let percentage = (count * 100) / self.max;

        self.logger
            .info(&format!("you are using up to {}% of your quota", percentage));
    }
}

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T: Logger> {
    logger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: usize) -> Self {
        Self {
            logger,
            value: 0,
            max,
        }
    }

    pub fn set_value<U>(&mut self, value: &std::rc::Rc<U>) {
        self.value = std::rc::Rc::strong_count(value);
        let percent = (self.value * 100) / self.max;

        if percent >= 100 {
            self.logger.error("Error: you are over your quota!");
        } else if percent >= 70 {
            self.logger.warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                percent
            ));
        }
    }

    pub fn peek<U>(&self, value: &std::rc::Rc<U>) {
        let current = std::rc::Rc::strong_count(value);
        let percent = (current * 100) / self.max;
        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            percent
        ));
    }
}

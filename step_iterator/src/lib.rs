use std::ops::Add;
#[derive(Debug)] // give struct 
pub struct StepIterator<T: Add<Output = T>> {
    pub begin: T, //start
    pub end: T, // end
    pub step: T, // steps for iterator
}
    
impl<T: Add<Output = T>> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator { begin: beg, end, step } // calling struct 
    }
}
    
impl<T> std::iter::Iterator for StepIterator<T>
where T: Add<Output = T> + PartialEq<T> + Clone + PartialOrd {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { // function for next mut 
        let result: T; 

        if self.begin > self.end {
            return None;
        }else{
            result = self.begin.clone();
            self.begin = self.begin.clone() + self.step.clone();
        }

        return Some(result);
    }
}
// test it
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
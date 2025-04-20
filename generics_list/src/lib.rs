#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    // Create a new empty list
    pub fn new() -> List<T> {
        List { head: None }
    }

    // Push a new element to the beginning of the list (LIFO style)
    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take(), // take the current head and assign as next
        };
        self.head = Some(Box::new(new_node));
    }

    // Pop an element from the front of the list
    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next;
        }
    }

    // Return the length of the list
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }

        count
    }
}

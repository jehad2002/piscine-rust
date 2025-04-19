pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    // إنشاء عقدة جديدة بقائمة مرجعية
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list }
    }

    // إضافة عنصر للقائمة
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    // إزالة جميع العناصر التي تشير لنفس التخصيص
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|x| !Rc::ptr_eq(x, &element));
    }
}

// إرجاع عدد المراجع لنفس القيمة
pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}

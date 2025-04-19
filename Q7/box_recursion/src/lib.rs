#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    // إنشاء بيئة عمل جديدة بدون أي عامل
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    // إضافة عامل في بداية القائمة
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: self.grade.take(), // تأخذ ما في grade وتضعه كالتالي للعامل الجديد
        };
        self.grade = Some(Box::new(new_worker));
    }

    // إزالة آخر عامل تمت إضافته (أول عنصر في القائمة)
    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(boxed_worker) = self.grade.take() {
            self.grade = boxed_worker.next; // تعيين العنصر التالي كالعنصر الحالي
            Some(boxed_worker.name)
        } else {
            None
        }
    }

    // عرض اسم ودور آخر عامل تمت إضافته
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|worker| (worker.name.clone(), worker.role.clone()))
    }
}

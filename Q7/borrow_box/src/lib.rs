#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    // إنشاء جلسة جديدة داخل Box
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        })
    }

    // إرجاع الفائز الحالي أو حالة التعادل
    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 > self.p2.1 {
            self.p1.clone()
        } else if self.p2.1 > self.p1.1 {
            self.p2.clone()
        } else {
            ("Same score! tied".to_string(), self.p1.1)
        }
    }

    // تحديث النقاط للاعب معين
    pub fn update_score(&mut self, user_name: String) {
        if self.is_finished() {
            return;
        }

        if self.p1.0 == user_name {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name {
            self.p2.1 += 1;
        }
    }

    // حذف الجلسة
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }

    // دالة داخلية لتحديد إذا كانت اللعبة قد انتهت
    fn is_finished(&self) -> bool {
        let max_score = self.nb_games / 2 + 1;
        self.p1.1 >= max_score || self.p2.1 >= max_score
    }
}

use chrono::{Date, Local};

fn main() {
    fn_4_1();
}

struct DueDate {
    due_date: Date<Local>
}

impl DueDate {
    fn new(due_date: Date<Local>) -> Self {
        DueDate { due_date }
    }

    fn is_expired(&self, date: &Date<Local>) -> bool {
        date > &self.due_date
    }
}

fn fn_4_1() {
    // 期日の業務ルールを扱うクラス
    let due_date = DueDate::new(Local::today());
    println!("{}", due_date.is_expired(&Local::today()));
}

use chrono::{Date, Local};
use std::collections::HashSet;
use std::hash::Hash;

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

trait Rule {
    fn ok(&self, value: i32) -> bool;
    fn ng(&self, value: i32) -> bool {
        self.ok(value)
    }
    fn eq(&self) -> bool;
    fn hash(&self);
}

impl PartialEq for dyn Rule {
    fn eq(&self, other: &Self) -> bool {
        self.eq()
    }
}

impl Hash for dyn Rule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash()
    }
}

impl Eq for dyn Rule {
    
}

struct Policy {
    rules: HashSet<Box<dyn Rule>>
}

impl Policy {
    fn new() -> Self {
        let rules: HashSet<Box<dyn Rule>> = HashSet::new();
        Policy { rules }
    }

    fn comply_with_all(&self, value: i32) -> bool {
        for rule in &self.rules {
            if rule.ng(value) {
                return false;
            }
        }
        true
    }

    fn add_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.insert(rule);
    }
}

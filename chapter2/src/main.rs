fn main() {
    fn_2_1();
    fn_2_2();
    fn_2_3();
}

struct Yen { 
    value: i32,
}

impl Yen {
    fn new(value: i32) -> Self {
        Yen { value }
    }
}

fn is_baby(cutomer_type: &str) -> bool {
    cutomer_type == "baby"
}

fn baby_fee(base_fee: i32) -> Yen {
    Yen::new((base_fee as f32 * 0.2) as i32)
}

fn is_child(cutomer_type: &str) -> bool {
    cutomer_type == "child"
}

fn child_fee(base_fee: i32) -> Yen {
    Yen::new((base_fee as f32 * 0.5) as i32)
}

fn is_senior(cutomer_type: &str) -> bool {
    cutomer_type == "senior"
}

fn senior_fee(base_fee: i32) -> Yen {
    Yen::new((base_fee as f32 * 0.8) as i32)
}

fn adult_fee(base_fee: i32) -> Yen {
    Yen::new((base_fee as f32 * 1.0) as i32)
}

fn fn_2_1() {
    // メソッドに抽出する
    if is_child("child") {
        let _fee = child_fee(10);
    }
}

fn fn_2_2() {
    fn fee() -> Yen {
        // else句をなくした書き方
        if is_child("child") { return child_fee(10); }
        if is_senior("senior") { return senior_fee(10); }
        adult_fee(10)
    }
    
    let yen = fee();
    println!("{}", yen.value);
}

fn fn_2_3() {
    fn fee() -> Yen {
        // 複文は単文に分ける
        if is_baby("baby") { return baby_fee(10); }
        if is_child("child") { return child_fee(10); }
        if is_senior("senior") { return senior_fee(10); }
        adult_fee(10)
    }
    
    let yen = fee();
    println!("{}", yen.value);
}
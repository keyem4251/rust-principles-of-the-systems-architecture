fn main() {
    fn_2_1();
    fn_2_2();
    fn_2_3();
    fn_2_4();
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

fn fn_2_4() {
    // 区分ごとのロジックを別のクラスに分けて記述する
    struct AdultFee {}
    
    impl AdultFee {
        fn new() -> Self {
            AdultFee {}
        }

        fn fee(&self) -> Yen {
            Yen::new(100)
        }

        fn label(&self) -> String {
            "大人".to_string()
        }
    }

    struct ChildFee {}
    
    impl ChildFee {
        fn new() -> Self {
            ChildFee {}
        }
        fn fee(&self) -> Yen {
            Yen::new(50)
        }

        fn label(&self) -> String {
            "子供".to_string()
        }
    }

    struct SeniorFee {}

    impl SeniorFee {
        fn new() -> Self {
            SeniorFee {}
        }
        fn fee(&self) -> Yen {
            Yen::new(80)
        }

        fn label(&self) -> String {
            "シニア".to_string()
        }
    }

    let fee1 = AdultFee::new();
    println!("{}: {}", fee1.label(), fee1.fee().value);

    let fee2 = ChildFee::new();
    println!("{}: {}", fee2.label(), fee2.fee().value);

    let fee3 = SeniorFee::new();
    println!("{}: {}", fee3.label(), fee3.fee().value);
}
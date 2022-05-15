fn main() {
    fn_2_1();
    fn_2_2();
    fn_2_3();
    fn_2_4();
    fn_2_5();
    fn_2_6();
    fn_2_7();
    fn_2_8();
}

struct Yen { 
    value: i32,
}

impl Yen {
    fn new(value: i32) -> Self {
        Yen { value }
    }

    fn add(&self, yen: &Yen) -> Self {
        Yen { value: self.value + yen.value }
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

fn fn_2_5() {
    // 区分ごとのクラスを同じ型をとして扱う
    trait Fee {
        fn yen(&self) -> Yen;
        fn label(&self) -> String;
    }

    struct AdultFee {}

    impl AdultFee {
        fn new() -> Self {
            AdultFee {}
        }
    }

    impl Fee for AdultFee {
        fn yen(&self) -> Yen {
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
    }

    impl Fee for ChildFee {
        fn yen(&self) -> Yen {
            Yen::new(50)
        }

        fn label(&self) -> String {
            "子供".to_string()
        }
    }

    struct Charge { 
        fee: Box<dyn Fee>,
    }

    impl Charge {
        fn new(fee: Box<dyn Fee>) -> Self {
            Charge { fee }
        } 

        fn yen(&self) -> Yen {
            self.fee.yen()
        }
    }

    struct Reservation {
        fees: Vec<Box<dyn Fee>>
    }

    impl Reservation {
        fn new() -> Self {
            Reservation {
                fees: vec![],
            }
        }

        fn add_fee(&mut self, fee: Box<dyn Fee>) {
            self.fees.push(fee);
        }

        fn fee_total(&self) -> Yen {
            let mut total = Yen::new(0);
            for fee in &self.fees {
                total = total.add(&fee.yen());
            }
            total
        }
    }

    let adult_fee_struct = AdultFee::new();
    let child_fee_struct = ChildFee::new();
    let adult_charge = Charge::new(Box::new(adult_fee_struct));
    let child_charge = Charge::new(Box::new(child_fee_struct));
    println!("2_5: {}", adult_charge.yen().value);
    println!("2_5: {}", child_charge.yen().value);

    let mut reservation = Reservation::new();
    let adult_fee_struct2 = AdultFee::new();
    let child_fee_struct2 = ChildFee::new();
    reservation.add_fee(Box::new(adult_fee_struct2));
    reservation.add_fee(Box::new(child_fee_struct2));
    println!("2_5: {}", reservation.fee_total().value);
}

fn fn_2_6() {
    // if文を使わずに区分ごとのオブジェクトを生成する
    trait Fee {
        fn yen(&self) -> Yen;
        fn label(&self) -> String;
    }

    struct AdultFee {}

    impl AdultFee {
        fn new() -> Self {
            AdultFee {}
        }
    }

    impl Fee for AdultFee {
        fn yen(&self) -> Yen {
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
    }

    impl Fee for ChildFee {
        fn yen(&self) -> Yen {
            Yen::new(50)
        }

        fn label(&self) -> String {
            "子供".to_string()
        }
    }

    struct FeeFactory {}

    impl FeeFactory {
        fn fee_by_name(name: &str) -> Box<dyn Fee> {
            match name {
                "adult" => Box::new(AdultFee::new()),
                "child" => Box::new(ChildFee::new()),
                _ => panic!("不正な区分です"),
            }
        }
    }

    let adult_fee_struct = FeeFactory::fee_by_name("adult");
    println!("2_6: {}", adult_fee_struct.yen().value);
}

fn fn_2_7() {
    // 列挙型を使用する
    #[derive(PartialEq)]
    enum FeeType {
        Adult,
        _Child,
        _Senior,
    }

    struct Guest {
        fee_type: FeeType,
    }

    impl Guest {
        fn new() -> Self {
            Guest { fee_type: FeeType::Adult }
        }

        fn is_adult(&self) -> bool {
            self.fee_type == FeeType::Adult
        }
    }
    let guest = Guest::new();
    println!("2_7: {}", guest.is_adult());
}

fn fn_2_8() {
    // enumにロジックを入れる
    struct AdultFee {}

    impl AdultFee {
        fn new() -> Self {
            AdultFee {}
        }
    }

    struct ChildFee {}

    impl ChildFee {
        fn new() -> Self {
            ChildFee {}
        }
    }

    enum FeeType {
        Adult(AdultFee),
        Child(ChildFee),
    }

    impl FeeType {
        fn of_adult_fee() -> Self {
            FeeType::Adult(AdultFee::new())
        }

        fn of_child_fee() -> Self {
            FeeType::Child(ChildFee::new())
        }

        fn yen(&self) -> Yen {
            match self {
                FeeType::Adult(_s) => Yen::new(100),
                FeeType::Child(_s) => Yen::new(50),
            }
        }

        fn label(&self) -> String {
            match self {
                FeeType::Adult(_s) => "大人".to_string(),
                FeeType::Child(_s) => "子供".to_string(),
            }
        }
    }

    let adult_fee_type = FeeType::of_adult_fee();
    println!("2_8: {}", adult_fee_type.yen().value);

    let child_fee_type = FeeType::of_child_fee();
    println!("2_8: {}", child_fee_type.label());
}

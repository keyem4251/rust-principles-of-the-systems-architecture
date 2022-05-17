fn main() {
    fn_3_1();
}

fn fn_3_1() {
    // メソッドはインスタンス変数を使い、クラスが肥大化したら小さく分ける
    struct Customer {
        person_name: PersonName,
    }

    impl Customer {
        fn new(person_name: PersonName) -> Self {
            Customer { person_name }
        }
    }

    struct PersonName {
        first_name: String,
        last_name: String,
    }

    impl PersonName {
        fn new(first_name: String, last_name: String) -> Self {
            PersonName {
                first_name,
                last_name,
            }
        }

        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }

    let person_name = PersonName::new("first_name".to_string(), "last_name".to_string());
    let customer = Customer::new(person_name);
    println!("{}", customer.person_name.full_name());
}

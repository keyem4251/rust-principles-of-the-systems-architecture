fn main() {
    fn_2_1();
}

fn fn_2_1() {
    // メソッドに抽出する
    fn is_child(cutomer_type: &str) -> bool {
        cutomer_type == "child"
    }

    fn child_fee(base_fee: f32) -> f32 {
        base_fee * 0.5
    }

    if is_child("child") {
        let _fee = child_fee(10.0);
    }
}

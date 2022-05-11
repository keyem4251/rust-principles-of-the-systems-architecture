fn main() {
    fn_1_1();
    fn_1_2();
    fn_1_3();
    fn_1_4();
}

fn fn_1_1() {
    // 切れ目がはっきりしないコード
    let quantity = 10;
    let unit_price = 2;
    let mut _price = quantity * unit_price;
    if _price < 3000 {
        _price += 500;
    }
    _price = _price * 30;
}

fn fn_1_2() {
    // 空白行を入れて3つの段落に分けたコード
    let quantity = 10;
    let unit_price = 2;
    let mut _price = quantity * unit_price;

    if _price < 3000 {
        _price += 500;
    }

    _price = _price * 30;
}

fn fn_1_3() {
    // 目的ごとに変数を用意する
    let quantity = 10;
    let unit_price = 2;
    let base_price = quantity * unit_price;

    let mut shipping_cost = 0;
    if base_price < 3000 {
        shipping_cost = 500;
    }

    let _item_price = (base_price + shipping_cost) * 30;
}

fn fn_1_4() {
    // メソッドとして独立させる
    let quantity = 10;
    let unit_price = 2;
    let base_price = quantity * unit_price;

    let shipping_cost = shipping_cost(base_price);

    let _item_price = (base_price + shipping_cost) * 30;
}

fn shipping_cost(base_price: i32) -> i32 {
    if base_price < 3000 {
        return 500;
    }
    0
}

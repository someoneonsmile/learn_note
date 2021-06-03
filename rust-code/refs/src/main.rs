fn main() {
    let s = String::from("hellow world!");
    // 以对象的引用作为参数, 而不获取所有权
    let len = calculate_len(&s);
    println!("The len of s is {}", len);

    // 可变引用
    let mut s = s;
    {
        let _ = &mut s;
    }
    let r1 = &mut s;
    // err: 只能同时存在一个可变引用
    // let r2 = &mut s;
    // err: 不能同时存在可变与不可变引用
    // let r2 = &s;
    change(r1);
    println!("The value of s is {}", s);

    no_dangle();
}

// 引用类型作为函数参数称为借用
fn calculate_len(s: &String) -> usize {
    s.len()
}

// 可变引用
fn change(s: &mut String) {
    s.push_str("!!");
}

// 悬垂引用
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

use std::collections::HashMap;

fn main() {
    vec();
    string();
    hash_map();
}

fn vec() {
    // new 方式创建
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    // vec! 宏方式创建
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    // 遍历
    for i in &v {
        println!("{}", i);
    }

    // 遍历可变引用
    for i in &mut v {
        *i += 6;
    }
    println!("{:?}", v);

    // 索引语法或者 get 方法访问
    let third = &v[2];
    println!("third: {}", third);

    match v.get(2) {
        Some(third) => println!("Option third: {}", third),
        None => println!("None"),
    }

    if let Some(does_not_exist) = v.get(10) {
        println!("Index of 10 is {}", does_not_exist);
    }

    // pannic!
    // let does_not_exist = &v[10];
    // println!("Index of 10 is {}", does_not_exist);

    // vec 通过枚举可以存储不同的数据类型
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(3),
        Cell::Float(6.0),
        Cell::Text(String::from("12")),
    ];
    println!("{:?}", row);
}

fn string() {
    // 创建方式
    let _s = String::new();
    let _s = "".to_string();
    let mut s = String::from("");

    // 更改字符串
    s.push_str("foo");
    println!("{}", s);
    let s2 = String::from("bar");
    s.push_str(&s2);
    println!("{}", s);
    s.push_str(&s2[..]);
    println!("{}", s);

    let foo = String::from("foo");
    let bar = String::from("bar");

    // format 拼接字符串
    let s = format!("{}-{}", foo, bar);
    println!("foo: {}, bar: {}, s: {}", foo, bar, s);

    // "+" 调用 add 函数, 签名: fn add(self, s: &str) -> String
    // &String 解引用强制多态转为 &str
    let s = foo + &bar;
    // foo 已经被 move 了, 此处不能再使用
    // println!("foo: {}", foo);
    println!("add: {}", s);
    println!("bar: {}", bar);

    // 遍历字符
    for c in "卝芈".chars() {
        println!("{}", c);
    }
}

fn hash_map() {
    // 新建 map
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("blue"), String::from("red")];
    let init_scores = vec![10, 20];
    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
    println!("scores: {:?}", scores);

    // 所有权
    let filed_name = String::from("filed_name");
    let filed_value = String::from("filed_value");
    let mut map = HashMap::new();
    map.insert(filed_name, filed_value);
    // 编译失败, filed_name, filed_value 不再有效
    // println!("filed_name: {}, filed_value: {}", filed_name, filed_value);

    // map get
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);
    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);
    println!("{:?}", score.unwrap_or(&0));

    // 遍历 map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新 map
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // option
    println!("{:?}", None::<i32>.unwrap_or_default());
}

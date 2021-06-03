fn main() {
    println!("this is main");
    another_function(5, 6);

    // not allow
    // let x = (let y = 6);

    // 表达式有返回值, 语句没有返回值
    // {} 宏调用, 函数调用都是表达式
    let x = 5;
    let y = {
        let x = 6;
        x + 1
    };
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    plus_one(five());

    branches();
}

// 函数参数标注类型注解
fn another_function(x: i32, y: i32) {
    println!("this is another function");
    println!("x: {}, y: {}", x, y);
}

fn five() -> i32 {
    5
}

// -> 标记返回值类型
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn branches() {
    // if expression
    let x = if true { 5 } else { 6 };
    println!("Condition value {}", x);

    // loop expression
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // 倒计时
    for number in (1..=3).rev() {
        println!("{}", number);
    }
}

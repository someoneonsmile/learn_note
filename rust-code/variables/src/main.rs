fn main() {
    // 变量默认不可变, 可用 mut 修饰为可变变量
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // 常量可以在任何域中声明, 总是不可变
    // 总是标注类型, 只能设置为常量表达式
    const MAX_POINTS: u32 = 10_000;
    println!("The const value of MAX_POINTS is {}", MAX_POINTS);

    // shadowing, 不同于可变变量
    // shadowing (隐藏) 只是覆盖变量名称
    // 可以改变变量类型
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Then len of spaces is {}", spaces);
}

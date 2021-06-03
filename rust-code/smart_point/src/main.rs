use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop fn call!")
    }
}

fn main() {
    let x = MyBox::new(String::from("rust"));
    println!("{}", *x);
    // std::mem::drop, 位于 prelude
    // 手动提早释放, 会自动调用析构函数
    drop(x);
    println!("end!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dref_test() {
        let x = 6;
        let y = &x;
        assert_eq!(x, *y);
    }

    #[test]
    fn dref_box_test() {
        let x = 6;
        let y = Box::new(x);
        assert_eq!(x, *y);
    }

    #[test]
    fn dref_mybox_test() {
        let x = 6;
        let y = MyBox::new(x);
        assert_eq!(x, *y);
    }
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

/// 泛型实现方法
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
    /// 方法泛型
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

/// 为特定泛型类型实现方法
impl Point<f32, f32> {
    fn distince(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 5 };
    let x = p.x();
    let y = p.y();
    println!("x: {}, y: {}", x, y);
    // method not found in `Point<{integer}>`
    // println!("distince of {:?}: {}", p, p.distince());

    let p = Point { x: 5.0, y: 5.0 };
    println!("distince of {:?}: {}", p, p.distince());

    let p1 = Point { x: "5", y: 5 };
    let p2 = Point { x: 5, y: "5" };
    let p3 = p1.mixup(p2);
    println!("mixup: {:?}", p3);
}

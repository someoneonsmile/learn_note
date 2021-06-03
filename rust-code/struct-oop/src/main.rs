#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

impl Rectange {
    // 关联函数
    fn square(size: u32) -> Rectange {
        Rectange {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectange) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectange {
        width: 10,
        height: 5,
    };
    let rect2 = Rectange {
        width: 6,
        height: 3,
    };
    // {} Rectange doesn't implemented the `std::format::Display`
    // {:?} Rectange doesn't implemented the `std::format::Display`
    println!("the rectange is {:?}", rect);
    println!("the rectange is {:#?}", rect);

    println!("the area of rectange {:?} is {}", rect, rect.area());
    println!("can hold is {}", rect.can_hold(&rect2));
    println!("the square is {:?}", Rectange::square(32));
}

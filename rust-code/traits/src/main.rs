pub trait Summary {
    fn sum_author(&self) -> String;

    /// 默认实现, 可以调用其它没有默认实现的函数
    fn sum(&self) -> String {
        format!("Read more from {}...", self.sum_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn sum_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn sum(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn sum_author(&self) -> String {
        format!("@{}", self.author)
    }

    // fn sum(&self) -> String {
    // format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

/// trait 为参数
pub fn notify<T: Summary>(item: &T) {
    println!("{}", item.sum());
}

pub fn notify_two<T>(item1: &T, item2: &T)
where
    T: Summary,
{
    println!("{} - {}", item1.sum(), item2.sum());
}

fn main() {
    let tweet = Tweet {
        username: String::from("alis"),
        content: String::from("my day"),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
    // println!("{}", tweet.sum());

    let article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
            hockey team in the NHL.",
        ),
    };
    notify(&article);
    // println!("{}", article.sum());

    // notify_two(&tweet, &article);
    notify_two(&tweet, &tweet);
    notify_two(&article, &article);
}

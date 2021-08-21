fn main() {
    println!("Hello, world!");
    let inner = map(None);
    println!("{:?}", inner);

}

fn map(out: Option<Out>) -> Option<Inner> {
    out?.inner
}

#[derive(Debug)]
struct Out {
    inner: Option<Inner>
}

#[derive(Debug)]
struct Inner {
    msg: String
}

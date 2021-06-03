use std::fmt::Debug;

/// ref 关键字的作用
/// ref 同 &
fn main() {
    let x = false;
    print_type_name_of(x);
    // bool

    let x = &false;
    print_type_name_of(x);
    // &bool

    let x = &false;
    print_type_name_of(&x);
    // &&bool

    let &x = &false;
    print_type_name_of(x);
    // bool

    let &x = &false;
    print_type_name_of(&x);
    // &bool

    let ref x = &false;
    print_type_name_of(x);
    // &&bool

    print_type_name_of(Some(x));
    // Some(false):core::option::Option<&&bool>
}

fn print_type_name_of<T: Debug>(t: T) {
    println!("{:?}:{}", t, std::any::type_name::<T>());
}

// false:bool
// false:&bool
// false:&&bool
// false:bool
// false:&bool
// false:&&bool

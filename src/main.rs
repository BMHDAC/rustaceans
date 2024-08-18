pub mod variables;

fn main() {
    let x = 69;

    let mut y = &x; // y is &i32
    println!("{}", y);
    y = &420; // You can change the values of y, but not the value it pointed to

    println!("{}", y);
    let z = &mut y; //z is &mut &i32
    println!("{}", z);
    //z = &mut &x; // You cannot change the value of z to hold different reference
    *z = &mut &32; // But you can deref z and get a mutable ref
    println!("{}", z);
}

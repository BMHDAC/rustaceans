// Rust memory model  centers on the idea that all values have a single owner.
// And this model is enforced by the borrow checker
// If the value is moved, owner ship is transfered to the next owner, and  you cannot access the
// value from the variable that flow from the original owner
// If a the values type implement the Copy trait, the values is not considered to have been moved
// or reassigned, but copied instead. And both the old and new value remain accessible.
// Variables that do not implement the copy trait, are dropped when they goes out of scope
fn main() {
    let x: i32 = 420;

    let y: Box<i32> = Box::new(69);

    {
        let z = (x, y);
    }
    let x2 = x; // x is copied so it's still here

    let y2 = y; // Box is not Copy, so it was moved into z
}

// Rust allow for the owner of a variable to lend out that values to other without giving up owner
// ship through references. References are pointer that come with addition contract for how they
// can be used.
// We mainly have 2 type of references: Shared References and Mutable References
// Shared References assume that the values will not change
// Mutable References assume that there's no other threads, or flows access the target value,
// whether to Shared of Mutable ref
fn borrow_and_lifetimes() {
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

// There're also types that provide interior mutability, meaning they allow you to mutate a value
// through shared reference. These types fall into 2 categories: Those that let you get a mutable
// reference through shared one, and those that let you replace value given only shared reference
// 1st category consits of types liek Mutex, RefCell
// 2nd category consists of those that give you method to manipulate data in place, like AtomicI32,
// Cell

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // A value is stored in a 'place'. In rust it's called a localtion that can hold values.
    // A pointer is a value that holds the address of a region of memory.
    let x: i32 = 43; // x is a location that hold the value of 43 as an 32 bit interger
    let y: i32 = 32; // y is the same as x for 32
    let pointer_x: &i32 = &x; // pointer_x is a localtion that hold the address of x. We call.
                              // it reference to x.
                              // In short we can just says pointer_x store a value &43
    let mut pointer_y: &i32 = &x;
    pointer_y = &y; // Even though pointer_x and pointer_y store the samve value at first, but
                    // they  store separate copies of that values, hence if we
                    // change pointer_y, the value of pointer_x stayed unchanged

    // If we look at this particular example, we do not store the actual value of the string,
    // but rather a pointer into it, that's  called the string slice.
    // If we want to get the value out of string_slice, we can use string_slice.to_owned()
    // method
    let string_slice: &str = "Hello world";

    // THIS IS FOR THE CONCEPT CALLED THE HIGH-LEVEL MODEL
    // We DO NOT think of variables as place that hold actually bytes. But as names given to
    // values as they are initiated.
    // Using this model, the program will consists of flows, each one tracing the lifetime of
    // an instance of a value
    // Flow can fork and merge if there're branches
    // There're rules to the flows. You cannot have two parallel flows with mutable access. Or
    // any flow if there's no value.
    // That is why the variables is only accessible once it has given value. Consider the
    // following source code:
    let mut x: i32; // you cannot access X before it's given a value because there's no where
                    // to draw the flow from
    x = 32; // The flow is initiated, you call draw a flow from the value 32
            //
    {
        let y = &x; // This make a second mutable flow from x;
    }

    x = 42; // X is already borrowed as mutable, you can't borrow it anymore here. In details,
            // it created another mutable flows from x, which conflicts with flow from y.
    assert_eq!(*y, 42); // this is unreacheable

    // THIS IS ANOTHER CONCEPT CALLED LOW-LEVEL MODEL
    // In the low-level model, you can think of variables as value slot. When you assign a
    // variable to it, the slot is filled, if it had values, the old values is dropped and
    // replaced
    // You cannot access an empty slot. Which mean the variable is either moved or not
    // initialized
    // A pointer to a variables refers to that variables's backing memory and can be deref to
    // get to its value for example, if your re assign x like before, the pointer to x do not
    // change.

    // MEMORIES
    //  There's 2 main concept of memory. The Stack and the Heap
    //  You either allocate a value on the stack, or on the heap and get a pointer to it
}

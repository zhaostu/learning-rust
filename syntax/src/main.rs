use std::cell::Cell;

fn main() {
    variable_bindings();
    functions();
    primitive_types();
    comments();
    if_();
    loops();
    ownership();
    reference_and_borrowing();
    lifetimes();
    mutability();
    struct_();
    enums();
    match_();
    patterns();
    method_syntax();
    vectors();
    strings();
    generics();
    traits();
    drop();
    if_let();
    trait_objects();
}

fn variable_bindings() {
    // Variable bindings
    let x = 5;
    println!("{}", x);

    // Patterns
    let (x, y) = (2, 3);
    println!("{}, {}", x, y);

    // Type annotation
    let y: u32 = 10;
    println!("{}", y);

    // Mutability
    let mut x = 10;
    println!("x is {} before mutation", x);
    x = 5;
    println!("x is {} after mutation", x);

    // Initializing bindings
    // The following won't compile because x is not initialized.
    // let x: i32;
    // println!("{}", x);
    
    // Scope and shadowing
    {
        println!("x is {} inside the scope, before shadowing", x);
        let x: u16 = 121;
        println!("x is {} inside the scope", x);
    }
    println!("x is {} outside the scope", x);

    // Here the variable is no longer mutable
    let x = x;
    println!("x is {}", x);
}

fn functions() {
    addition(50, 100);
    println!("{} plus one is {}", 10, add_by_one(10));

    // Expression vs statements
    // expression returns a value, statements don't

    // This is an declaration statement
    // let x = 10;
    // The evaluation of "x = 10" is empty tuple ()
    // Expression statement turns express to statments.

    println!("{} is even? {}", 3, is_even(3));

    // Function pointers
    let f: fn(i32) -> i32 = add_by_one;

    println!("Using function pointers {}", f(10));
}

fn addition(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn add_by_one(x: i32) -> i32 {
    x + 1
}

fn is_even(i: i32) -> bool {
    if i % 2 == 0 {
        return true;
    }
    false
}

// Can be used as any type
fn diverges() -> ! {
    loop {};
}

fn primitive_types() {
    // Booleans
    let x: bool = false;
    println!("x is {}", x);

    // char, supports Unicode
    let x: char = 'A';
    println!("x is {}", x);

    // Numeric values
    // signed and fixed size
    let x: i8 = -12;
    println!("x is {}", x);
    // unsigned and fixed size
    let x: u8 = 12;
    println!("x is {}", x);
    // variable size (depending on underlying machine size)
    let x: usize = 1200;
    println!("x is {}", x);
    // floating point
    let x = 1.0;
    println!("x is {}", x);

    // Arrays (fixed sized)
    let a: [u8; 3] = [1, 2, 3];
    println!("a[0] is {}", a[0]);
    // shorthand initialization
    let a = [100; 20];
    println!("a[0] is {}", a[0]);
    println!("length of a is {}", a.len());

    // Slices
    let complete = &a[..];
    println!("length of complete is {}", complete.len());
    let some: &[u32] = &a[5..10];
    println!("length of some is {}", some.len());

    // str
    // Is an unsized type

    // Tuples
    // Ordered list of fixed size
    let mut x: (i32, &str) = (1, "hello");
    let y = (2, "hellos");
    x = y;
    // accessing values in tuple with destructuring let
    let (number, word) = x;
    println!("There are {} {}.", number, word);
    // single element tuple
    (0,);

    // Tuple indexing
    println!("There are {} {}.", x.0, x.1);
}

/// # Doc comments here
///
/// Markdown is supported
fn comments() {
    //! # This documents the containing element
    //! instead of the following element
    (0,); // This is a line comment.
}

fn if_() {
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    // if is an expression
    let y = if x == 6 { 10 } else { 20 };
    println!("y is {}", y);
}

fn loops() {
    // loop
    // indefinite loop
    loop {
        break;
    }

    // while
    let mut x = 0;
    while x < 2 {
        println!("x is now {} in while loop", x);
        x += 1;
    }

    // for
    // for var in expression { code }
    for x in 0..5 {
        println!("x is now {} in for loop", x);
    }

    // Enumerate
    for (i, j) in (5..10).enumerate() {
        println!("i is {} and j is {}", i, j);
    }

    // Ending iteration early
    // break
    let mut x = 5;
    loop {
        x += x - 3;
        println!("x is {}", x);
        if x % 5 == 0 { break; }
    }
    // continue
    for x in 0..10 {
        if x % 2 == 0 { continue; }
        println!("x is {}", x);
    }

    // Loop labels
    'outer: for x in 0..6 {
        'inner: for y in 0..6 {
            if x % 2 == 0 { continue 'outer };
            if y % 2 == 0 { continue 'inner };
            println!("x is {} and y is {}", x, y);
        }
    }
}

fn ownership() {
    //! using zero-cost abstractions
    //! ownership is a primary example of zero-cost abstractions
    //! all features talked about are done compile time
    // Vector v will be deallocated deterministically after it goes out of scope
    // even if it is allocated on the heap
    let v = vec![1, 2, 3];
    // The ownership is transferred to v2
    let v2 = v;
    // println!("v[0] is: {}", v[0]); // This will generate an error.
    
    take_ownership(v2);
    // println!("v2[0] is: {}", v2[0]); // This will also generate an error.

    // Copy types (trait)
    let v = 1;
    let v2 = v;
    println!("v is {} and v2 is {}", v, v2);
}

fn take_ownership(v: Vec<i32>) -> i32 {
    // nothing happens
    v[0]
}

fn reference_and_borrowing() {
    let v = vec![1, 2, 3];
    take_reference(&v);
    println!("v[0] is {}", v[0]);

    // Mutable reference
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("Mutable x is now {}", x);

    // The rules
    // Borrower scope should never lasts longer than the owner
    // You can only have one or the other type of borrows
    // Only one mutable reference is allowed
}

fn take_reference(v: &Vec<i32>) -> i32 {
    // v[1] = 1; // Reference (borrowed) are immutable.
    v[0]
}

fn lifetimes() {
    explicit_lifetime(&10);
    function_with_lifetime();
    let e = ExplicitLifetime { x: &5 };
    println!("{}", e.x());

    // Static lifetime for the entire program.
    let x: &'static str = "Hello World!";
    println!("{}", x);

    // Lifetime Elision
    // input lifetime: parameter
    // output lifetime: return value
    // elide input lifetime and use this to infer output lifetime
    // 1. each argument has distinct life time
    // 2. if only one input lifetime, same output lifetime.
    // 3. if multiple lifetime, but one is "self", lifetime of self for output
    // otherwise, fail.
}

fn explicit_lifetime<'a>(x: &'a i32) {
    println!("x is {}", x);
}

fn function_with_lifetime<'a, 'b>() {
}

// The ExplicitLifetime struct cannot outlive the x it contains
struct ExplicitLifetime<'a> {
    x: &'a i32,
}

impl<'a> ExplicitLifetime<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn mutability() {
    // mutable variable binding
    let mut x = 5;
    x = 6;
    println!("{}", x);

    // mutable reference
    let mut x = 5;
    let y = &mut x; // This is immutable reference.
    *y = 10;
    println!("y is {}", *y);

    let mut x = 5;
    let mut z = 10;
    let mut y = &mut x;
    y = &mut z;
    println!("y is {}", *y);

    // Interior vs exterior mutability
    // Field level mutability
    struct Point {
        x: i32,
        y: Cell<i32>,
    }

    let point = Point { x: 10, y: Cell::new(10) };
    point.y.set(11);
    println!("point is {}, {:?}", point.x, point.y);
}

fn struct_() {
    // Update syntax
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let a = Point3d { x: 1, y: 2, z: 3 };
    let b = Point3d { y: 10, .. a };
    println!("{}, {}, {}", a.x, a.y, a.z);
    println!("{}, {}, {}", a.x, b.y, b.z);

    // Tuple structs
    // better to use struct than tuple structs
    struct Color(i32, i32, i32);

    // "newtype" pattern
    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);

    // Unit-like struct
    struct Unit;
    let x = Unit;
}

fn enums() {
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move {x: i32, y: i32},
        Write(String),
    }

    let m1 = Message::Quit;
    let m2 = Message::Move {x: 10, y: 20};

    // Constructor as functions
    let m3 = Message::Write("Hello World!".to_string());
}

fn match_() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let number = match x {
        1 => "one",
        2 => "two",
        5 => "five",
        _ => "other",
    };

    // Matching on enums
    enum Message {
        Quit,
        Move {x: i32, y: i32},
    }

    let msg = Message::Move {x: 1, y: 2};
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move {x: x, y: y} => println!("Moving to {} {}", x, y),
    }
}

fn patterns() {
    let x = "x";
    let c = "c";
    match c {
        x => println!("x: {} c: {}", x, c),
    }
    println!("x: {}", x);

    // Multiple patterns
    match x {
        "x" | "y" => println!("is x or y"),
        _ => println!("not x or y"),
    }

    // Destrcturing
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x: x1, .. } => println!("({}, 0)", x1),
    }

    // Ignoring bindings
    // Use _ or .. inside the pattern
    
    // ref and ref mut
    let mut x = 5;

    match x {
        ref r => println!("Got a reference to {}", r),
    }

    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    // Ranges
    // Use ..., mostly used with integers and chars
    // Bindings
    // Use @
    match x {
        a @ 1 ... 2 | a @ 3 ... 5 => println!("one through five ({}).", a),
        6 ... 10 => println!("six through ten"),
        _ => println!("everything else"),
    }

    // Guards
    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);

    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck!"),
    }
}

fn method_syntax() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        // Associated function
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }

        fn grow(&self, increment: f64) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius + increment }
        }
    }

    let c = Circle::new(0.0, 0.0, 2.0);
    println!("area is {}", c.area());
    // Chaining method calls
    println!("c2's area is {}", c.grow(2.0).area());

    // Builder pattern
    struct CircleBuilder {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl CircleBuilder {
        fn new() -> CircleBuilder {
            CircleBuilder { x: 0.0, y: 0.0, radius: 1.0 }
        }

        fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.x = coordinate;
            self
        }

        fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.y = coordinate;
            self
        }

        fn radius(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.radius = coordinate;
            self
        }

        fn finalize(&self) -> Circle {
            Circle::new(self.x, self.y, self.radius)
        }
    }

    let c3 = CircleBuilder::new()
        .x(3.0)
        .radius(10.0)
        .finalize();

    println!("area is {}", c3.area());
}

fn vectors() {
    let v = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 10]; // ten zeros

    println!("the third element is {}", v[2]); // Index is usize type

    // Iterating
    for i in &v2 {
        println!("A reference to {}", i);
    }
}

fn strings() {
    // resizable, a sequence of utf-8 characters, not null terminated
    // &str is a string slice, has fixed size.
    let greeting = "Hello there."; // &'static str
    let s = "foo
        bar";
    let s2 = "foo\
        bar";
    println!("{}", s);
    println!("{}", s2);

    let mut s = "Hello".to_string(); // String
    s.push_str(", world.");
    println!("{}", s);

    // Indexing
    // because of utf-8 strings do not support indexing
    let name = "赵洋磊";
    for b in name.as_bytes() {
        print!("{} ", b);
    }
    println!("");
    for c in name.chars() {
        print!("{} ", c);
    }
    println!("");

    // Slicing
    // but for some reason you can do slicing
    let last_name = &name[0..3]; // byte offsets, has to end on character boundary
    println!("{}", last_name);

    // Concatenation
    let hello = "Hello ".to_string();
    let world = "world!";
    let hello_world = hello + world;
    println!("{}", hello_world);
}

fn generics() {
    enum MyOption<T> { // T is just convention.
        Some(T),
        None,
    }

    let x: MyOption<i32> = MyOption::Some(5);
    let y: MyOption<f64> = MyOption::Some(10.24);

    // Also can have mutliple types.
    // enum Result<A, Z> {}

    // Generic functions
    // fn takes_anything<T>(x: T) {
    // }

    // Generic structs
    // struct Point<T> {
    //      x: T,
    //      y: T,
    // }
    // impl<T> Point<T> {}
}

fn traits() {
    trait HasArea {
        fn area(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    struct Square {
        side: f64,
    }

    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    // Trait bounds on generic functions
    fn print_area<T: HasArea>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }

    let c = Circle { radius: 2.0 };
    let s = Square { side: 2.0 };
    print_area(c);
    print_area(s);

    // Trait bounds on generic structs
    struct Rectangle<T> {
        width: T,
        height: T,
    }

    impl<T: PartialEq> Rectangle<T> {
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }

    let r = Rectangle {
        width: 47,
        height: 47,
    };

    println!("This is a square? {}", r.is_square());

    // Rules for implementing traits
    impl HasArea for i32 {
        fn area(&self) -> f64 {
            *self as f64
        }
    }
    println!("silly {}", 5.area());
    // Two rules:
    // Traits has to be defined in your scope to apply (use "use")
    // Either the trait, or the type you're writing impl for, must be defined by you.

    // Mutliple bounds
    // use +
    // fn foo<T: Clone + Debug>(x: T) {}

    // Where clause
    // syntax sugar
    // fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {}
    // equals
    // fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {}

    // Default methods
    trait foo {
        fn is_valid(&self) -> bool;
        fn is_invalid(&self) -> bool { !self.is_valid() }
    }

    // Inheritance
    trait bar : foo {
        fn is_good(&self);
    }

    // Deriving
    #[derive(Debug)]
    struct Foo;
    println!("{:?}", Foo);
}

fn drop() {
    // A special trait from Rust standard library. When things goes out of scope.
    struct Firework {
        strength: i32,
    };

    impl Drop for Firework {
        fn drop(&mut self) {
            println!("Dropping {}!", self.strength);
        }
    }

    // First in, last out
    let small = Firework { strength: 1 };
    let big = Firework { strength: 100 };
}

fn if_let() {
    let option = Option::Some(5);

    if let Some(x) = option {
        println!("{}", x);
    } else {
        println!("Nothing");
    }

    let mut v = vec![1, 3, 5, 7, 11];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

fn trait_objects() {
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }

    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    let x = 5u8;
    let y = "Hello".to_string();
    
    // Static dispatching
    fn do_something<T: Foo>(x: T) {
        println!("{}", x.method());
    }
    do_something(x);
    do_something(y);

    // Trait objects can store any type that implement the trait.
    // obtained by casting or coercing
    // Dynamic dispatching
    fn do_something_else(x: &Foo) {
        println!("{}", x.method());
    }
    do_something_else(&x);
}

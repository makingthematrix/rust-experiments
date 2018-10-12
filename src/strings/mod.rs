//! A string examples module

use std;
use std::thread;
use std::fmt;

pub fn strings() {
    strings1()
}

/// ## Format
///
/// ```rust
///
/// ```
fn strings1() {
    println!("Hello, world 1!")
}

fn strings2() {
    let hello = "Hello, world 2!";
    println!("{}", hello);
}

fn strings3() {
    let hello = String::from("Hello, world 3!");
    println!("{}", hello);
}

fn strings4() {
    let mut hello = String::new();
    hello.push_str("Hello, world 4!");
    println!("{}", hello);
}

fn strings5() {
    let hello: Vec<char> = "Hello, world 5!".chars().collect();
    println!("{:?}", &hello);
}

fn strings6() {
    let hello: Vec<char> = "Hello, world 6!".chars().collect();
    for i in 0 .. hello.len() {
        print!("{}", hello[i]);
    }
    println!()
}

fn strings7() {
    let hello: Vec<char> = "Hello, world 7!".chars().collect();
    let mut it = hello.iter();
    loop {
        match it.next() {
            Some(c) => print!("{}", c),
            None => break,
        }
    }
    println!()
}

fn strings8() {
    let hello: Vec<char> = "Hello, world 8!".chars().collect();
    let mut it = hello.iter();
    let mut end = false;
    while !end {
        print!("{}", it.next().unwrap_or_else(|| { end = true; &'\n' }));
    }
}

fn strings9() {
    let hello: Vec<char> = "Hello, world 9!".chars().collect();
    let mut it = hello.iter();
    while let Some(c) = it.next() { print!("{}", c); }
    println!()
}

fn strings10() {
    let hello: Vec<char> = "Hello, world 10!".chars().collect();
    for c in hello.iter() {
        print!("{}", c);
    }
    println!()
}

fn strings11() {
    let hello: Vec<char> = "Hello, world 11!".chars().collect();
    hello.iter().for_each(|c| print!("{}", c));
    println!()
}

fn strings12() {
    let hello = || { println!("Hello, world 12!") };
    hello()
}

fn strings13() {
    let hello = || { println!("Hello, world 13!") };
    let handle = thread::spawn(hello);
    handle.join();
}

fn strings14() {
    struct Hello(&'static str);

    let hello = Hello("Hello, world 14!");
    println!("{}", hello.0)
}


fn strings15() {
    struct Hello(&'static str);

    impl fmt::Display for Hello {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let hello = Hello("Hello, world 15!");
    println!("{}", hello)
}

fn strings16() {
    struct Hello(&'static str);

    impl From<&'static str> for Hello {
        fn from(s: &'static str) -> Self {
            Hello(s)
        }
    }

    let hello: Hello = "Hello, world 16!".into();
    println!("{}", hello.0)
}

fn strings17() {
    struct Hello {
        h: String,
        w: String
    }

    impl Hello {
        pub fn new(hh: &'static str, ww: &'static str) -> Self {
            Hello { h: String::from(hh), w: String::from(ww) }
        }

        fn join(&self) -> String {
            format!("{}, {} 17!", self.h, self.w).to_owned()
        }
    }

    let hello = Hello::new("Hello", "world");
    println!("{}", hello.join());
}

fn strings18() {
    let hello = vec!["Hello", "world 18!"];
    println!("{}", hello.join(", "));
}

fn strings19() {
    let hello = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 32, 49, 57, 33];
    println!("{}", String::from_utf8_lossy(&hello));
}

fn strings20() {
    let hello = vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 32, 50, 48, 33];
    println!("{}", String::from_utf8(hello).unwrap());
}

fn strings21() {
    let hello_bytes: [u8; 16] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 32, 50, 49, 33];
    let hello_vec: Vec<char> = hello_bytes.iter().map(|b| *b as char).collect();
    let hello_str: String = hello_vec.iter().collect();
    println!("{}", hello_str);
}

fn strings22() {
    struct Hello;
    impl Drop for Hello { // this is called when a Hello instance is deallocated
        fn drop(&mut self) {
            println!("Hello, world 22!");
        }
    }

    let _hello = Hello;
}

/*
Now with enums:
enum Hello {
    H,
    W { x: i32, y: i32 },
}

with pretty debug:
println!("{:#?}", hello); // where hello implements Debug

With result:
fn function_that_may_fail() -> Result<(), String> { ... }

fn hello() -> Result<String, String> {
    function_that_may_fail()?; // if fails, it will propagate its Error<String> upwards
    Ok("Hello") // if the function above doesn't fail, it will return Ok<String>
}


special enum:
enum MyErr {
    Failed,
    ReallyFailed,
    OMGWhyyyy,

    #[doc(hidden)]
    __hidden // this means that even if a match covers the above three cases, it's still not exhaustive, so it needs to cover default
}

and then:
let err = MyErr::...
match err {
    Failed =>
    ReallyFailed =>
    OMGWhyyy =>
    _ => ... // default required

// match can also unapply cases
// use `..` as a wildcard for fields in the unapplied struct you're not interested in
struct A { x: usize, y: usize }
struct B { a: A }

let a = A { x: 1, y: 2 }
let b = B { a: a }

match b {
    B { A { x: 5, .. } } => ...
    _ =>
}

slices:
let mut data = vec!['a', 'b', 'c'];
let slice = &mut data[..]; // creates a "window" into data
data.push('d'); // that actually should work, but it doesn't right now, bcs `slice` sees only ['a', 'b', 'c'], so adding a new element doesn't touch it

AsRef:
fn accept_either<S>(s: S) where S: AsRef<S> { // accepts both String and &str
}


Course:
spacekookie/rust-three-days-course, presentation (English),

Iterators can cycle!
iter.cycle()
Learn about IntoIter

struct Point { x: usize, y: usize }

impl Point {
    fn set_x(self, x: usize) -> Self {
        Self { x, ..self }
    }
}

let p1 = Point { 1, 1 };
let p2 = p1.set_x(2); // consumes p1 and creates p2 (but the compile may optimize it out, and return p1 with changed x )
                      // good for implementing builders

pub(crate) Point { ... } // visible in the crate, but not outside


*/
/*
A trait object with dynamic dispatch (what's that?)

trait Distance {
    fn distance(&self, other: &dyn Distance) -> f64; // normally that's not possible, bcs the size of `Distance` is not known at compile-time
}

https://alschwalm.com/blog/static/2017/03/07/exploring-dynamic-dispatch-in-rust/

Learn about Rust futures with generics, and impl Trait.
Not possible to have impl Trait in trait methods. (what does it mean?)
Find minimal examples on the internet

clap-rs - a crate for parsing command line arguments

Macros: "A Little Book of Macros"

cargo expand // displays the code after macro expansions

what's Send and Sync?

Rustonomicon - a book about Rust unsafe code


thread_local! macro should be interesting


James Munn's "Tiny Rocket" article about optimizing binary sizes

There are bindings to Raspberry Pi, but not yet to Arduino :(
https://becominghuman.ai/
ESP8266 is another microchip
and read about NodeMCU

Futures:
docs.rs/crate/futures/0.1.25
usually used together with tokio
futures work by themselves only from nightly, but you can have them on stable as a part of a library (tokio?)

Tests:
proptest is a test library which may replace or help quickcheck
*/
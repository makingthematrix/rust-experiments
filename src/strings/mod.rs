use std;
use std::thread;
use std::fmt;

pub fn strings() {
    strings20()
}

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
    for c in hello.iter() {
        print!("{}", c);
    }
    println!()
}

fn strings10() {
    let hello: Vec<char> = "Hello, world 10!".chars().collect();
    hello.iter().for_each(|c| print!("{}", c));
    println!()
}

fn strings11() {
    let hello = || { println!("Hello, world 11!") };
    hello()
}

fn strings12() {
    let hello = || { println!("Hello, world 12!") };
    let handle = thread::spawn(hello);
    handle.join();
}

fn strings13() {
    struct Hello(&'static str);

    let hello = Hello("Hello, world 13!");
    println!("{}", hello.0)
}


fn strings14() {
    struct Hello(&'static str);

    impl fmt::Display for Hello {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let hello = Hello("Hello, world 14!");
    println!("{}", hello)
}

fn strings15() {
    struct Hello(&'static str);

    impl From<&'static str> for Hello {
        fn from(s: &'static str) -> Self {
            Hello(s)
        }
    }

    let hello: Hello = "Hello, world 15!".into();
    println!("{}", hello.0)
}

fn strings16() {
    struct Hello {
        h: String,
        w: String
    }

    impl Hello {
        pub fn new(hh: &'static str, ww: &'static str) -> Self {
            Hello { h: String::from(hh), w: String::from(ww) }
        }

        fn join(&self) -> String {
            format!("{}, {} 16!", self.h, self.w).to_owned()
        }
    }

    let hello = Hello::new("Hello", "world");
    println!("{}", hello.join());
}

fn strings17() {
    let hello = vec!["Hello", "world 17!"];
    println!("{}", hello.join(", "));
}

fn strings18() {
    let hello = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 32, 49, 56, 33];
    println!("{}", String::from_utf8_lossy(&hello));
}

fn strings19() {
    let hello = vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 32, 49, 57, 33];
    println!("{}", String::from_utf8(hello).unwrap());
}

fn strings20() {
    let hello_bytes: [u8; 16] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 32, 50, 48, 33];
    let hello_vec: Vec<char> = hello_bytes.iter().map(|b| *b as char).collect();
    let hello_str: String = hello_vec.iter().collect();
    println!("{}", hello_str);
}
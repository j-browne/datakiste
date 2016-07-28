mod my {
    struct T {
        a: i32,
    }

    pub struct S {
        t: T,
        b: f64,
    }

    impl S {
        pub fn new(a: i32, b: f64) -> S {
            S{t: T {a: a}, b: b}
        }
    }

    pub mod mymy {
        use super::S;
        pub fn print(s1: S, s2: S) {
            println!("{} {}\t{} {}", s1.t.a, s1.b, s2.t.a, s2.b);
        }
    }
}

fn main() {
    let s1 = my::S::new(1, 1.0);
    let s2 = my::S::new(2, 2.0);
    my::mymy::print(s1, s2);
}

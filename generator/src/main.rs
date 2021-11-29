#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};

fn main() {
    let xs = vec![1, 2, 3];
    let gen = || {
        let mut sum = 0;
        println!("Hello");
        for v in xs.iter() {
            println!("{}", v);
            sum += v;
            yield sum;
        }
        for v in xs.iter().rev() {
            println!("{}", v);
            sum -= v;
            yield sum;
        }
        println!("world!");
    };

    let mut pinned = Box::pin(gen);
    loop {
        match pinned.as_mut().resume(()) {
            GeneratorState::Yielded(n) => println!("Got value {}", n),
            GeneratorState::Complete(_) => break,
        }
    }

    let gen2 = static || {
        let to_borrow = String::from("Hello");
        let borrowed = &to_borrow;

        yield borrowed.len();

        println!("{} world!", borrowed);
    };

    // while let Some(val) = gen2.next() {
    //     println!("{}", val);
    // }
    let mut pinned2 = Box::pin(gen2);
    if let GeneratorState::Yielded(n) = pinned2.as_mut().resume(()) {
        println!("Gen2 got value {}", n);
    };

    let _ = pinned2.as_mut().resume(());
}

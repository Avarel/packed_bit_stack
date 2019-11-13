mod lib;

use lib::PackedBitStack;

fn main() {
    let mut stack = PackedBitStack::new();
    println!("{}", std::mem::size_of::<PackedBitStack>());
    stack.push_u128(5000000000123213122348024730572349053);
    println!("{}", stack.pop_u128());
    
}

#[test]
fn store_string_on_heap() {
    let mut stack = PackedBitStack::new();

    unsafe {
        stack.push_heap(String::from("string lol!"));
        assert_eq!("string lol!", stack.pop_heap::<&str>());
    }
}

#[test]
fn store_simple_bits_on_heap() {
    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Test {
        a: u8,
        b: usize,
    }

    let a = Test { a: 50, b: 700 };
    let b = a.clone();

    let mut stack = PackedBitStack::new();

    unsafe {
        stack.push_bits(a);
        assert_eq!(b, stack.pop_bits::<Test>())
    }
}
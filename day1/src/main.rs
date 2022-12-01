#![feature(binary_heap_into_iter_sorted)] // Can only be used with Rust nightly
use std::fs;
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    let contents = fs::read_to_string("input.txt").unwrap();
    let arr = contents
        .split("\n\n")
        .collect::<Vec<&str>>();

    for elf in arr {
        let cals: i32 = elf
            .split("\n")
            .map(|val| val.parse::<i32>().unwrap_or(0))
            .sum();
        heap.push(cals);
    }
    println!("{}", heap.peek().unwrap());

    /*
     * Uncomment for use with Rust stable
     *
    let mut top3 = 0;
    for _ in 0..3 {
        top3 += heap.pop().unwrap();
    }
    println!("{}", top3);
    */

    // Can only be used with Rust nightly
    let top3: i32 = heap
        .into_iter_sorted()
        .take(3)
        .sum();

    println!("{}", top3);
}

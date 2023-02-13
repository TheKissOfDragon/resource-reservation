use std::{fmt::Debug, io};
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() -> io::Result<()> {
    let mut arr: [i32; 3] = [1, 4, 3];
    let a: u32 = 32;
    bubble_sort(&mut arr);
    iter(&mut arr);
    let closure = |x: u32| x + a;
    let sum: u32 = add(closure, THREE_HOURS_IN_SECONDS);
    println!("sum:{sum:?}");
    Ok(())
}

fn add<T>(t: T, b: u32) -> u32
where
    T: Fn(u32) -> u32,
{
    t(b)
}
fn iter<T: Ord + Debug>(arr: &mut [T]) {
    for i in arr {
        println!("{i:?}");
    }
}
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

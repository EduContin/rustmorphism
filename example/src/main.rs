use polymorphic_fn::polymorphic_fn;

// Small implementation helper
#[inline(never)]
fn small_impl(x: i32) -> i32 {
    println!("Small implementation selected");
    x + 1
}

// Large implementation helper with a massive static array
#[inline(never)]
fn large_impl(x: i32) -> i32 {
    static LARGE_DATA: [u8; 10_000_000] = [42; 10_000_000]; // 10 million bytes
    let sum: u64 = LARGE_DATA.iter().map(|&b| b as u64).sum();
    println!("Large implementation selected, sum: {}", sum);
    x + 1
}

polymorphic_fn! {
    pub fn my_function(x: i32) -> i32 {
        { small_impl(x) },
        { large_impl(x) }
    }
}

fn main() {
    let input = 5;
    println!("Result: {}", my_function(input));
}
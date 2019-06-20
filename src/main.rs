use fasthash::*;

fn main() {
    let h1 = city::hash64("hello world");
    let h2 = metro::hash64_with_seed("hello world", 123);
    println!("{:?} {:?}", h1, h2);
}

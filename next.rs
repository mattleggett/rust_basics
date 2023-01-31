#[derive(Debug)]
struct UnPrintable(i32);


fn main() {
    println!("hi there {}", "Matt");

    println!("{:?}", UnPrintable(1));
}
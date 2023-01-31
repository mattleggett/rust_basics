fn main() {
    println!("hi there {}", "Matt");

    #[derive(Debug)]
    struct UnPrintable(i32);

    // println!("{}", UnPrintable)
}
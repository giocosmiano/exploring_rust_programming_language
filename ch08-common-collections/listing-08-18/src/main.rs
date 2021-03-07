use test::bench::run_once;

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    print!("{}, {}", s2, s3);
    // print!("{}, {}, {}", s1, s2, s3);
}

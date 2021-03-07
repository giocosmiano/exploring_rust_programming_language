fn main() {
    let v = vec![1, 2, 3];

    let maybe_value = v.get(99);
    match maybe_value {
        Some(val) => println!("{}", val),
        None => println!("nothing to see here")
    }

    v[99];
}

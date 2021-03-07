fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    let maybe_value = map.get(&field_name);
    match maybe_value {
        Some(d) => println!("{} {}", field_name, d),
        None => println!("{} is empty", field_name)
    }
}
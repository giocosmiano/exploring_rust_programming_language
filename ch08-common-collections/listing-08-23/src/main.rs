fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let maybe_score = scores.get(&team_name);

    match maybe_score {
        Some(score) => println!("{} {}", team_name, score),
        None => println!("{} is empty", team_name)
    }

    let team_name = String::from("Green");
    let maybe_score = scores.get(&team_name);

    match maybe_score {
        Some(score) => println!("{} {}", team_name, score),
        None => println!("{} is empty", team_name)
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

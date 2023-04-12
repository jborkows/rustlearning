use std::collections::HashMap;
// has to be imported not like vector
pub fn start_fun() {
    create_map();
    create_map_from_tupples();
    adding_strings_move_ownership();
    calculate_word_in_text();
    //for maps one can define his own hasher
}

fn create_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn create_map_from_tupples() {
    let teams = vec![String::from("Blue"), String::from("Green")];
    let initial_scores = vec![10, 50];
    let _scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

fn adding_strings_move_ownership() {
    let field_name = String::from("A");
    let field_value = String::from("a letter");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name);//both value and key ownership is moved to map
    // println!("{}", field_value);
}
fn calculate_word_in_text() {
    let text = "Ala ma kota. Kot ma Alę";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count);
}

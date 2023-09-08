pub fn inspect(value: &String) {
    if value.ends_with("s") {
        println!("Plural {}", value)
    } else {
        println!("Singular {}", value)
    }
}

pub fn change(value: &mut String) {
    if value.ends_with("s") {
        println!("Plural {}", value)
    } else {
        value.push_str("s");
    }
}

pub fn eat(value: String) -> bool {
    if value.starts_with('b') && value.contains("a") {
        true
    } else {
        false
    }
}

pub fn bedazzle(value: &mut String) {
    *value = String::from("sparkly")
}

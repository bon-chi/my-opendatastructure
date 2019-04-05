use std::collections::HashMap;

fn main() {
    let map: HashMap<char, char> = [('}', '{'), (')', '('), (']', '[')]
        .iter()
        .cloned()
        .collect();
    let pattern = String::from("{}");
    println!("{} is {}", pattern, is_matching(&pattern, &map));
    let pattern = String::from("{)");
    println!("{} is {}", pattern, is_matching(&pattern, &map));
    let pattern = String::from("{()}");
    println!("{} is {}", pattern, is_matching(&pattern, &map));
    let pattern = String::from("{{{{{{{{(([[]]))}}}}}}}}");
    println!("{} is {}", pattern, is_matching(&pattern, &map));
    let pattern = String::from("{()()[]{()}}");
    println!("{} is {}", pattern, is_matching(&pattern, &map));
    let pattern = String::from("[()}");
    println!("{} is {}", pattern, is_matching(&pattern, &map));
}

fn is_matching(string: &String, map: &HashMap<char, char>) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match map.get(&c) {
            Some(close_parent) => {
                if let Some(poped) = stack.pop() {
                    if !(poped == *close_parent) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            None => {
                stack.push(c);
            }
        }
    }
    true
}

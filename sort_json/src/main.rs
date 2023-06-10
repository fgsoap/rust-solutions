use serde_json::Value;

fn main() {
    let json_str = r#"[{"name": "Alice", "age": 25}, {"name": "Bob", "age": 30}, {"name": "Charlie", "age": 20}, {"name": "Anna", "age": 30}]"#;
    let mut json: Vec<Value> = serde_json::from_str(json_str).unwrap();
    json.sort_by(|a, b| {
        let a_age = a["age"].as_i64().unwrap_or(std::i64::MAX);
        let b_age = b["age"].as_i64().unwrap_or(std::i64::MAX);
        let age_cmp = a_age.cmp(&b_age);
        if age_cmp == std::cmp::Ordering::Equal {
            let a_name = a["name"].as_str().unwrap_or("");
            let b_name = b["name"].as_str().unwrap_or("");
            a_name.cmp(b_name)
        } else {
            age_cmp
        }
    });
    let sorted_json_str = serde_json::to_string(&json).unwrap();
    println!("{}", sorted_json_str);
}

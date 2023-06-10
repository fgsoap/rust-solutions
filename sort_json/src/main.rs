use serde_json::Value;

fn main() {
    let json_str =
        r#"[{"name": "Alice", "age": 25}, {"name": "Bob", "age": 30}, {"name": "Charlie"}]"#;
    let mut json: Vec<Value> = serde_json::from_str(json_str).unwrap();
    json.sort_by(|a, b| {
        let a_key = a["age"].as_i64().unwrap_or(std::i64::MAX);
        let b_key = b["age"].as_i64().unwrap_or(std::i64::MAX);
        a_key.cmp(&b_key)
    });
    let sorted_json_str = serde_json::to_string(&json).unwrap();
    println!("{}", sorted_json_str);
}

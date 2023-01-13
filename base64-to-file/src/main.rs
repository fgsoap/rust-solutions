use base64::{engine::general_purpose, Engine as _};

#[derive(Debug)]
struct S;

fn main() {
    println!("{:?}", S);

    let kind = infer::get_from_path("./bm")
        .expect("file read successfully")
        .expect("file type is known");

    println!("{:?}", kind.mime_type());

    let bytes = std::fs::read("./123").unwrap();
    println!("{:#?}", bytes);
    // for byte in bytes.iter() {
    //     println!("{:b}", byte);
    // }

    let orig = b"data";
    println!("{:?}", orig);
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(orig);
    assert_eq!("ZGF0YQ", encoded);
    assert_eq!(
        orig.as_slice(),
        &general_purpose::STANDARD_NO_PAD.decode(encoded).unwrap()
    );

    // or, URL-safe
    let encoded_url = general_purpose::URL_SAFE_NO_PAD.encode(orig);
    println!("{}", encoded_url);

    let encoded = "SGVsbG8gV29ybGQh"; // Hello World!
    let decoded = general_purpose::STANDARD_NO_PAD.decode(encoded).unwrap();
    let decoded_str = std::str::from_utf8(&decoded).unwrap();
    println!("Decoded: {}", decoded_str);
}

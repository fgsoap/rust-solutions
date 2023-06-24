// use std::rc::Rc;

// use std::error::Error;
// use std::fs::File;

use regex::Regex;
// use std::io::{self, ErrorKind, Read};

// use ndarray::array;
// use std::collections::VecDeque;
// use std::{collections::BinaryHeap, path::Component};
#[allow(unused_imports)]

fn main() {
    // let mut array: [i32; 4] = [1, 2, 3, 4];
    // // println!("{:#?}", &array[0..array.len()]);
    // array.reverse();
    // for i in &array {
    //     println!("{}", i);
    // }

    // let mut stack = vec![1, 2, 3];
    // stack.push(4);
    // stack.pop();
    // println!("{:#?}", &stack);
    // stack.push(5);
    // println!("{:#?}", &stack);

    // let mut queue = VecDeque::new();
    // queue.push_back(1);
    // queue.push_front(2);
    // queue.pop_back();
    // queue.pop_front();
    // queue.push_back(3);
    // println!("{:#?}", &queue);

    // let vertices = array![1, 2, 3, 4, 5];
    // let edges = array![
    //     [0, 1, 1, 1, 1],
    //     [1, 0, 0, 1, 0],
    //     [1, 0, 0, 0, 1],
    //     [1, 1, 0, 0, 1],
    //     [1, 0, 1, 1, 0]
    // ];
    // println!("{:#?} {:#?}", &vertices, &edges);

    // let names = vec!["Jack", "Luck", "Tom"];
    // fn hash(id: usize) -> usize {
    //     (id - 1) % 10000
    // }
    // let inx = hash(10002);
    // println!("{:?}", inx);

    // println!("{:#?}", names[inx]);

    // Type inference lets us omit an explicit type signature (which
    // would be `BinaryHeap<i32>` in this example).
    // let mut heap = BinaryHeap::new();

    // // We can use peek to look at the next item in the heap. In this case,
    // // there's no items in there yet so we get None.
    // assert_eq!(heap.peek(), None);

    // // Let's add some scores...
    // heap.push(1);
    // heap.push(2);
    // heap.push(5);

    // // Now peek shows the most important item in the heap.
    // assert_eq!(heap.peek(), Some(&5));

    // // We can check the length of a heap.
    // assert_eq!(heap.len(), 3);

    // // We can iterate over the items in the heap, although they are returned in
    // // a random order.
    // for x in &heap {
    //     println!("{x}");
    // }

    // // If we instead pop these scores, they should come back in order.
    // assert_eq!(heap.pop(), Some(5));
    // assert_eq!(heap.pop(), Some(2));
    // assert_eq!(heap.pop(), Some(1));
    // assert_eq!(heap.pop(), None);

    // // We can clear the heap of any remaining items.
    // heap.clear();

    // // The heap should now be empty.
    // assert!(heap.is_empty());

    // struct Struct {
    //     e: i32,
    // }
    // let (a, b, c, d, e);
    // (a, b) = (1, 2);
    // [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct { e, .. } = Struct { e: 5 };
    // println!("{:#?}", (a, b, c, d, e));

    // let guess: i32 = "42".parse().expect("Not a number!");
    // println!("{:#?}", guess);
    // println!("{:#?}", 2 & 3);

    // let bytes = std::fs::read("./123.txt").unwrap();
    // println!("{:#?}", bytes);
    // // for byte in bytes.iter() {
    // //     println!("{:b}", byte);
    // // }

    // #[derive(Debug)]
    // enum UiObject {
    //     Button,
    //     SelectBox,
    // }

    // let objects = [UiObject::Button, UiObject::SelectBox];

    // for o in objects {
    //     draw(o)
    // }

    // fn draw(o: UiObject) {
    //     println!("{:?}", o);
    // }

    // pub trait Draw {
    //     fn draw(&self);
    // }

    // pub struct Button {
    //     pub width: u32,
    //     pub height: u32,
    //     pub label: String,
    // }

    // impl Draw for Button {
    //     fn draw(&self) {
    //         println!("{:?} {:?} {:?}", self.width, self.height, self.label)
    //     }
    // }

    // struct SelectBox {
    //     width: u32,
    //     height: u32,
    //     options: Vec<String>,
    // }

    // impl Draw for SelectBox {
    //     fn draw(&self) {
    //         println!("{:?} {:?} {:?}", self.width, self.height, self.options)
    //     }
    // }

    // impl Draw for String {
    //     fn draw(&self) {
    //         println!("{:?} {:?}", self.len(), self.bytes())
    //     }
    // }

    // pub struct Screen {
    //     pub components: Vec<Box<dyn Draw>>,
    // }

    // impl Screen {
    //     pub fn run(&self) {
    //         for component in self.components.iter() {
    //             component.draw();
    //         }
    //     }
    // }

    // let screen = Screen {
    //     components: vec![
    //         Box::new(SelectBox {
    //             width: 75,
    //             height: 10,
    //             options: vec![
    //                 String::from("Yes"),
    //                 String::from("Maybe"),
    //                 String::from("No"),
    //             ],
    //         }),
    //         Box::new(Button {
    //             width: 50,
    //             height: 10,
    //             label: String::from("OK"),
    //         }),
    //         Box::new(String::from("Hello World!")),
    //     ],
    // };

    // screen.run();

    // let mut v = vec![1, 2];
    // v.push(3);
    // println!("{:?}", v);

    // let _third = &v[2];
    // // match v.get(2) {
    // //     Some(third) => println!("{}", third),
    // //     None => println!("None!"),
    // // }
    // v.push(56);

    // for i in &mut v {
    //     *i += 10
    // }
    // for i in &v {
    //     println!("{:?}", i);
    // }

    // // HashMap
    // use std::collections::HashMap;
    // let mut scores = HashMap::new();
    // scores.insert("Blue", 10);

    // let old = scores.insert("Blue", 20);
    // // println!("old: {:?}, scores: {:?}", old, scores);
    // assert_eq!(old, Some(10));

    // let new = scores.get("Blue");
    // assert_eq!(new, Some(&20));

    // let v = scores.entry("Yellow").or_insert(5);
    // assert_eq!(v, &5);

    // let v = scores.entry("Yellow").or_insert(50);
    // assert_eq!(*v, 5);

    // let text = "hello world wonderful world";
    // let mut map = HashMap::new();
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    // println!("{:?}", &map)

    // let a = 3.1_f32 as i8;
    // let b = 100_i8 as i32;
    // let c = 'a' as i16;

    // println!("{},{},{}", a, b, c)

    // let mut values: [i32; 2] = [1, 2];
    // let p1: *mut i32 = values.as_mut_ptr();
    // println!("p1: {:?}", p1);
    // let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    // println!("first_adress: {:?}", first_address);
    // let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    // println!("second_address: {:?}", second_address);
    // let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    // unsafe {
    //     *p2 += 1;
    // }
    // assert_eq!(values[1], 3);

    // use std::convert::TryInto;

    // let a: u8 = 10;
    // let b: u16 = 256;

    // let b_: u8 = match b.try_into() {
    //     Ok(b1) => b1,
    //     Err(e) => {
    //         println!("{:?}", e.to_string());
    //         0
    //     }
    // };
    // println!("a: {}, b_: {}, u8::Max: {}", a, b_, u8::MAX);

    // if a < b_ {
    //     println!("a is less than b_.")
    // }

    // struct T1 {
    //     x: u32,
    //     y: u16,
    // }

    // #[allow(dead_code)]
    // #[derive(Debug)]
    // struct T2 {
    //     a: u32,
    //     b: u16,
    // }

    // fn reinterpret(t1: T1) -> T2 {
    //     let (x, y) = (t1.x, t1.y);
    //     // let T1 { x, y } = t1;
    //     T2 { a: x, b: y }
    // }

    // let a = T1 { x: 12, y: 13 };

    // let b = reinterpret(a);

    // println!("{:?}", b);

    // trait Trait {}
    // fn foo<X: Trait + std::fmt::Debug>(t: X) {
    //     println!("{:?}", t)
    // }
    // impl Trait for i32 {}

    // let t = 12;
    // foo(t);

    // let array: Rc<Box<[T; 3]>> = ...;
    // let first_entry = array[0];

    // panic!("crash and burn");

    // let f = File::open("hello.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut s = String::new();
    //     File::open("hello.txt")?.read_to_string(&mut s)?;
    //     Ok(s)
    // }
    // let f = read_username_from_file();
    // println!("{:?}", f);

    // fn _test_() -> Result<(), Box<dyn Error>> {
    //     let _f = File::open("hello.txt")?;
    //     Ok(())
    // }

    fn parse_csv_line(line: &str) -> Vec<String> {
        let re = Regex::new(r#""([^"]*)"|([^,"]+)"#).unwrap();
        re.captures_iter(line)
            .map(|cap| {
                cap.get(1)
                    .or_else(|| cap.get(2))
                    .unwrap()
                    .as_str()
                    .to_owned()
            })
            .collect()
    }
    let line = r#""John, Doe",42,"123, Main St.""#;
    let fields = parse_csv_line(line);
    println!("{:?}", fields);
}

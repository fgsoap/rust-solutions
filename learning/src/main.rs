// use ndarray::array;
// use std::collections::VecDeque;
// use std::{collections::BinaryHeap, path::Component};

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

    let mut v = vec![1, 2];
    v.push(3);
    println!("{:?}", v);

    let _third = &v[2];
    // match v.get(2) {
    //     Some(third) => println!("{}", third),
    //     None => println!("None!"),
    // }
    v.push(56);

    for i in &mut v {
        *i += 10
    }
    for i in &v {
        println!("{:?}", i);
    }
}

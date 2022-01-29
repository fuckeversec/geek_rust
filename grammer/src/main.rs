use std::borrow::Borrow;
use std::rc::Rc;
use std::sync::Arc;
use std::vec;

mod strtok_example;

#[derive(Debug)]
struct Node {
    id: u64,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn main() {
    // println!("Hello, world!");
    // let mut data = vec![1, 2, 3];

    // for item in data.iter_mut() {
    // data.push(*item + 1);
    // }

    // example_1();
    let a = Rc::new(1);
    let b = a.clone();
    let c = a.clone();
    println!("{} {} {}", a, b, c);

    let node1 = Node::new(1);
    let node2 = Node::new(2);

    println!("node1: {:?}, node2: {:?}", node1, node2);

    let s = Arc::new("rust rocks!");
    let s1 = s.clone();
    let handler = std::thread::spawn(move || {
        println!("{:?}", s1);
    });

    println!("main: {:?}", s);
    handler.join().unwrap();
}

// pub fn example_1() {
//     // 可变 vec使用方法
//     // https://doc.rust-lang.org/std/vec/index.html
//     let mut data = vec![1, 2, 3];

//     // 不可变
//     let data1 = vec![&data[0]];

//     println!("data[0]: {:p}", &data[0]);

//     for i in 0..100 {
//         // 不可变引用可变元素
//         data.push(i);
//     }

//     println!("data[0]: {:p}", &data[0]);
//     println!("boxed: {:p}", &data1);
// }

use std::cmp;
use std::io::{self, BufRead};
use std::collections::HashSet;
use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Hash, Eq, PartialEq, Debug)]
struct Rectangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

impl Rectangle {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Rectangle { x1: x1, y1: y1, x2: x2, y2: y2 }
    }
    pub fn area(self) -> i32 {
        (self.x2 - self.x1) * (self.y2 - self.y1)
    }
}

enum QuadNodeValue {
    Str(&'static str),
    Pivot(i32, i32)
}

struct QuadNode {
    value: QuadNodeValue,
    q1: Option<Box<QuadNode>>,
    q2: Option<Box<QuadNode>>,
    q3: Option<Box<QuadNode>>,
    q4: Option<Box<QuadNode>>
}

impl QuadNode {
    fn new(
        val: QuadNodeValue,
        q1: Option<Box<QuadNode>>,
        q2: Option<Box<QuadNode>>,
        q3: Option<Box<QuadNode>>,
        q4: Option<Box<QuadNode>>
    ) -> Self {
        QuadNode { value: val, q1: q1, q2: q2, q3: q3, q4: q4 }
    }
}


fn traverse_sum(node: QuadNode, lbx: i32, lby: i32, rbx: i32, rby: i32) -> i32 {
    match node.value {
        QuadNodeValue::Str(s) => {
            return match s {
                "empty" => 0,
                "covered" => Rectangle::area(Rectangle::new(lbx, lby, rbx, rby)),
                _ => 0
            }
        },
        QuadNodeValue::Pivot(x, y) => {
            let a1 = traverse_sum(*node.q1.unwrap(), lbx, lby, x, y);
            let a2 = traverse_sum(*node.q2.unwrap(), lbx, y, lbx, lby);
            let a3 = traverse_sum(*node.q3.unwrap(), x, lby, lbx, lby);
            let a4 = traverse_sum(*node.q4.unwrap(), x, y, lbx, lby);

            return a1 + a2 + a3 + a4;
        }
    }
}


fn add_rect(rect: Rectangle, mut node: QuadNode) -> QuadNode {
    let x1 = rect.x1;
    let y1 = rect.y1;
    let x2 = rect.x2;
    let y2 = rect.y2;

    if Rectangle::area(rect) <= 0 {
        return node
    }
    
    match node.value {
        QuadNodeValue::Str(s) => {
            return match s {
                "empty" => QuadNode::new(
                    QuadNodeValue::Pivot(x1, y1),
                    Some(Box::new(QuadNode::new(QuadNodeValue::Str("empty"), None, None, None, None))),
                    Some(Box::new(QuadNode::new(QuadNodeValue::Str("empty"), None, None, None, None))),
                    Some(Box::new(QuadNode::new(QuadNodeValue::Str("empty"), None, None, None, None))),
                    Some(Box::new(QuadNode::new(
                        QuadNodeValue::Pivot(x2, y2),
                        Some(Box::new(QuadNode::new(QuadNodeValue::Str("covered"), None, None, None, None))),
                        Some(Box::new(QuadNode::new(QuadNodeValue::Str("empty"), None, None, None, None))),
                        Some(Box::new(QuadNode::new(QuadNodeValue::Str("empty"), None, None, None, None))),
                        Some(Box::new(QuadNode::new(QuadNodeValue::Str("empty"), None, None, None, None)))
                    )))
                ),
                "covered" => node,
                _ => node
            }
        },
        QuadNodeValue::Pivot(x, y) => {
            node.q1 = Some(Box::new(add_rect(Rectangle::new(cmp::min(x, x1), cmp::min(y, y1), cmp::min(x, x2), cmp::min(y, y2)), *node.q1.unwrap())));
            node.q2 = Some(Box::new(add_rect(Rectangle::new(cmp::max(x, x1), cmp::min(y, y1), cmp::max(x, x2), cmp::min(y, y2)), *node.q2.unwrap())));
            node.q3 = Some(Box::new(add_rect(Rectangle::new(cmp::min(x, x1), cmp::max(y, y1), cmp::min(x, x2), cmp::max(y, y2)), *node.q3.unwrap())));
            node.q4 = Some(Box::new(add_rect(Rectangle::new(cmp::max(x, x1), cmp::max(y, y1), cmp::max(x, x2), cmp::max(y, y2)), *node.q4.unwrap())));
            
            return node
        }
    }
}

fn rectangle_coverage(rectangles: Vec<Rectangle>) -> i32 {

    let mut root = QuadNode::new(QuadNodeValue::Str("empty"), None, None, None, None);
    for rec in rectangles {
        root = add_rect(rec, root);
    }
    return traverse_sum(root, -1000000, -1000000, 1000000, 1000000);
}

fn main() {
    let input = io::stdin();
    let mut lines = input.lock().lines();
    let mut hash_rect = HashSet::<Rectangle>::new();

    while let Some(line) = lines.next() {
        let length: i32 = line.unwrap().trim().parse().unwrap();
        for _ in 0..length {
            let line = lines
                .next()
                .expect("there was no next line")
                .expect("the line could not be read");

            let mut split = line.split_whitespace();
            let x1: i32 = split.next().unwrap().parse().unwrap();
            let y1: i32 = split.next().unwrap().parse().unwrap();
            let x2: i32 = split.next().unwrap().parse().unwrap();
            let y2: i32 = split.next().unwrap().parse().unwrap();
    
            let r = Rectangle::new(x1, y1, x2, y2);
            hash_rect.insert(r);
        }
    }
    let mut rectangles: Vec<Rectangle> = hash_rect.into_iter().collect();
    rectangles.shuffle(&mut thread_rng());

    // let r1 = Rectangle { x1: 1, y1: 1, x2: 4, y2: 4 };
    // let r2 = Rectangle { x1: 0, y1: 0, x2: 3, y2: 3 };
    // let rectangles = vec![r1, r2];
    let area = rectangle_coverage(rectangles);
    println!("{}", area);
}

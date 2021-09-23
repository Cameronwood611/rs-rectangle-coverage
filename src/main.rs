
struct Rectangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

enum QuadNodeValue {
    Str(&'static str),
    Pivot((i32, i32))
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


fn add_rectangle(rect: Rectangle, node: QuadNode) -> QuadNode {
    node
}


fn rectangle_coverage(rectangles: Vec<Rectangle>) -> i32 {

    let mut root = QuadNode::new(QuadNodeValue::Str("empty"), None, None, None, None);
    for rec in rectangles {
        root = add_rectangle(rec, root);
    }
    3
}


fn main() {
    let r1 = Rectangle { x1: 1, y1: 1, x2: 4, y2: 4 };
    let r2 = Rectangle { x1: 0, y1: 0, x2: 3, y2: 3 };
    let rectangles = vec![r1, r2];
    let area = rectangle_coverage(rectangles);
    println!("Total area covered: {}", area);
}

use std::{char, collections::HashMap};

#[derive(Debug)]
struct Node {
    ch: Option<char>,
    freq: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn frequency(s: &str) -> HashMap<char, i32> {
    let mut hashmap = HashMap::new();
    for ch in s.chars() {
        let counter = hashmap.entry(ch).or_insert(0);
        *counter += 1;
    }
    println!("{:?}", hashmap);
    hashmap
}

fn new_node(freq: i32, ch: Option<char>) -> Node {
    Node {
        freq: freq,
        ch: ch,
        left: None,
        right: None,
    }
}

fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

fn assign_code(p: &Box<Node>, h: &mut HashMap<char, String>, s: String) {
    if let Some(ch) = p.ch {
        h.insert(ch, s);
    } else {
        if let Some(ref l) = p.left {
            assign_code(l, h, s.clone() + "0");
        }
        if let Some(ref r) = p.right {
            assign_code(r, h, s.clone() + "1");
        }
    }
}
fn encode_string(s: &str, h: &HashMap<char, String>) -> String {
    let mut r = "".to_string();
    let mut t: Option<&String>;

    println!("{:?}", h);
    for ch in s.chars() {
        t = h.get(&ch);
        r.push_str(t.unwrap());
    }
    r
}

fn decode_string(s: &str, root: &Box<Node>) -> String {
    let mut retval = "".to_string();
    let mut nodeptr = root;

    for x in s.chars() {
        if x == '0' {
            if let Some(ref l) = nodeptr.left {
                nodeptr = l;
            }
        } else {
            if let Some(ref r) = nodeptr.right {
                nodeptr = r;
            }
        }
        if let Some(ch) = nodeptr.ch {
            retval.push(ch);
            nodeptr = root;
        }
    }
    retval
}

fn main() {
    let msg = "Huffman coding is fun!";
    let f = frequency(msg);

    let mut p: Vec<Box<Node>> = f
        .iter()
        .map(|x| new_box(new_node(*(x.1), Some(*(x.0)))))
        .collect();

    while p.len() > 1 {
        p.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
        let a = p.pop().unwrap();
        let b = p.pop().unwrap();
        let mut c = new_box(new_node(a.freq + b.freq, None));
        c.left = Some(a);
        c.right = Some(b);
        p.push(c);
    }

    let root = p.pop().unwrap();
    let mut h: HashMap<char, String> = HashMap::new();
    assign_code(&root, &mut h, "".to_string());

    let enc = encode_string(msg, &h);
    println!("{}", enc);
    println!("decoded = {:?}", decode_string(&enc, &root));
}

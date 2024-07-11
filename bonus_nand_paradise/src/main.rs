mod ast;
use ast::node::Node as Node;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input>", args[0]);
        return;
    }
    let input = &args[1];
    let tree = Node::parse(input);
    if let Err(e) = tree {
        println!("Error: {}", e);
        return;
    }
    let tree : Option<Rc<RefCell<Node>>> = tree.unwrap();
    println!("{}", tree.unwrap().borrow().to_nand_paradise());
}

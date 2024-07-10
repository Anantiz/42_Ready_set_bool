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
    println!("Input:    {}", input);
    let tree : Option<Rc<RefCell<Node>>> = tree.unwrap();
    if let Some(tree) = tree.clone() {
        println!("Parsed:   {}", tree.borrow().to_rpn());
    } else {
        println!("Empty tree");
    }
    let tree : Rc<RefCell<Node>> = tree.unwrap();

    let cnf = tree.borrow().to_cnf();
    if let Some(cnf) = cnf {
        println!("\nCNF form: {}", cnf.borrow().to_rpn());
    } else {
        println!("Empty CNF");
    }
}

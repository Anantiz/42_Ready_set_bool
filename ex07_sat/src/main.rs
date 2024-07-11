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
        println!("Infix:    {}", tree.borrow());
    } else {
        println!("Empty tree");
        return;
    }
    println!("");


    let tree : Rc<RefCell<Node>> = tree.unwrap();

    print!("Original Input: ");
    let sat = tree.borrow_mut().dumbfuck_sat();
    match sat {
        Err(s) => println!("Error: {}", s),
        Ok(b) => println!("Satisfiable: {}", b)
    }

    print!("SAT using CNF:  ");
    let cnf = tree.borrow().to_cnf();
    if cnf.is_none() {
        println!("Error: Could not convert to CNF");
        return;
    }
    let sat = tree.borrow_mut().dumbfuck_sat();
    match sat {
        Err(s) => println!("Error: {}", s),
        Ok(b) => println!("Satisfiable: {}", b)
    }
}

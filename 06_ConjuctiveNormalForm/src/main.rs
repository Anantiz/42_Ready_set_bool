use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;

mod ast;

fn main()
{
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input>", args[0]);
        return;
    }
    let mut input : String = String::new();
    for i in 1..args.len() {
        input = args[i].clone();
    }
    if input.len() == 0 {
        println!("Invalid input");
        return;
    }

    let tree : Result<Option<Rc<RefCell<ast::AstNode>>>, String> = ast::AstNode::rpn_to_ast(&input);
    match tree {
        Ok(tree) => {
            match tree {
                Some(tree_rc_ref) => {
                    // Why Borrowing an Rc<A<B>> returns a Ref<B> -> Sensless behavior
                    let tree : Ref<ast::AstNode> = tree_rc_ref.borrow();
                    // Why clone() in Ref<B> returns a B -> Sensless behavior
                    let tree : ast::AstNode = (*tree).clone();
                    // Can Rust even be coherent with itself for more than 2 lines?

                    println!("Original:                {:?}", tree);
                    let negated = ast::AstNode::to_negation_normal_form(tree);
                    if negated.is_none() {
                        println!("Unknown Error");
                        return;
                    }
                    let negated = negated.unwrap();
                    let negated = negated.borrow();
                    let cnf = ast::AstNode::to_cnf((*negated).clone());
                    println!("Printing CNF");
                    println!("Conjuctive Normal form:  {:?}", *cnf.unwrap().borrow());
                },
                None => println!("Invalid input")
            }
        },
        Err(e) => println!("Error: {}", e)
    }
}
#[path ="./ast.rs"]
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

    let tree : Result<Option<Box<ast::AstNode>>, String> = ast::AstNode::rpn_to_ast(&input);
    match tree {
        Ok(tree) => {
            match tree {
                Some(tree) => {
                    println!("Original:                {:?}", tree);
                    let negated = ast::AstNode::to_negation_normal_form(*tree);
                    if negated.is_none() {
                        println!("Unknown Error");
                        return;
                    }
                    let cnf = ast::AstNode::to_cnf(*negated.unwrap());
                    println!("Conjuctive Normal form:  {:?}", cnf.unwrap());
                },
                None => println!("Invalid input")
            }
        },
        Err(e) => println!("Error: {}", e)
    }
}
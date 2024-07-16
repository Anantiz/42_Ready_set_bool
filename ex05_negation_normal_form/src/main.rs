#[path ="./ast.rs"]
mod ast;

//Warper to satisfy exact subject requirements
fn negation_normal_form(formula: &str) -> String {

    let tree : Result<Option<Box<ast::AstNode>>, String> = ast::AstNode::rpn_to_ast(&formula);
    match tree {
        Ok(tree) => {
            match tree {
                Some(tree) => {
                    println!("Original: {:?}", tree);
                    let negated = ast::AstNode::to_negation_normal_form(*tree);
                    let str_rpn = negated.unwrap().to_rpn();
                    println!("Negated:  {:?}", str_rpn);
                    return str_rpn;
                },
                None => {
                    println!("Invalid input");
                    return String::from("");
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            return String::from("");
        }
    }
}

fn main()
{
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input>", args[0]);
        return;
    }
    let input = args[1].clone();
    if input.len() == 0 {
        println!("Invalid input");
        return;
    }
    _ = negation_normal_form(&input);


}
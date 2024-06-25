// Basically We want to distribute the negations of an expression
// !(A & B) --> !A | !B

// But in polish form
// AB&! --> A! B! |

// Best way to do it is an AST
/*
          !
        /
       &
      /  \
     A    B

Then Distribute the tree to remove `Operator negations`
Meaning, a negation whose child is an operator, for this
    > Take the oposite of the operator (AND -> OR, OR -> AND)
    > Negate the children
        > If the childrens are operators, distribute them again recursively
*/
mod ast;

fn main() {
    let input = "AB&!"; // Example input in polish notation
    let tree = ast::parse_polish_expression(input);
    println!("Original AST: {:?}", tree);

    let distributed_ast = tree.negation_normal_form();
    println!("Distributed AST: {:?}", distributed_ast);

    let output = distributed_ast.to_polish_notation();
    println!("Output Polish Notation: {}", output);
}

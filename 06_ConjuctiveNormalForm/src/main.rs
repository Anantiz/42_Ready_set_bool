mod ast;
#[path = "tables/maxterms.rs"]
mod maxterms;

fn main()
{
    // let input = "ABCD&|&";
    let input = "AB&";
    let cnf = conjunctive_normal_form(input);
    println!("{}", input);
    println!("{}", cnf);
}

fn join_asts(asts : Vec<Box<ast::Expr>>) -> ast::Expr
{
    let mut root : Box<ast::Expr> = Box::new(ast::Expr::Or{
        left : Box::new(ast::Expr::Var('H')),
        right : Box::new(ast::Expr::Var('H'))});

    let mut current = root.as_mut();
    let mut len: usize = asts.len();

    for ast in asts.iter()
    {
        if len == 1
        {
            *current = *ast.clone();
            break;
        }
        if let ast::Expr::Or{left, right} = &mut *current
        {
            *left = ast.clone();
            *right = Box::new(ast::Expr::Or{
                left : Box::new(ast::Expr::Var('H')),
                right : Box::new(ast::Expr::Var('H'))
            });
        }
        if let ast::Expr::Or{right, ..} = current
        {
            current = right.as_mut();
        }
        len = len - 1;
    }
    return *root;
}

fn maxterm_to_ast(maxterm : Vec<(bool, bool)>, mut literal_count : u32) -> Box<ast::Expr>
{

    let mut root : Box<ast::Expr> = Box::new(ast::Expr::And{
        left : Box::new(ast::Expr::Var('H')),
        right : Box::new(ast::Expr::Var('H'))});

    let mut c : u8 = b'A';
    let mut current = root.as_mut();

    for literal in maxterm.iter()
    {
        if literal.0
        {
            if literal_count == 1
            {
                if let ast::Expr::And{right, ..} = &mut *current
                {
                    *right = insert_literal(c as char , literal.1);
                }
                break;
            }
            literal_count -= 1;
            if let ast::Expr::And{left, right} = &mut *current
            {
                *left = insert_literal(c as char , literal.1);
                *right = Box::new(ast::Expr::And{
                    left : Box::new(ast::Expr::Var('H')),
                    right : Box::new(ast::Expr::Var('H'))
                });
            }

            if let ast::Expr::And{right, ..} = current
            {
                current = right.as_mut();
            }
        }
        c += 1; // Map from A-Z
    }

    fn insert_literal(c : char, value : bool) -> Box<ast::Expr>
    {
        if value
        {
            return Box::new(ast::Expr::Var(c));
        }
        else
        {
            return Box::new(ast::Expr::Not(Box::new(ast::Expr::Var(c))));
        }
    }

    return root;
}

fn count_literal(maxterm : Vec<(bool, bool)>) -> u32
{
    let mut count = 0;
    for literal in maxterm.iter()
    {
        if literal.0
        {
            count += 1;
        }
    }
    count
}

fn maxterms_to_function(maxterms : Vec<Vec<(bool, bool)>>) -> ast::Expr
{
    let mut asts : Vec<Box<ast::Expr>> = Vec::new();

    for maxterm in maxterms.iter()
    {
        let literal_count = count_literal(maxterm.clone());
        let ast = maxterm_to_ast(maxterm.clone(), literal_count);
        asts.push(ast);
    }
    return join_asts(asts);
}

fn conjunctive_normal_form(formula: &str) -> String
{
    let terms = maxterms::get_maxterms(formula);
    let ast = maxterms_to_function(terms);
    let cnf = ast.invert_terms();
    return cnf.to_polish_notation()
}

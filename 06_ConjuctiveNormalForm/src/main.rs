/*
Goal: Convert into Conjuctive Normal Form (CNF)

Example:

In RPN:

    ABCD&|& => ABC|BD|&&

As an AST:

      &                          &
    /   \                      /   \
    A    |             =>     A     &
       /   \                      /   \
      B      &                   |     |
            / \                 / \   / \
           C   D               B   C  B  D


Method:
    Make a formula Out of the Maxterms
    Then Negate inverse the Terms
    Tada! CNF

1. Isolate the maxterms of the input
2. Make a formula out of the maxterms
3  Negate the formula
4. Tada! CNF

*/

mod ast;
mod maxterms;

fn main()
{
    let input = "ABCD&|&";

}


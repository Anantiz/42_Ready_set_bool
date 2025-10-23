## Ready Set Bool 🦀 

from 42school Math branch

### Preface:

This project was a battle to keep my `sanity` for the following reasons:
- Understanding what the heck is up with `Tseytin`
- I learned that `Moser-de-brujin` sequence exists, and I now believe in `Witchcraft`
- Just `rust` being `rust`

### Description

Various exercises to discover Boolean-algebra, the world of formal-proofs and other discrete math concepts.

I chose `rust` to learn about it hands-on with a concrete project instead of doing tutorials.

### Features:

- Parses RPN boolean expression into an AST fit for mathematical modeling (ex03 to ex08)
- Ex06: An overly complex CNF method. (I was not yet tired: *ex06_conjuctive_normal_form/src/ast/to_cnf.rs*)
- Ex07: A naive SAT Solver. (I was tired: *ex07_sat/src/ast/sat.rs*)
- Witchcraft, Moser and de-Bruijn were surely not drinking just water.

### Usage:

Requirements: [rustc, cargo, rustup].

Ex00 to ex03 are simple rust files, use: `rustc <filename>`
Afterwards they are all cargo projects, run `cargo run` at the root of each exercise.

### Technical outline:

Ex00-Ex01: Implement an adder and multiplier only using bit-wise operations

Ex02: A function that converts bits to gray-code

Ex03-Ex07: Progressive exercise starting from "execute a boolean operation with an RPN stack" to reach "Convert expression to normal forms and implement a SAT solver"

Ex08-Ex09: Generate power-sets and apply unions/intersects etc... on sets

Ex10-Ex11: Space-filling curve and their inverse function: Making you wonder if you even know what is real or not

Bonus: The universe is NAND-complete

### Skills & Lesson learned:

- Do not make ASTs in rust.
- Lambda calculus & Advanced Boolean Algebra.
- Handling fast-dense learning of several unknown advanced subjects at once.

### Notes:

***The rust experience***
- `self.left.clone().unwrap().borrow();`

- `Result<Option<Rc<RefCell<Node>>>, String>`

___

**Useful links:**

- [Lambda Calculus Course](https://www.youtube.com/watch?v=ViPNHMSUcog) - Visual introduction
- [Boolean Algebra PDF](https://www.csie.ntu.edu.tw/~lyuu/complexity/2006/20061011.pdf) - Comprehensive theory
- [Tseytin Transformation](https://en.wikipedia.org/wiki/Tseytin_transformation) - For clever CNF conversion
- [Moser-de Bruijn Sequence](https://en.wikipedia.org/wiki/Moser%E2%80%93de_Bruijn_sequence) - Questioning reality

**Note for 42 students**: The project evaluation sheet doesn't allow Tseytin transformation for CNFs as it wants you to keep the same truth table instead of just having it to be equisatisfiable. Imo it's a short-coming of the subject for ignoring the whole concept of equisatisfiabilty.

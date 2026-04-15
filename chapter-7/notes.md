## Packages, Crates, Modules, Use

- Package is the project we make using cargo new.
- There are two type of crates. Binary and Library.
- Root of binary crate is the src/main.rs, whereas for library crate, it is src/lib.rs
- To make modules, we use `mod` keyword. Using `pub` with it or its members will give public access.
- Sibling modules can access eachother, but not their private members. To give access, use `pub` to all the child members.
- There are 2 ways to define the module path. Absolute and Relative path.
- Absolute path starts with `crate` keyword as root all the way till the module we want to use, where :: is used to seperate the modules. Absolute path is preferred.
- Relative path will see the current parent mod.
- If mod A is parent of mod B and we want to access mod A fn from mod B, use super:: . its like using .. in file system to represent directory before.
- We choose to define private fields in Struct in mod. If there are any private fields, we cannot directly create its object ourselves. Will need a method impl to create it.
- For enums, the access is only set to whole enum, not to individual fields.
- `use` only creates shortcut for particular scope. if use is in the files root and there is a module, the scope of use is not valid inside the mod. in this case, we can use `pub use` to bring it to other scopes too.
- we can give custom name to imported modules like `use std::io::Result as IoResult;`
- if there are conflicting imports and no custom name, we will have to specify which module to use, using their parent like `parent::child1` & `parent::child2`
- to import multiple modules from same crate or mod, we can use curly braces. like `use std::{cmp::Ordering, io};`
- when we want to use a parent and a child itself but want to do it in 1 line, we can import the parent by placing self inside the bracket. example: `use std::io::{self, Write};`
- to bring all the items defined in path, use *. `use std::collections::*;`

### declaring modules and file seperation
When we declare a module in a file, like `mod garden;` in lib.rs, compiler will try to find a file with same name as garden in same directory as lib to find its implementation.
But if we declare another module in a module, like `mod vegetables;` inside garden.rs, this time- compiler will try to find vegetables.rs inside garden/ directory. if we wrote vegetables.rs in same directory as garden, then it means that vegetables is independant of garden. so when it is inside garden, it will contain inside the appropriate folder
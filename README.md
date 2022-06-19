# tw3-witcherscript-superset
Transpiler for the WitcherScript superset `WitcherScriptSuperset`

The goal of this project is to offer a basic compiler (more of a transpiler) that takes as input `.wss` files and converts them into `.ws` files.
This allows the use of new features available to `wss` that get converted to valid witcherscript code.

## Main features & goals
 - [ ] Generics, with mangled names to allow use of `wss` libraries
 - [ ] Macros, conditional compilation, support for recursive macros (macros that generate calls to macros)
 - [ ] Constant primitive variables in the global scope (macro constants)
 - [ ] Lambdas
 - [ ] Closures
 - [ ] Variable declarations anywhere in function bodies
 - [ ] some forms of static analysis, or at least syntax validation
 - [ ] namespaces and import statements

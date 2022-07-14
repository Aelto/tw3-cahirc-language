# tw3-witcherscript-superset
Transpiler for the WitcherScript superset `WitcherScriptSuperset`

The goal of this project is to offer a basic compiler (more of a transpiler) that takes as input `.wss` files and converts them into `.ws` files.
This allows the use of new features available to `wss` that get converted to valid witcherscript code.

## Main features & goals
 - [x] Generics, with mangled names to allow use of `wss` libraries
 - [ ] Macros, conditional compilation, support for recursive macros (macros that generate calls to macros)
 - [ ] For..in loops
 - [ ] Constant primitive variables in the global scope (macro constants)
 - [ ] Lambdas
 - [ ] Closures
 - [ ] Variable declarations anywhere in function bodies
 - [ ] some forms of static analysis, or at least syntax validation
 - [ ] namespaces and import statements

# Using it
The compiler requires a config fil to be able to compile any project, here is a
basic configuration file: `cahirc.toml`
```toml
[package]
name = "my-awesome-project" 

# the source directory that contains your `.wss` files
src = "src" 

# the output directory where the compiler will emit the `.ws` code.
# WARNING: the directory is cleared at the start of every compilation
dist = "dist" 

# You can copy the following lines to add new dependencie
# [dependencies]
# example = "./example-lib"
```

once you have the configuration file placed at the root of your project, the
following command will compile your code:
```
cahirc
```

## The syntax
The syntax of the cahirc language is the exact same as the WitcherScript language
with a few additions.

```js
// a basic witcherscript program
function main() {
  var i: int;

  for (i = 0; i < 5; i += 1) {
    // ...
  }
}
```

### Generics
To define a generic function/class you can use the `<T>` annotation right behind
the type's name.

```js
function add<T>(a: T, b: T): T {
  return a + b;
}

class Counter<T> {
  var value: T;

  function set(value: T) {
    this.value = value;
  }
}
```

where `T` can be replaced by any letter or word, and where you can have multiple words
separated by commas for multiple types like so: `<Type1, Type2>`

### Macros

#### Compile time constants
```js
#define const A_CONSTANT = "Hello world!";

function main() {
  print(A_CONSTANT!);
}
```

As you may notice, the syntax for defining a macro is similar to the C syntax.
However, the syntax for using one is different, it requires the extra symbol `!`
behind the name of the macro constants.

This is done for simplicity while implementing the compiler, but it also improves
readability as you quickly know what is a local variable vs what is a global macro
constant.

#### Conditional compilation (not supported yet)
```js
#define const DEBUG = true;

function log(message: string) {
#if DEBUG
  print(message);
#endif
}
```

#### Macro functions
```js
#define function FOREACH(list, type, body) {
  var i: int;

  for (i = 0; i < list.Size(); i += 1) {
    var child: type = list[i];

    body
  }
};

function main() {
  var my_list: array<string> = { "foo", "bar", "foobar" };
  var sum: string;

  FOREACH!(my_list, string, {{
    print(child);

    sum += child;
  }});

  print(sum);
}
```
will expand into:
```js
function main() {
  var my_list: array<string> = { "foo", "bar", "foobar" };
  var sum: string;

  var i: int;

  for (i = 0; i < my_list.Size(); i += 1) {
    var child: string = list[i];

    print(child);

    sum += child;
  }

  print(sum);
}
```
---
As you may notice the `!` symbol that is required for macro constants is also required for macro functions. You may also notice you do not need to write any type, the preprocessor will do a simple
find/replace with the identifiers without checking anything. The code emitted by your macro may be invalid and the pre-processor will not emit any error.

The second important detail is how you are able to pass a variable, an identifier `string`, but also a whole piece of code `{{ ... }}`. The pre-processor treats this parameter as any other parameter.

---

Recursive macros are also possible:
```js
#define const DEBUG = true;

// a macro that generates a basic if DEBUG condition,
// so the supplied `code` is run only if DEBUG is true.
#define function IF_DEBUG(code) {
  #if DEBUG
  code
  #endif
};

// a macro that expands into a print call, but only
// if DEBUG is true.
#define function PRINT(message) {
  IF_DEBUG!({
    print(message);
  })
};

function main() {
  PRINT!("Program is in debug");
}
```
The pre-processor will continue to expand macro calls until none of them are found in the program anymore.
> **Warning**: It is not the compiler's duty to detect infinite
> recursivity in the macro functions you write. If such a
> thing were to happen, the program would never stop growing
> until it runs out of memory.

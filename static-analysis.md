# static analysis
the concept of static analysis is relatively simple, but its implementation requires some organising. It must be done in multiple passes:
- first we register all of the static types such as Compound types (classes and structs) and functions
- then for each function we infer and store the types of the local variables. Either by understanding the expressions, if it's a string, or if it's an integer or a float; Or by finding the function calls and getting the return types of the functions.
- after all of the local variables are known, we start analysing the functions calls by verifying the parameters' types match with the function definition.
- and finally once a function is fully analysed and deemed valid, we clear the data to reduce the memory footprint of the compiler


# useful articles
 - https://blog.polybdenum.com/2020/08/15/subtype-inference-by-example-part-7-spanned-error-messages.html for example usage of `Spanned`
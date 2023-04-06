## More Rust - Guessing Game

`use std::io;` : Brings the input/output library into scope from the standard `std` library.


`let apples = 5;` Declares an **immutable** variable (a variable where we cannot change the value) - Rust variables are immutable by default.

`let mut bananas = 5;` Declares a **mutable** variable, specifically declared with the `mut` keyword before the variable name.

Three things in `let mut guess = String::new()`:
1. String is a string type provided by the standard library
2. The `::` syntax indicates that `new` is associated to the type before it - in this case `String`.
3. The `new()` function creates a new object, in this case a new `String` type.



`io::stdin()` - calls the `stdin` function from the previously imported library.
We then call the `read_line` method using `.read_line(&mut guess)`, this takes the input from the user in `stdin` and convert it to a string.
The ampersand `&` indicates that the argument is a *reference* to the variable, which references its exact place in memory rather than storing the data in memory itself. Also note we have to reference `mut` again since variables are by default immutable.

The `read_line` method passes the user input into a string, however it also returns a `Result` type value: these can be either `Ok` or `Err` - fairly self explanatory what they do - `Ok` means the operation was successful, whereas `Err` not so much, and displays information as to why it might have failed.

`Result` type values have their respective specific methods, and in our case we see we have the `expect` method - it will display the parameter passed into `expect` when the result is an `Err`.


##### Crate
A crate is a collection of Rust source code files.

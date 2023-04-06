fn main() {
    println!("Hello world!");
    println!("Test 2");
    // The `{}` will be replaced by an argument
    println!("{} testing", "testing");
    // We can take positional arguments by specifying an integer inside of `{}`
    println!("Mountain: {0},{1},{2},{1},{0}","lowest", "middle", "highest");
    // We can do the same with named arguments:
    println!("Object: {object}, text: {text}",
            object = "Object text",
            text = "text text");

    // We can specify the 'width' of the text
    println!("5 Width test: {numbertest:>5}", numbertest = 2);
    // We can also use named arguments in the format by adding in a `$`:
    println!("name argument format: {number:0>width$}", number = 25, width=6);

    //FIXME Before:
    // println!("My name is {0}, {1} {0}", "Bond");
    // FIXME After:
    println!("My name is {0}, {1} {0}", "Bond", "James");

    
    // #[allow(dead_code)] // disable `dead_code` which warn against unused module
    // struct Structure(i32);
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // print pi to 3 decimal places.
    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}");

}
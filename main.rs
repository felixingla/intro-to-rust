fn main() {

    println!("");
    println!("###################");
    println!("### HELLO WORLD ###");
    println!("###################");
    println!("");

        // In general, the `{}` will be automatically replaced with any
        // arguments. These will be stringified.
        println!("{} days", 31);

        // Positional arguments can be used. Specifying an integer inside `{}`
        // determines which additional argument will be replaced. Arguments start
        // at 0 immediately after the format string
        println!("{0}, this is {1}. {1}, this is {0}", "Felix", "Frankie");

        // As can named arguments.
        println!("{subject} {verb} {object} and {verb2}",
                object="cereal",
                subject="felix",
                verb="eats",
                verb2="sleeps");

        // Different formatting can be invoked by specifying the format character after a
        // `:`.
        println!("Base 10 repr:               {}",   69420);
        println!("Base 2 (binary) repr:       {:b}", 69420);
        println!("Base 8 (octal) repr:        {:o}", 69420);
        println!("Base 16 (hexadecimal) repr: {:x}", 69420);
        println!("Base 16 (hexadecimal) repr: {:X}", 69420);
        
        // You can right-align text with a specified width. This will output
        // "    1". 4 white spaces and a "1", for a total width of 5.
        println!("Felix is {number:>24} years old", number="24");

        // You can pad numbers with extra zeroes.4 This will output "00001".
        println!("{number:0>24} years old", number=24);

        // You can use named arguments in the format specifier by appending a `$`
        println!("{number:0>width$}", number=24, width=100);
        

        // Rust even checks to make sure the correct number of arguments are
        // used.
        println!("My name is {0}, {1} {0}", "Felix", "Ingla");
        // FIXME ^ Add the missing argument: "James"

        // Only types that implement fmt::Display can be formatted with `{}`. User-
        // defined types do not implement fmt::Display by default

        #[allow(dead_code)]
        struct Structure(i32);

        // This will not compile because `Structure` does not implement
        // fmt::Display
        //println!("This struct `{}` won't print...", Structure(3));
        // TODO ^ Try uncommenting this line

        // For Rust 1.58 and above, you can directly capture the argument from a
        // surrounding variable. Just like the above, this will output
        // "     1". 5 white spaces and a "1".
        let number: f64 = 2.0;
        let width: usize = 10;
        println!("{number:>width$}");
        

        // Add a println! macro call that prints: 
        //Pi is roughly 3.142 by controlling the number of decimal places shown. 
        //For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. 
        //(Hint: you may need to check the std::fmt documentation for setting the number 
        //of decimals to display)

        let pi = 3.141592;
        let width: usize = 3;
        println!("Pi is roughly {pi:.width$}");
    

}

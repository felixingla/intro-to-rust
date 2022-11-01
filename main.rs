fn main() {


        println!("");
        println!("1, Nov 2022");
        
        println!("");println!("");

                /* 
                Another way to add to String objects together is 
                using a macro function called format. 
                The use of Format! is as shown below.
                */

                let n1 = "Tutorials".to_string();
                let n2 = "Point".to_string();
                let n3 = format!("{} {}",n1,n2);
                println!("{}",n3);

        
        println!("");println!("");
                
                /*
                A string value can be appended to another string. 
                This is called concatenation or interpolation. 
                The result of string concatenation is a new string object. 
                The + operator internally uses an add method. 
                The syntax of the add function takes two parameters. 
                The first parameter is self – the string object itself and the second parameter is a reference of the second string object. This is shown below −
                */

                let name = "Felix".to_string();
                let last_name = "Ingla".to_string();
                let full_name = name + &last_name;
                println!("{}", full_name);



        println!("");println!("");

                let n1 = "Tutorials".to_string();
                let mut i = 1;
                for n in n1.chars(){
                        println!("{}: {}", i, n);
                        i += 1;
                }

        println!("");println!("");

                //The split() string method returns an iterator over 
                //substrings of a string slice, separated by characters 
                //matched by a pattern. The limitation of the split() method 
                //is that the result cannot be stored for later use. 
                //The collect method can be used to store the result 
                //returned by split() as a vector.

                let fullname = "Kannan,Sudhakaran,Tutorialspoint";
                let mut i = 1;

                for token in fullname.split(","){
                        println!("{}: token is {}",i,token);
                        i += 1;
                }
        
                //store in a Vector
                println!("\n");
                let tokens:Vec<&str> = fullname.split(",").collect();
                println!("firstName is {}",tokens[0]);
                println!("lastname is {}",tokens[1]);
                println!("company is {}",tokens[2]);

        println!("");println!("");

                let msg = "Tutorials Point has good tutorials".to_string();
                let mut i = 1;

                for token in msg.split_whitespace(){
                println!("{}: {}", i ,token);
                i = i + 1;
                }

        println!("");println!("");

                //The push_str() function appends a given 
                //string slice onto the end of a String.
                let mut company = "Tutorial".to_string();
                company.push_str("s for beginners");
                println!("{}",company);

        println!("");println!("");

                //To access all methods of String object, 
                //convert a string literal to object type 
                //using the to_string() function.
                let new_string_2 = "Hello".to_string();
                println!("{}", new_string_2);
                //Replace function
                let new_string_3 = new_string_2.replace("Hello", "Good morning");
                println!("{}", new_string_3);
                
        
        println!("");println!("");
        
                // An empty string object is created using the new() method 
                // and its value is set to hello:
                let mut new_string = String::new();
                new_string.push_str("hello");
                println!("{}", new_string);


        println!("");println!("");

                let empty_string = String::new();
                println!("length is {}", empty_string.len());

                let content_string = String::from("TutorialsPoint");
                println!("length is {}", content_string.len());


        println!("");println!("");

                let company:&'static str = "TutorialsPoint";
                let location:&'static str = "Hyderabad";
                println!("company is : {} location :{}",company,location);

/*

        println!("");
        println!("31, Oct 2022");
        
        println!("");println!("");

                let uname = "Felix";
                let uname = uname.len();
                println!("Felix has : {} letters", uname);
        
        println!("");println!("");

                let mut variable = 200;
                println!("The value is: {}", variable);
                variable = 100;
                println!("The value is now: {}", variable);

        println!("");println!("");

                let isfun:bool = true;
                println!("Is Rust Programming Fun ? {}",isfun);

        println!("");println!("");

                let float_with_separator = 11_000.555_001_0_1;
                println!("float value {}",float_with_separator);
                
                let int_with_separator = 50_0_0_0;
                println!("int value {}",int_with_separator);


        println!("");println!("");

        
                let result = 10.020;        //f64 by default
                let interest:f32 = 8.35;
                let cost:f64 = 15000.600;  //double precision
                
                println!("result value is {}", result) ;
                println!("interest is {}",interest);
                println!("cost is {}",cost);
        
        println!("");println!("");

                let age:u8 = 255;

                // 0 to 255 only allowed for u8
                let weight:u8 = 255;   //overflow value is 0
                let height:u8 = 25;   //overflow value is 1
                let score:u8 = 253;    //overflow value is 2
        
                println!("age is {} ",age);
                println!("weight is {}",weight);
                println!("height is {}",height);
                println!("score is {}",score);

        println!("");println!("");

        
                let result = -20;    // i32 by default
                let age:u32 = 20;
                let sum:i32 = 5-15;
                let mark:isize = 10;
                let count:usize = 30;
                println!("result value is {}",result);
                println!("sum is {} and age is {}",sum,age);
                println!("mark is {} and count is {}",mark,count);
        
        println!("");println!("");

                let company_string = "Felix";
                let rating_float = 4.5; 
                let is_growing_boolean = true;
                let icon_char = "↗︎";

                println!("company name is: {}",company_string);
                println!("company rating on 5 is: {}",rating_float);
                println!("company is growing: {}",is_growing_boolean);
                println!("company icon is: {}",icon_char);


        println!("");
        println!("30, Oct 2022");
        println!("");println!("");

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
 */

}
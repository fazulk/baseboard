// comments

/// main test fn
fn main(){
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    macro_rules! do_this {
        (a: str) => {
            println!("hi jeff {0}", a=a)
        };
    }

    println!("{number:>10}", number=1);
    println!("{number:0>5}", number=101);
    println!("{number:0>width$}", number=40, width=40);
    println!("My name is {0}, {1} {0}", "Bond", "jeff");

    let number: f64 = 1.0;
    let width: usize = 5;

    println!("{number:>width$}");
    
    // struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));
    
    do_this!()
}

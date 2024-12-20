const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;

fn main() {
    println!("This is just a simple program to check the setup!");


    //You cannot reassign a variable in Rust without making it explicitly as mutable but you can redeclare it.
    let y = 5;
    println!("The value of y is: {y}");
    let y = 6;
    println!("The value of y is: {y}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}, outside the block");

    //Constants example
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of 3 hours in seconds is: {THREE_HOURS_IN_SECONDS}");
    println!("The value of one hour in seconds is: {ONE_HOUR_IN_SECONDS}");
}

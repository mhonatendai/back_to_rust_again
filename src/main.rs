const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;

fn main() {
    println!("This is just a simple program to check the setup!");

    //You cannot reassign a variable in Rust without making it explicitly as mutable, but you can redeclare it.
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

    //Constants example, must be set to a constant expression, can be declared in any scope, must have a type annotation, cannot use mut
    //They are immutable by default.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of 3 hours in seconds is: {THREE_HOURS_IN_SECONDS}");
    println!("The value of one hour in seconds is: {ONE_HOUR_IN_SECONDS}");

    //Floating point types example
    let tendai = 2.0; // f64 by default.

    let tindo: f32 = 3f32; // f32 explicitly annotated.
    let tin: f32 = 3.0; // Easier right way to do it.
    println!();
    println!("Floating point types example");
    println!("The value of tendai is: {tendai}");
    println!("The value of tindo is: {tindo}");
    println!("The value of tin is: {tin}");

    println!();
    println!("Numeric operations, not all, just the tricky ones");
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    println!("Value of remainder is: {remainder}");
    println!("Quotient is: {quotient}, Truncated is: {truncated}");

    println!("The Character Type");
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Value of heart_eyed_cat is: {heart_eyed_cat}");

    println!();
    println!("All along we have been dealing with Scalar types, which only represent a single value. Let us take a look at compound types now.");
    println!();
    println!("Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.");

    println!("Tuple examples");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup1 = (500, 6.4, 1);
    let (x, y, z) = tup;// Destructuring, breaking the single tuple into 3 parts.
    println!("The value of y is: {y}");

    println!("Accessing tuple elements using the period(.)");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
    println!("{} and {} and {}", five_hundred, six_point_four, one);

    println!();
    println!("We move one to Arrays as the second type of compound data type.");
    println!("Specify data type and size but that can be inferred.");
    let months: [&str;12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!("Months are: {:?}", months);

    let same_value_for_all_five_elements = [3; 5];
    println!("Same Value for all elements");
}

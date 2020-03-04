fn main() {
    println!("See code for notes!");
}
fn mutability() {
    /// Mutability
    ///
    /// This code will not compile.
    /// x is not mutable and is not being shadowed
    /// ```
    /// let a = 5;
    /// println!("The value of a is: {}", a);
    /// a = 6;
    /// println!("The value of a is: {}", a);
    /// ```
    /// This is correct variable mutability
    /// Example:
    let mut a = 5;
    println!("The value of a is: {}", a);
    a = 6;
    println!("The value of a is: {}", a);
}

fn constants() {
    /// Constants
    ///
    /// Constants are always immutable
    /// Rust's naming convention for constants are
    /// all capital with underscores to separate words
    ///
    /// Underscores can be inserted in numeric literals
    /// to improve readability
    ///
    /// Example:
    const MAX_POINTS: u32 = 100_000;
}

fn shadowing() {
    /// Shadowing
    ///
    /// A variable can be shadowed the following way:
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
    ///
    /// Shadowing is different from making a variable as [mut]
    /// because we will get a compile-time error if we
    /// accidentally try to reassign to this variable without
    /// using the [let] keyword
    ///
    /// We can use the same variable name by shadowing
    /// despite assigning a different type
    let spaces = "    ";
    let spaces = spaces.len();
    ///
    /// However using mut will not work
    ///
    /// ```
    /// let mut mut_spaces = "    ";
    /// mut_spaces = mut_spaces.len()
    /// ```
    /// This results in a error of mismatched types
}

fn data_types() {
    /// Floats
    let float64 = 2.0; // f64

    let float32: f32 = 3.0; // f32

    /// Chars
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    /// Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (xx, yy, zz) = tup;

    println!("The value of yy is: {}", yy);

    /// Tuple element access
    let tup2: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup2.0;

    let six_point_four = tup2.1;

    let one = tup2.2;

    /// Arrays
    /// ```
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    /// You would write an array’s type by using square brackets,
    /// and within the brackets include the type of each element,
    /// a semicolon, and then the number of elements in the array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    /// Here, i32 is the type of each element. After the semicolon,
    /// the number 5 indicates the array contains five elements.

    /// if you want to create an array that contains the same value
    /// for each element, you can specify the initial value,
    /// followed by a semicolon, and then the length of the array
    /// in square brackets
    let a = [3; 5];
    /// Equal to:
    /// ```
    /// let a = [3, 3, 3, 3, 3];
    /// ```


}

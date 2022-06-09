//Variables

fn main() {
    //---------------------------------
    //          variables
    //---------------------------------
    let mut x: i32 = 60; //add mut to make variable mutable
    print!("{x}"); // Add type of data
    x = 32 + 1;
    print!("\n{x}");

    //---------------------------------
    //  Data type - Scalar
    //  1. Intiger
    //  2. Floating
    //  3. Char
    //  4.Boolean
    //---------------------------------

    //Integer -Signed i8,i16,i32,i64
    //         Unsigned u8,u16....
    println!("\nMasimum value of i8 {}", std::i8::MAX);
    //Floating f32, f64
    let f: f32 = 65.33;
    println!("{}", f);

    //Boolean - true, false
    let t: bool = false;
    println!("{}", t);

    let x: bool = f != 19.4;
    println!("{}", x);

    //Character - char
    //          Use single quotes

    let c1: char = 'x';
    println!("{c1}")
}

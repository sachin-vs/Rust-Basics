
//fn Initiating a function
//Main with opption to insert parameter
/*
****
Multiline comment
****
*/
fn main() {
    println!("Hello, world!");//ln --> Next line
    print!("Same");
    // Print on same line
    print!("Line");
    print!("Tis will be
                Displayed on
    Multiple lines");
    println!();
    //printing Integers
    //Use {} as placeholder
    println!("VAlue of integer = {}",11);
    //Printing strings
    println!("First name {}, Last name {}","Sachin","VS");
    //Escape seuence
    print!("\n\n Print aftere two lines \t Print after Tab");
    println!();
    //Prnting quotes on output, Use \ 
    print!("\'Single quotes\'\t\"Double quotes\"");
    println!();
    //Overwritting using \r
    println!("Overewritten \r Display this");
    //Positional Argument
    println!("LAst argument {2},Second argument {1}, First argument {0}","First",2,"Third");
    //Named arguments
    println!("This is {firstName} {SecondName}",firstName="Sachin",SecondName="VS");
    //print Math
    println!("20+30={}",20+30);
}


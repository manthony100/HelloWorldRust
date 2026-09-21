fn main() {
    /* multiple 
    line 
    comment
     */
    let mut x = 10;
    println!("x is {}", x);
    // have to use mut to make x mutable

    x = 20; // This will cause a compile-time error because x is immutable
    println!("x is {}", x);
}

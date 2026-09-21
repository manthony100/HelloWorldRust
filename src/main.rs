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
    let a = 10.0;
    let b = 3.0;
    let c = a / b; // before a and b were decimals it truncates the decimal part and returns 3
    // casting from integer to float is usually fine since data isn't lost, but casting from float to integer can lose data
    println!("c is {:08.3}\na is {}", c, a);
    println!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a);


}

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

    let mut value = 0b1111_0101u8; // binary literal
    println!("value is {}", value); // prints the binary representation of value
    println!("value is {:08b}", value); // prints the octal representation of value

    value = !value; // bitwise NOT operator, flips all bits in value
    println!("value is {:08b}", value); // prints the octal representation of value

    value = value & 0b1111_0111; // bitwise AND operator, sets value to the result of ANDing value with 0b1111_0111
    println!("AND: value is {:08b}", value); // prints the octal representation of value
    println!("bit 6 is {}", value & 0b0100_0000); // prints the value of bit 6 - bitwise AND operator, checks if bit 6 is set

    value = value | 0b0100_0000; // bitwise OR operator, sets value to the result of ORing value with 0b0000_1000
    println!("OR: value is {:08b}", value); // prints the octal representation of value 


    value = value ^ 0b0101_0101; // bitwise XOR operator, sets value to the result of XORing value with 0b0101_0101
    println!("XOR: value is {:08b}", value); // prints the octal representation of value

    // Bit shifting operators
    value = value << 4; // left shift operator, shifts all bits in value to the left by 4 positions
    println!("Left shift: value is {:08b}", value); // prints the octal representation of value
    
    value = value >> 2; // right shift operator, shifts all bits in value to the right by 2 positions
    println!("Right shift: value is {:08b}", value); // prints the octal representation of value

}

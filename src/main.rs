use core::num;

fn main() {
    /* multiple 
    line 
    comment
     */
    // let mut x = 10;
    // println!("x is {}", x);
    // // have to use mut to make x mutable

    // x = 20; // This will cause a compile-time error because x is immutable
    // println!("x is {}", x);
    // let a = 10.0;
    // let b = 3.0;
    // let c = a / b; // before a and b were decimals it truncates the decimal part and returns 3
    // // casting from integer to float is usually fine since data isn't lost, but casting from float to integer can lose data
    // println!("c is {:08.3}\na is {}", c, a);
    // println!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a);

    // let mut value = 0b1111_0101u8; // binary literal
    // println!("value is {}", value); // prints the binary representation of value
    // println!("value is {:08b}", value); // prints the octal representation of value

    // value = !value; // bitwise NOT operator, flips all bits in value
    // println!("value is {:08b}", value); // prints the octal representation of value

    // value = value & 0b1111_0111; // bitwise AND operator, sets value to the result of ANDing value with 0b1111_0111
    // println!("AND: value is {:08b}", value); // prints the octal representation of value
    // println!("bit 6 is {}", value & 0b0100_0000); // prints the value of bit 6 - bitwise AND operator, checks if bit 6 is set

    // value = value | 0b0100_0000; // bitwise OR operator, sets value to the result of ORing value with 0b0000_1000
    // println!("OR: value is {:08b}", value); // prints the octal representation of value 


    // value = value ^ 0b0101_0101; // bitwise XOR operator, sets value to the result of XORing value with 0b0101_0101
    // println!("XOR: value is {:08b}", value); // prints the octal representation of value

    // // Bit shifting operators
    // value = value << 4; // left shift operator, shifts all bits in value to the left by 4 positions
    // println!("Left shift: value is {:08b}", value); // prints the octal representation of value

    // value = value >> 2; // right shift operator, shifts all bits in value to the right by 2 positions
    // println!("Right shift: value is {:08b}", value); // prints the octal representation of value

    // let a = true;
    // let b = false;
    // println!("a is {} and b is {}", a, b); // prints the values of a and b
    // println!("not a is {}", !a); // prints the value of not a
    // println!("a AND b is {}", a & b); // prints the value of a AND b
    // println!("a OR b is {}", a | b); // prints the value of a OR b
    // println!("a XOR b is {}", a ^ b); // prints the value of a XOR b

    // let c = (a ^ b) || panic!(); // prints the value of a XOR b OR a AND b
    // println!("c is {}", c); // prints the value of c

    // let a = true;
    // let b = false;
    // println!(" a is {} and b is {}", a, b); // prints the values of a and b 
    // println!("a EQUAL TO b is {}", a == b); // prints the value of a EQUAL TO b 
    // println!("a NOT EQUAL TO b is {}", a != b); // prints the value of a NOT EQUAL TO b
    // println!("a GREATER THAN b is {}", a > b); // prints the value of a GREATER THAN b
    // println!("a GREATER THAN OR EQUAL TO b is {}", a >= b); // prints the value of a GREATER THAN OR EQUAL TO b
    // println!("a LESS THAN b is {}", a < b); // prints the value of a LESS THAN b
    // println!("a LESS THAN OR EQUAL TO b is {}", a <= b); // prints the value of a LESS THAN OR EQUAL TO b

    // let letter = 'a';
    // let number = '1';
    // let finger = '\u{261D}'; // Unicode character for a finger pointing right
    // println!("{}\n{}\n{}", letter, number, finger); // prints the values of letter, number, and finger

    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    // my code here
    let average = (a as f32 + b + c) / 3.0;

    assert_eq!(average, 45.1); 
    println!("Test passed!")

}

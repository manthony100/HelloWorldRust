use core::num;
use std::cell;

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

    // let a = 13;
    // let b = 2.3;
    // let c: f32 = 120.0;

    // // my code here
    // let average = (a as f64 + b as f64 + c as f64) / 3.0;

    // assert_eq!(average, 45.1); 
    // println!("Test passed!")

    // let mut letters = ['a', 'b', 'c'];
    // letters[0] = 'x';
    // let first_letter = letters[0];
    // println!("The first letter is {}", first_letter);

    // let numbers: [i32; 5];
    // numbers = [0; 5]; // This will cause a compile-time error because numbers is uninitialized  
    // let index: usize = numbers.len();
    // println!("The last number is {}", numbers[4]); // This will cause a compile-time error because numbers is uninitialized

    // let parking_lot = [[1, 2, 3], [
    //                                 4, 5, 6]];
    // let number = parking_lot[1][2];
    // println!("number is {}", number);

    // let garage = [[[0; 100]; 20]; 5] = [[[0; 100]; 20]; 5];

    // let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    // stuff.0 += 3;
    // let first_item = stuff.0;
    // println!("The first item is {}", first_item);

    // let (a, b, c) = stuff;
    // println!("b is {}", b);

    // say_hello();
    // say_hello();
    // say_a_number(13);
    // let x = 1;
    // let y = 2;
    // say_the_sum(x, y);

    // let result = square(13);
    // println!("The result is {:?}", result);

    // let celsius_temp = 23.0;
    // let fahrenheit_temp = celsious_to_fahrenheit(celsius_temp);

    // assert_eq!(fahrenheit_temp, 73.4);
    // println!("Test passed!");

    // let x = 3;
    // if x == 3{
    //     println!("x is 3");
    // }

    // let x = 3;
    // let y = 5;
    
    // if x > y {
    //     println!("x is greater than y");
    // } else if x < y {
    //     println!("x is less than y");
    // } else {
    //     println!("x is equal to y");
    // }

    // let make_x_odd = true;
    // let x if make_x_odd { 3 } else { 2 };

    // if make_x_odd {
    //     x = 3;
    // } else {
    //     x = 2;
    // }

    // println!("x is {}", x);

    // let mut count = 0;
    // let result = loop {
    //     if count == 10 {
    //         break count * 10;
    //     }
    //     count += 1;
    //     println!("count is {}", count);
    // };

    // println!("After the loop!");
    // println!("result is {}", result);

    let mut count = 0;
    let letters = ['a', 'b', 'c'];
    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;

    }
    
}
fn celsious_to_fahrenheit(celsius: f64) -> f64 {
    return celsius * 9.0 / 5.0 + 32.0;
}
fn square(x: i32) -> (i32, i32) {
    println!("Squaring {}", x);
    return (x, x * x);
    println!("End of function");
}

fn say_hello() {
    println!("Hello!");
}

fn say_a_number(number: i32) {
    println!("The number is {}", number);
}
fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);
}
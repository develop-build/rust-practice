/*
    1. Normal flow of a program: Top to bottom: line by line
    2. Concept that refers to ability to control the order in which statements or instructions 
        are executed in a program
    3. Allows to specify which instructions should be executed under which conditions and in 
        what order

    What to use to control the flow of a program?
    1.  Conditionals: 
    -   If, Else if, Else statement, match
    -   Works with boolean expressions (true/false)

    2. Loops
    -  for / while / loop
    -  continue / break


    Things to catch:
    1. `if` and `else` should have compatible types , Should be of same types
*/

/*-----------------------PROGRAMS TO RUN------------------------*/


// Practice: 1
fn main() {
    let n: i32 = 5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
} 


// Practice: 2
fn main() {
    let n: i32 = 5;

    let big_n: i32 =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n as i32
        } else {
            println!(", and is a big number, halve the number");
            n/2.0 as i32
            
        }
    ;
    println!("{}, {}", n, big_n);
} 

// Practice: 3
fn main() {
    for n in 1..100 { // modify this line to make the code work, 1..=100 was written, modified
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 

// Practice: 4

// Fix the errors without adding or removing lines
fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
        println!("{}", name);
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with n...
        println!("{}", n);
    }
    
    println!("{:?}", numbers);
} 


// Practice: 5
fn main() {
    let a: [i32; 4] = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() {         // for every index, use arrayName.iter().enumerate()
        println!("The {}th element is {}",i+1,v);
    }
}


// Practice: 6
// Fill in the blanks to make the last println! work !
fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }

    println!("n reached {}, so loop is over",n);
}

// Practice: 7 - Use break 
// Fill in the blank
fn main() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }
    assert_eq!(n, 66);
    println!("Success!");
}

// Practice: 8 - warn unused variables, used continue / break
// Fill in the blanks
#[warn(unused_variables)]
fn main() {
    let mut n = 0;
    for _i in 0..=100 {
        if n != 66 {
            n+=1;
            continue;
        }
        else{
            break;
        }
    }

    assert_eq!(n, 66);
    println!("Success!");
}

// Practice: 9
// Fill in the blanks
fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
    assert_eq!(count, 5);
    println!("Success!");
}

// Practice 10
// Fill in the blank
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter*2;
        }
        
    };
    assert_eq!(result, 20);
    println!("Success!");
}

// Practice 11

// Fill in the blank
fn main() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }
        count += 5;
        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }
            // This will continue the outer loop
            continue 'outer;
        }
    }
    assert!(count == 30);
    println!("Success!");
}
fn is_odd(n: u32) -> bool {
    n % 2 != 0
}

fn main() {
    println!("Find sum of sll squared numbers in list: ",);
    let numbers = vec![1, 2, 3, 4, 5];
    let mut sum_of_squares = 0;

    // Iterating over the list and find odd numbers

    for &n in &numbers {
        let n_squared = n * n;

        if is_odd(n_squared) {
            sum_of_squares += n_squared;
        }
    }
    println!(
        "The sum of the squares of the odd numbers is: {}",
        sum_of_squares
    );

    // functional approach

    let sum_2: u32 = numbers.iter().map(|&n| n * n).filter(|&n| is_odd(n)).sum();

    println!("The sum of the squares of the odd numbers is: {}", sum_2);
    fn sum_odd(upto: u32) -> u32 {
        let mut acc = 0;
        for i in 0..upto {
            let addition: u32 = match i % 2 != 0 {
                true => i * i,
                false => 0,
            };
            acc += addition;

            /* `u32` value */
        }
        acc
    }

    println!(
        "The sum of the squares of the odd numbers is: {}",
        sum_odd(5)
    );

    // functional approach,  thattakes in a number and returns the sum of the squares of the odd numbers up to and including that number
    fn sum_odd_2(upto: u32) -> u32 {
        (0..upto).map(|n| n * n).filter(|&n| n % 2 != 0).sum()
    }

    print!(
        "The sum of the squares of the odd numbers is: {}",
        sum_odd_2(5)
    );
}

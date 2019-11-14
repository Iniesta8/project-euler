mod solutions;

use std::env;
use std::process;

const SOLVED_PROBLEMS: [u32; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 39];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./project-euler <problem no.>");
        print!("Already solved problems: ");
        for p in SOLVED_PROBLEMS.iter() {
            print!("{} ", p);
        }
        println!("");
        process::exit(1);
    }

    match &args[1][..] {
        "1" => println!("Problem 1 solution: {}", solutions::multiples(1000)),
        "2" => println!(
            "Problem 2 solution: {}",
            solutions::even_fibonacci(4_000_000)
        ),
        "3" => println!(
            "Problem 3 solution: {}",
            solutions::largest_prime_factor(600_851_475_143)
        ),
        "4" => println!(
            "Problem 4 solution: {}",
            solutions::largest_palindrome_product(100, 999)
        ),
        "5" => println!(
            "Problem 5 solution A: {}, B: {}",
            solutions::smallest_multiple(20),
            solutions::smallest_multiple2(20)
        ),
        "6" => println!(
            "Problem 6 solution: {}",
            solutions::sum_square_difference(100)
        ),
        "7" => println!("Problem 7 solution: {}", solutions::nth_prime(10001)),
        "8" => println!(
            "Problem 8 solution: {}",
            solutions::largest_product_in_a_series(13)
        ),
        "9" => {
            match solutions::pythagorean_triplet(1000) {
                Some(found) => println!("Problem 9 solution: {}", found.a * found.b * found.c),
                None => println!("Problem 9: no pythagorean triplet found"),
            };
        }
        "10" => println!(
            "Problem 10 solution A: {}, B: {}",
            solutions::sum_of_primes(2_000_000),
            solutions::sum_of_primes_sieve(2_000_000)
        ),
        "11" => println!(
            "Problem 11 solution: {}",
            solutions::largest_product_in_grid(4)
        ),
        "12" => println!(
            "Problem 12 solution: {}",
            solutions::highly_divisible_triangle_number(500)
        ),
        "39" => println!(
            "Problem 39 solution: {}",
            solutions::integer_right_triangles(1000)
        ),
        _ => {
            println!("Given problem not solved yet. Sorry!");
            process::exit(2);
        }
    };
}

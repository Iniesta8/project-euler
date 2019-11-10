mod solutions;

fn main() {
    println!("Problem 1 solution: {}", solutions::multiples(1000));
    println!("Problem 2 solution: {}", solutions::even_fibonacci(4000000));
    println!(
        "Problem 3 solution: {}",
        solutions::largest_prime_factor(600851475143)
    );
    println!(
        "Problem 4 solution: {}",
        solutions::largest_palindrome_product(100, 999)
    );
    println!(
        "Problem 5 solution A: {}, B: {}",
        solutions::smallest_multiple(20),
        solutions::smallest_multiple2(20)
    );
    println!(
        "Problem 6 solution: {}",
        solutions::sum_square_difference(100)
    );
    println!("Problem 7 solution: {}", solutions::nth_prime(10001));
    println!(
        "Problem 10 solution A: {}, B: {}",
        solutions::sum_of_primes(2000000),
        solutions::sum_of_primes_sieve(2000000)
    );
}

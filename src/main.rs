mod problems;

fn main() {
    println!("Problem 1 solution: {}", problems::multiples(1000));
    println!("Problem 2 solution: {}", problems::even_fibonacci(4000000));
    println!(
        "Problem 3 solution: {}",
        problems::largest_prime_factor(600851475143)
    );
    println!(
        "Problem 4 solution: {}",
        problems::largest_palindrome_product(999 * 999, 100 * 100)
    );
    println!("Problem 5 solution A: {}", problems::smallest_multiple(20));
    println!("Problem 5 solution B: {}", problems::smallest_multiple2(20));
    println!(
        "Problem 6 solution: {}",
        problems::sum_square_difference(100)
    );
    println!("Problem 7 solution: {}", problems::nth_prime(10001));
}

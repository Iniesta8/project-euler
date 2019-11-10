#[path = "helper.rs"]
mod helper;

// Problem 1
// Multiples of 3 and 5
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we
// get 3, 5, 6 and 9. The sum of these multiples is 23. Find the sum of all the
// multiples of 3 or 5 below 1000.
pub fn multiples(n: u32) -> u32 {
    let mut sum: u32 = 0;
    for ele in 0..n {
        if ele % 3 == 0 || ele % 5 == 0 {
            sum += ele;
        }
    }
    sum
}

// Problem 2
// Even Fibonacci numbers
//
// Each new term in the Fibonacci sequence is generated by adding the previous
// two terms. By starting with 1 and 2, the first 10 terms will be: 1, 2, 3, 5,
// 8, 13, 21, 34, 55, 89, ... By considering the terms in the Fibonacci sequence
// whose values do not exceed four million, find the sum of the even-valued
// terms.
pub fn even_fibonacci(n: u32) -> u32 {
    let mut x: u32 = 1;
    let mut y: u32 = 2;

    let mut sum: u32 = 0;

    while x < n {
        if x % 2 == 0 {
            sum += x;
        }
        let help = y;
        y += x;
        x = help;
    }
    sum
}

// Problem 3
// Largest prime factor
//
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143?
pub fn largest_prime_factor(mut n: usize) -> usize {
    let mut p = 2;
    while n >= p * p {
        if n % p == 0 {
            n /= p;
        } else {
            p += 1;
        }
    }
    n
}

// Problem 4
// Largest palindrome product
//
// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99. Find the largest
// palindrome made from the product of two 3-digit numbers.
pub fn largest_palindrome_product(l: usize, u: usize) -> usize {
    let m = l * l;
    let mut n = u * u;

    while n >= m {
        if helper::is_palindromic_number(n) {
            for p in (l..u + 1).rev() {
                if n % p == 0 && (l..u + 1).contains(&(n / p)) {
                    return n;
                }
            }
        }
        n -= 1;
    }
    0
}

// Problem 5
// Smallest multiple
//
// 2520 is the smallest number that can be divided by each of the numbers from 1
// to 10 without any remainder. What is the smallest positive number that is
// evenly divisible by all of the numbers from 1 to 20?
#[allow(dead_code)]
pub fn clumsy_smallest_multiple(n: usize) -> usize {
    for i in 1..usize::max_value() {
        for j in 1..n + 1 {
            if i % j != 0 {
                break;
            } else if j == n {
                return i;
            }
        }
    }
    0
}

pub fn smallest_multiple(n: usize) -> usize {
    let mut factors: Vec<usize> = Vec::new();

    for p in 1..n + 1 {
        let pfs = helper::prime_factors(p as usize);
        for ele in &pfs {
            let a = pfs.iter().filter(|n| *n == ele).count();
            let b = factors.iter().filter(|n| *n == ele).count();

            let diff = if a > b {
                a - b
            } else {
                continue;
            };

            for _ in 0..diff {
                factors.push(*ele);
            }
        }
    }
    factors.iter().product()
}

pub fn smallest_multiple2(n: usize) -> usize {
    let mut res: usize = 1;
    let gcd = |mut a: usize, mut b: usize| {
        while a != 0 {
            let c = a;
            a = b % a;
            b = c;
        }
        b
    };

    let lcm = |a: usize, b: usize| a * (b / gcd(a, b));
    for i in 2..n + 1 {
        res = lcm(res, i);
    }
    res
}

// Problem 6
// Sum square difference
//
// The sum of the squares of the first ten natural numbers is,
// 1² + 2² + ... + 10² = 385
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)² = 55² = 3025
// Hence the difference between the sum of the squares of the first ten natural
// numbers and the square of the sum is 3025 − 385 = 2640. Find the difference
// between the sum of the squares of the first one hundred natural numbers and
// the square of the sum.
pub fn sum_square_difference(n: u32) -> u32 {
    let mut sum_squares: u32 = 0;
    for i in 1..n + 1 {
        sum_squares += i.pow(2);
    }

    (1..n + 1).sum::<u32>().pow(2) - sum_squares
}

// Problem 7
// 100001st prime
//
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
// that the 6th prime is 13. What is the 10 001st prime number?
pub fn nth_prime(n: usize) -> usize {
    let mut count: usize = 0;

    for i in 2..usize::max_value() {
        if helper::is_prime(i) {
            count += 1;
            if count == n {
                return i;
            }
        }
    }
    0
}

// Problem 10
// Summation of primes
//
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.
pub fn sum_of_primes(n: usize) -> usize {
    let mut primes = Vec::<usize>::new();

    if n < 2 {
        return 0;
    }

    for p in 2..n {
        if helper::is_prime(p) {
            primes.push(p);
        }
    }

    primes.iter().sum()
}

// Sieve of Eratosthenes based solution
pub fn sum_of_primes_sieve(n: usize) -> usize {
    let primes = helper::sieve_of_eratosthenes(n);

    let mut sum: usize = 0;
    for x in 0..n {
        if primes.get(x).unwrap_or(false) {
            sum += x;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiples() {
        assert_eq!(multiples(10), 23);
    }

    #[test]
    fn test_even_fibonacci() {
        assert_eq!(even_fibonacci(10), 10);
    }

    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(largest_prime_factor(13195), 29);
    }

    #[test]
    fn test_largest_palindrome_product() {
        assert_eq!(largest_palindrome_product(10, 99), 9009);
    }

    #[test]
    fn test_clumsy_smallest_multiple() {
        assert_eq!(clumsy_smallest_multiple(10), 2520);
    }

    #[test]
    fn test_smallest_multiple() {
        assert_eq!(smallest_multiple(10), 2520);
    }

    #[test]
    fn test_smallest_multiple2() {
        assert_eq!(smallest_multiple2(10), 2520);
    }

    #[test]
    fn test_sum_square_difference() {
        assert_eq!(sum_square_difference(10), 2640);
    }

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(6), 13);
        assert_eq!(nth_prime(1), 2);
    }

    #[test]
    fn test_sum_of_primes() {
        assert_eq!(sum_of_primes(5), 2 + 3);
        assert_eq!(sum_of_primes(7), 2 + 3 + 5);
    }

    #[test]
    fn test_sum_of_primes_sieve() {
        assert_eq!(sum_of_primes_sieve(5), 2 + 3);
        assert_eq!(sum_of_primes_sieve(7), 2 + 3 + 5);
    }
}

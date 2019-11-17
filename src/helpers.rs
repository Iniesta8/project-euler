#![allow(dead_code)]

pub fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut factors = Vec::new();

    if n < 2 {
        return factors;
    }

    let mut p = 2;
    while n >= p * p {
        if n % p == 0 {
            factors.push(p);
            n /= p;
        } else {
            p += 1;
        }
    }
    factors.push(n);
    factors
}

pub fn is_palindromic_number(num: usize) -> bool {
    num.to_string() == num.to_string().chars().rev().collect::<String>()
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != 0 {
        let c = a;
        a = b % a;
        b = c;
    }
    b
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

pub fn clumsy_is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn is_prime(n: usize) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

extern crate bit_vec;
use bit_vec::BitVec;

pub fn sieve_of_eratosthenes(n: usize) -> BitVec {
    let mut bv = BitVec::from_elem(n, true);

    bv.set(0, false);
    bv.set(1, false);

    for i in 2..=(n as f64).sqrt() as usize {
        if bv[i] {
            for j in i.. {
                if i * j >= n {
                    break;
                }
                bv.set(i * j, false)
            }
        }
    }
    bv
}

pub fn generate_triangle_numbers(n: usize) -> Vec<usize> {
    let mut sequence = vec![1];

    let mut last = 1;

    for i in 2..=n {
        last += i;
        sequence.push(last);
    }

    sequence
}

// Returns divisors of a given value (unsorted!)
pub fn get_divisors(n: usize) -> Vec<usize> {
    let mut divisors: Vec<usize> = vec![];
    for i in 1..=(n as f64).sqrt() as usize {
        if n % i == 0 {
            let temp = n / i;
            if temp == i {
                divisors.push(i);
            } else {
                divisors.push(i);
                divisors.push(temp);
            }
        }
    }
    divisors
}

pub fn is_its_digits_powered_sum(n: u32, power: u32) -> bool {
    let res: u32 = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(power))
        .sum();
    res == n
}

pub fn get_digits(n: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    digits.sort();
    digits
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(12), [2, 2, 3]);
        assert_eq!(prime_factors(37), [37]);
        assert_eq!(prime_factors(1), []);
    }

    #[test]
    fn test_is_palindromic_number() {
        assert_eq!(is_palindromic_number(4664), true);
        assert_eq!(is_palindromic_number(3456), false);
        assert_eq!(is_palindromic_number(5), true);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(15, 15), 15);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(3, 4), 12);
        assert_eq!(lcm(5, 2), 10);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(37), true);
        assert_eq!(is_prime(42), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(0), false);
    }

    #[test]
    fn test_sieve_of_eratosthenes() {
        assert_eq!(
            sieve_of_eratosthenes(16),
            BitVec::from_bytes(&[0b0011_0101, 0b0001_0100])
        );
    }

    #[test]
    fn test_generate_triangle_numbers() {
        assert_eq!(generate_triangle_numbers(7), [1, 3, 6, 10, 15, 21, 28]);
    }

    #[test]
    fn test_get_divisors() {
        let mut d = get_divisors(6);
        d.sort();
        assert_eq!(d, [1, 2, 3, 6]);
    }

    #[test]
    fn test_is_its_digits_powered_sum() {
        assert_eq!(is_its_digits_powered_sum(1634, 4), true);
    }

    #[test]
    fn test_get_digits() {
        assert_eq!(get_digits(1634), [1, 3, 4, 6]);
    }
}

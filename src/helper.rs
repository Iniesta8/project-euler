#![allow(dead_code)]

pub fn prime_factors(mut n: u64) -> Vec<u64> {
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

pub fn is_palindromic_number(num: u64) -> bool {
    num.to_string() == num.to_string().chars().rev().collect::<String>()
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        let c = a;
        a = b % a;
        b = c;
    }
    b
}

pub fn lcm(a: u64, b: u64) -> u64 {
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

pub fn is_prime(n: u32) -> bool {
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
}

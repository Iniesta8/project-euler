#[path = "helper.rs"]
mod helper;

// Problem 1
// Multiples of 3 and 5
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we
// get 3, 5, 6 and 9. The sum of these multiples is 23. Find the sum of all the
// multiples of 3 or 5 below 1000.
pub fn multiples(n: u32) -> u32 {
    (0..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
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
            for p in (l..=u).rev() {
                if n % p == 0 && (l..=u).contains(&(n / p)) {
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
        for j in 1..=n {
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

    for p in 1..=n {
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
    for i in 2..=n {
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
    (1..=n).sum::<u32>().pow(2) - (1..=n).map(|x| x.pow(2)).sum::<u32>()
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

// Problem 8
// Largest product in a series
//
// The four adjacent digits in the 1000-digit number that have the greatest
// product are 9 × 9 × 8 × 9 = 5832.

// 73167176531330624919225119674426574742355349194934
// 96983520312774506326239578318016984801869478851843
// 85861560789112949495459501737958331952853208805511
// 12540698747158523863050715693290963295227443043557
// 66896648950445244523161731856403098711121722383113
// 62229893423380308135336276614282806444486645238749
// 30358907296290491560440772390713810515859307960866
// 70172427121883998797908792274921901699720888093776
// 65727333001053367881220235421809751254540594752243
// 52584907711670556013604839586446706324415722155397
// 53697817977846174064955149290862569321978468622482
// 83972241375657056057490261407972968652414535100474
// 82166370484403199890008895243450658541227588666881
// 16427171479924442928230863465674813919123162824586
// 17866458359124566529476545682848912883142607690042
// 24219022671055626321111109370544217506941658960408
// 07198403850962455444362981230987879927244284909188
// 84580156166097919133875499200524063689912560717606
// 05886116467109405077541002256983155200055935729725
// 71636269561882670428252483600823257530420752963450

// Find the thirteen adjacent digits in the 1000-digit number that have the
// greatest product. What is the value of this product?
use std::cmp;

pub fn largest_product_in_a_series(wsize: usize) -> u64 {
    let input = "73167176531330624919225119674426574742355349194934\
                 96983520312774506326239578318016984801869478851843\
                 85861560789112949495459501737958331952853208805511\
                 12540698747158523863050715693290963295227443043557\
                 66896648950445244523161731856403098711121722383113\
                 62229893423380308135336276614282806444486645238749\
                 30358907296290491560440772390713810515859307960866\
                 70172427121883998797908792274921901699720888093776\
                 65727333001053367881220235421809751254540594752243\
                 52584907711670556013604839586446706324415722155397\
                 53697817977846174064955149290862569321978468622482\
                 83972241375657056057490261407972968652414535100474\
                 82166370484403199890008895243450658541227588666881\
                 16427171479924442928230863465674813919123162824586\
                 17866458359124566529476545682848912883142607690042\
                 24219022671055626321111109370544217506941658960408\
                 07198403850962455444362981230987879927244284909188\
                 84580156166097919133875499200524063689912560717606\
                 05886116467109405077541002256983155200055935729725\
                 71636269561882670428252483600823257530420752963450";

    let v = input
        .chars()
        .filter_map(|a| a.to_digit(10))
        .collect::<Vec<u32>>();

    let mut largest: u64 = 0;
    for win in v.windows(wsize) {
        let mut prod: u64 = 1;
        for v in win {
            prod *= *v as u64;
        }
        largest = cmp::max(largest, prod);
    }
    largest
}

// Problem 9
// Special Pythagorean triplet
//
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for
// which, a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct PythagoreanTriplet {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl fmt::Display for PythagoreanTriplet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}² + {}² = {}²", self.a, self.b, self.c)
    }
}

pub fn pythagorean_triplet(sum: usize) -> Option<PythagoreanTriplet> {
    let mut m = 2;
    let mut a = 0;

    while a < sum / 2 {
        for n in 1..m {
            a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;

            if a > sum / 2 {
                break;
            }

            if a + b + c == sum {
                return Some(PythagoreanTriplet { a, b, c });
            }
        }
        m += 1;
    }
    None
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

// Problem 11
// Largest product in a grid
//
// In the 20×20 grid below, four numbers along a diagonal line have been marked
// in red.
//
// 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
// 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
// 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
// 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
// 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
// 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
// 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
// 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
// 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
// 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
// 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
// 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
// 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
// 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
// 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
// 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
// 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
// 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
// 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
// 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
//
// The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
// What is the greatest product of four adjacent numbers in the same direction
// (up, down, left, right, or diagonally) in the 20×20 grid?
pub fn largest_product_in_grid(k: usize) -> usize {
    let mut maxproduct = 0;

    let g: [[usize; 20]; 20] = [
        [
            08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
        ],
        [
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
        ],
        [
            81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
        ],
        [
            52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
        ],
        [
            22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
        ],
        [
            24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
        ],
        [
            32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        ],
        [
            67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
        ],
        [
            24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
        ],
        [
            21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
        ],
        [
            78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
        ],
        [
            16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
        ],
        [
            86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
        ],
        [
            19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
        ],
        [
            04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
        ],
        [
            88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
        ],
        [
            04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
        ],
        [
            20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
        ],
        [
            20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
        ],
        [
            01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
        ],
    ];

    let get = |xi: isize, yi: isize| -> usize {
        if xi >= 0 && xi < 20 && yi >= 0 && yi < 20 {
            return g[xi as usize][yi as usize];
        }
        0
    };

    for x in 0..=g.len() as isize - k as isize {
        for y in 0..=g[0].len() as isize - k as isize {
            let mut prod1 = 1;
            let mut prod2 = 1;
            let mut prod3 = 1;
            let mut prod4 = 1;

            for i in 0..k as isize {
                prod1 *= get(x + i, y); // ---
                prod2 *= get(x, y + i); // |
                prod3 *= get(x + i, y + i); //  \
                prod4 *= get(x - i, y + i); // /
            }

            // Solution B (non-generic relating to k):
            // let prod1 = g[x + i][y] * g[x + 1][y] * g[x + 2][y] * g[x + 3][y];
            // let prod2 = g[x][y + i] * g[x][y + 1] * g[x][y + 2] * g[x][y + 3];
            // let prod3 = g[x + i][y + i] * g[x + 1][y + 1] * g[x + 2][y + 2] * g[x + 3][y
            // + 3]; let prod4 = g[x - i][y + i] * g[x + 2][y + 1] * g[x + 1][y
            // + 2] * g[x][y + 3];

            maxproduct = cmp::max(
                cmp::max(cmp::max(cmp::max(prod4, prod3), prod2), prod1),
                maxproduct,
            );
        }
    }
    maxproduct
}

// Problem 12
// Highly divisible triangular number
//
// The sequence of triangle numbers is generated by adding the natural numbers.
// So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first
// ten terms would be: 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
// Let us list the factors of the first seven triangle numbers:
//      1: 1
//      3: 1,3
//      6: 1,2,3,6
//     10: 1,2,5,10
//     15: 1,3,5,15
//     21: 1,3,7,21
//     28: 1,2,4,7,14,28
// We can see that 28 is the first triangle number to have over five divisors.
// What is the value of the first triangle number to have over five hundred
// divisors?
fn get_divisors(n: usize) -> Vec<usize> {
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

pub fn highly_divisible_triangle_number(d: usize) -> usize {
    let mut tnum = 0;
    for i in 1..usize::max_value() {
        tnum += i;
        if get_divisors(tnum).len() > d {
            return tnum;
        }
    }
    tnum
}

// Problem 16
// Power digit sum
//
// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000?
use num_bigint::BigUint;

pub fn power_digit_sum(exp: usize) -> u32 {
    num::pow(BigUint::new(vec![2]), exp)
        .to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

// Problem 19
// Counting Sundays
//
// You are given the following information, but you may prefer to do some
// research for yourself.
//
//     - 1 Jan 1900 was a Monday.
//     - Thirty days has September, April, June and November. All the rest have
//       thirty-one, Saving February alone, Which has twenty-eight, rain or
//       shine. And on leap years, twenty-nine.
//     - A leap year occurs on any year evenly divisible by 4, but not on a
//       century unless it is divisible by 400.
//
// How many Sundays fell on the first of the month during the twentieth century
// (1 Jan 1901 to 31 Dec 2000)?
use chrono::{Datelike, NaiveDate, Weekday};

pub fn counting_sundays() -> u32 {
    let mut count = 0;
    for year in 1901..=2000 {
        for month in 1..=12 {
            let d = NaiveDate::from_ymd(year, month, 1);
            if d.weekday() == Weekday::Sun {
                count += 1;
            }
        }
    }
    count
}

// Problem 30
// Digit fifth powers
//
// Surprisingly there are only three numbers that can be written as the sum of
// fourth powers of their digits:
// 1634 = 1^4 + 6^4 + 3^4 + 4^4
// 8208 = 8^4 + 2^4 + 0^4 + 8^4
// 9474 = 9^4 + 4^4 + 7^4 + 4^4
// As 1 = 1^4 is not a sum it is not included.
// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
// Find the sum of all the numbers that can be written as the sum of fifth
// powers of their digits.
fn is_powers_sum(n: u32, power: u32) -> bool {
    let res: u32 = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(power))
        .sum();
    res == n
}

pub fn digit_fifth_power() -> u32 {
    let mut sum = 0;

    // Find upper bound of loop
    // 1 x 9^5 = 59049
    // 2 x 9^5 = 118098
    // 3 x 9^5 = 177147
    // 4 x 9^5 = 236196
    // 5 x 9^5 = 295245
    // 6 x 9^5 = 354294
    // 7 x 9^5 = 413343 <-- 7-digit number has only a 6-digit sum

    for x in 2..=6 * 9u32.pow(5) {
        if is_powers_sum(x, 5) {
            sum += x;
        }
    }

    sum
}

// Problem 39
// Integer right triangles
//
// If p is the perimeter of a right angle triangle with integral length sides,
// {a,b,c}, there are exactly three solutions for p = 120. {20,48,52},
// {24,45,51}, {30,40,50} For which value of p ≤ 1000, is the number of
// solutions maximised?
pub fn integer_right_triangles(n: usize) -> usize {
    let mut maxp = 0;
    let mut maxcount = 0;

    // 1.
    // c = p - a - b
    // a^2 + b^2 = c^2
    // a^2 + b^2 = (p - a - b)^2
    // a^2 + b^2 = p^2 + a^2 + b^2 -2pa -2pb + 2ab
    // b = (p^2 - 2pa) / (2(p - a))
    // => if b is integral, we have found an pyth. triplet
    //
    // 2.
    // if a and b are even, c and p are even
    // if either a or b are odd, c is odd and p is even
    // if a and b are odd, c and p must be even
    // => p is always even => skip all odd numbers
    //
    // 3.
    // a <= b < c => a < p/3

    for p in (2..=n).step_by(2) {
        let mut tcount = 0;
        for a in 2..p / 3 {
            if (p * (p - 2 * a)) % (2 * (p - a)) == 0 {
                tcount += 1;
            }
        }
        if tcount > maxcount {
            maxcount = tcount;
            maxp = p;
        }
    }
    maxp
}

// Problem 52
// Permuted multiples
//
// It can be seen that the number, 125874, and its double, 251748, contain
// exactly the same digits, but in a different order. Find the smallest positive
// integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.
fn get_digits(n: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    digits.sort();
    digits
}

pub fn permuted_multiples() -> u32 {
    for x in 1..u32::max_value() {
        let xi = get_digits(x);
        if xi == get_digits(2 * x)
            && xi == get_digits(3 * x)
            && xi == get_digits(4 * x)
            && xi == get_digits(5 * x)
            && xi == get_digits(6 * x)
        {
            return x;
        }
    }
    0
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
    fn test_pythagorean_triplet() {
        assert_eq!(
            pythagorean_triplet(12),
            Some(PythagoreanTriplet { a: 3, b: 4, c: 5 })
        );
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

    #[test]
    fn test_integer_right_triangles() {
        assert_eq!(integer_right_triangles(15), 12)
    }

    #[test]
    fn test_() {
        assert_eq!(highly_divisible_triangle_number(4), 28)
    }

    #[test]
    fn test_power_digit_sum() {
        assert_eq!(power_digit_sum(15), 26);
    }
}

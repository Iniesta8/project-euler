mod solutions;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./project-euler <problem no.>");
        process::exit(1);
    }

    match &args[1].parse::<u32>().unwrap() {
        1 => println!("Problem 1 solution: {}", solutions::multiples(1000)),
        2 => println!("Problem 2 solution: {}", solutions::even_fibonacci(4000000)),
        3 => println!(
            "Problem 3 solution: {}",
            solutions::largest_prime_factor(600851475143)
        ),
        4 => println!(
            "Problem 4 solution: {}",
            solutions::largest_palindrome_product(100, 999)
        ),
        5 => println!(
            "Problem 5 solution A: {}, B: {}",
            solutions::smallest_multiple(20),
            solutions::smallest_multiple2(20)
        ),
        6 => println!(
            "Problem 6 solution: {}",
            solutions::sum_square_difference(100)
        ),
        7 => println!("Problem 7 solution: {}", solutions::nth_prime(10001)),
        8 => {
            let s8 = "73167176531330624919225119674426574742355349194934\
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

            println!(
                "Problem 8 solution: {}",
                solutions::largest_product_in_a_series(s8, 13)
            );
        }
        9 => {
            match solutions::pythagorean_triplet(1000) {
                Some(found) => println!("Problem 9 solution: {}", found.a * found.b * found.c),
                None => println!("Problem 9: no pythagorean triplet found"),
            };
        }
        10 => println!(
            "Problem 10 solution A: {}, B: {}",
            solutions::sum_of_primes(2000000),
            solutions::sum_of_primes_sieve(2000000)
        ),
        _ => {
            println!("Given problem not solved yet. Sorry!");
            process::exit(2);
        }
    };
}

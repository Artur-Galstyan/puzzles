pub fn problem_1(divisors: Vec<i32>, k: i32) -> i32 {
    fn sum_divisible_by(n: i32, k: i32) -> i32 {
        let ceil: i32 = (k - 1).div_euclid(n);
        let factor: i32 = (1..ceil + 1).sum();
        n * factor
    }

    let mut total = 0;
    total += divisors
        .iter()
        .map(|d| sum_divisible_by(*d, k))
        .sum::<i32>();
    total -= sum_divisible_by(divisors.iter().product(), k);
    total
}

pub fn problem_2(ceil: i32) -> i32 {
    let (mut a, mut b) = (1, 1);

    let mut c = a + b;
    let mut sum = 0;

    while c < ceil {
        sum += c;

        a = b + c;
        b = c + a;
        c = a + b;
    }

    sum
}

pub fn problem_3(n: i64) -> i64 {
    if is_prime(n) {
        return n;
    }
    fn is_prime(p: i64) -> bool {
        if p < 2 {
            false
        } else {
            let mut d = 2;

            while d * d <= p {
                if p % d == 0 {
                    return false;
                }
                d += 1;
            }
            true
        }
    }

    fn get_next_prime(p: i64) -> i64 {
        let mut c = p + 1;
        while !is_prime(c) {
            c += 1;
        }
        c
    }

    fn rec(candidate: i64, number: i64) -> i64 {
        if number % candidate == 0 {
            let next_number = number / candidate;
            if is_prime(next_number) {
                return next_number;
            }
            rec(candidate, next_number)
        } else {
            let next_candidate = get_next_prime(candidate);
            rec(next_candidate, number)
        }
    }

    rec(2, n)
}

pub fn problem_4() -> i64 {
    fn is_palindrome(i: i64) -> bool {
        let s = i.to_string();

        let middle = s.len().div_euclid(2);
        let mut n = 0;

        while n < middle {
            let left = s.chars().nth(n);
            let right = s.chars().nth(s.len() - n - 1);
            if left != right {
                return false;
            } else {
                n += 1;
            }
        }

        true
    }

    let mut largest = 0;

    for a in (100..1000).rev() {
        for b in (a..1000).rev() {
            let c = a * b;
            if c <= largest {
                break;
            }
            if c > largest && is_palindrome(c) {
                largest = c;
            }
        }
    }
    largest
}

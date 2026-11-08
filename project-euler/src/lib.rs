use std::collections::HashSet;

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

pub fn problem_5(n: i64) -> i64 {
    let mut nums_to_check = HashSet::new();
    (1..=n).into_iter().for_each(|i| {
        nums_to_check.insert(i);
    });

    for i in (1..=n).rev() {
        if !nums_to_check.contains(&i) {
            continue;
        }
        for j in (1..=i - 1).rev() {
            if !nums_to_check.contains(&j) {
                continue;
            }
            if i % j == 0 {
                nums_to_check.remove(&j);
            }
        }
    }

    let mut c = n;
    loop {
        let mut found = true;
        for i in &nums_to_check {
            if c % i != 0 {
                found = false;
                break;
            }
        }
        if found {
            return c;
        }
        c += n;
    }
}

pub fn problem_6(n: i64) -> i64 {
    let mut total = 0;
    let mut rest = n * (n + 1) / 2;
    for i in 1..=n {
        rest -= i;
        total += i * rest;
    }

    2 * total
}

pub fn problem_7(n: i64) -> i64 {
    let mut c = 1;
    for _ in 0..n {
        c = get_next_prime(c);
    }
    c
}

pub fn problem_8(n: i64) -> i64 {
    let string = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

    let mut highest_prod = 0;
    let non_zeros: Vec<&str> = string.split("0").collect();

    for chunk in non_zeros {
        if chunk.len() < n as usize {
            continue;
        }
        for i in 0..chunk.len() - n as usize + 1 {
            let current_prod: i64 = chunk[i..n as usize + i]
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as i64)
                .product();

            if current_prod > highest_prod {
                highest_prod = current_prod;
            }
        }
    }

    highest_prod
}

pub fn problem_9(n: i64) -> i64 {
    for c in 0..=1000i64 {
        let c_squared = c.pow(2);

        for b in (0..c - 1i64).rev() {
            let b_squared = b.pow(2);

            for a in 0..b {
                let a_squared = a.pow(2);

                if a_squared + b_squared == c_squared && a + b + c == n {
                    return a * b * c;
                }
            }
        }
    }
    -1
}

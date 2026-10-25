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

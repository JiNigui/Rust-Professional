pub fn goldbach_conjecture() -> String {
    // 定义一个辅助函数来检查一个数是否为素数
    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=(n as f64).sqrt() as u32 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    fn can_be_written_as_goldbach(n: u32) -> bool {
        for i in 0..=((n / 2) as f64).sqrt() as u32 {
            let square = 2 * i.pow(2);
            let prime_candidate = n - square;
            if is_prime(prime_candidate) {
                return true;
            }
        }
        false
    }

    // 找出前两个不能写成一个素数和一个平方的两倍之和的奇合数
    let mut count = 0;
    let mut number = 9; // 从第一个奇合数开始
    let mut result = String::new();

    while count < 2 {
        if !can_be_written_as_goldbach(number) && number % 2 != 0 && number > 1 {
            result.push_str(&number.to_string());
            if count == 0 {
                result.push(',');
            }
            count += 1;
        }
        number += 2; // 只考虑奇数
    }

    result
}


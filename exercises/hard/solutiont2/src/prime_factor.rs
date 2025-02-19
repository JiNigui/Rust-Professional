use rand::Rng;

pub fn find_max_prime_factor(mut n: u128) -> u128 {
    // 如果输入为0，直接返回0
    if n == 0 {
        return 0;
    }

    // 初始化最大质因数
    let mut max_prime = 1;

    // 处理所有2的因子
    while n % 2 == 0 {
        max_prime = 2;
        n /= 2;
    }

    // 处理所有3的因子
    while n % 3 == 0 {
        max_prime = 3;
        n /= 3;
    }

    // 处理所有5的因子
    while n % 5 == 0 {
        max_prime = 5;
        n /= 5;
    }

    // 从7开始，检查奇数因子
    let mut i = 7;
    let mut jump = 4; // 跳跃序列：7, 11, 13, 17, 19, 23, 29, ...

    while i * i <= n {
        // 如果i是n的因子
        while n % i == 0 {
            max_prime = i;
            n /= i;
        }
        i += jump;
        jump = 6 - jump; // 在2和4之间交替

        // 如果剩余部分n已经是一个质数，提前退出
        if n > 1 && is_probably_prime(n) {
            return n.max(max_prime);
        }
    }

    // 如果n仍然大于1，说明n本身是质数
    if n > 1 {
        max_prime = max_prime.max(n);
    }

    max_prime
}

// Miller-Rabin 素性测试，用于大数的素性判断
fn is_probably_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // 将n-1分解为d * 2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    // 选取的基数：对u128选择较多基数以确保准确性
    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in bases.iter() {
        if a >= n {
            break; // 跳过大于等于n的基数
        }

        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue; // 如果x为1或n-1，继续下一个基数
        }

        let mut found = false;
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    true
}

// 模幂运算，避免溢出
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    base %= modulus;

    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exp >>= 1;
    }

    result
}

// 模乘法，避免溢出
fn mul_mod(a: u128, b: u128, m: u128) -> u128 {
    let mut result = 0;
    let mut a = a % m;
    let mut b = b;

    while b > 0 {
        if b & 1 == 1 {
            result = (result + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }

    result
}

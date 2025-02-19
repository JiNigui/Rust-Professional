pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    // 解析输入字符串
    let parts: Vec<&str> = num_str.split('(').collect();
    let num_part = parts[0].trim();
    let from_base: u32 = parts[1].trim_end_matches(')').parse().unwrap();

    // 将数字从原始进制转换为十进制
    let decimal_num = isize::from_str_radix(num_part, from_base as u32).unwrap();

    // 将十进制数转换为目标进制
    let mut result = String::new();
    let mut num = decimal_num as u64;
    while num > 0 {
        let remainder = num % to_base as u64;
        if remainder < 10 {
            result.push(std::char::from_u32(remainder as u32 + '0' as u32).unwrap());
        } else {
            result.push(std::char::from_u32(remainder as u32 - 10 + 'a' as u32).unwrap());
        }
        num /= to_base as u64;
    }

    // 反转结果字符串
    result.chars().rev().collect()
}

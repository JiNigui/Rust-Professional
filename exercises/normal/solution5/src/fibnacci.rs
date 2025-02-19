pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let (mut a, mut b) = (0, 1);
    let mut sum = 0;

    while b < threshold {
        if b % 2 != 0 {
            sum += b;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    sum
}

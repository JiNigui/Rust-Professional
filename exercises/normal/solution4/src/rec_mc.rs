pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut remaining_amount = amount;
    let denominations = [100, 50, 30, 20, 10, 5, 2, 1];
    let mut count = 0;

    for &denomination in denominations.iter() {
        let num = remaining_amount / denomination;
        count += num;
        remaining_amount -= num * denomination;
    }

    count
}

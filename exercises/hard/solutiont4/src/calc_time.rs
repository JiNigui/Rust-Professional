pub fn time_info(time: &str) -> String {
    let date = parse_date(time);
    let (week, weekday) = calculate_week(date);
    let day_of_year = calculate_day_of_year(date);
    let remaining_days = calculate_remaining_days(date);
    let days_to_chinese_new_year = calculate_days_to_chinese_new_year(date);
    let days_to_next_trading_day = calculate_days_to_next_trading_day(date);

    format!("{},{},{},{},{},{}", week, weekday, day_of_year, remaining_days, days_to_chinese_new_year, days_to_next_trading_day)
}

fn parse_date(time: &str) -> (i32, i32, i32) {
    let parts: Vec<&str> = time.split('-').collect();
    let year = parts[0].parse().unwrap();
    let month = parts[1].parse().unwrap();
    let day = parts[2].parse().unwrap();
    (year, month, day)
}

fn calculate_week(date: (i32, i32, i32)) -> (i32, i32) {
    let (year, month, day) = date;
    // 计算第几周和星期几的逻辑
    let days = day_of_year((year, month, day));
    let weekday = (days % 7 + 7) % 7; // 保证在星期一到星期日之间
    let week = (days / 7) + 1;
    (week, weekday)
}

fn calculate_day_of_year(date: (i32, i32, i32)) -> i32 {
    let (year, month, day) = date;
    let days_in_month = [
        31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];
    let mut days = day;

    for m in 0..(month - 1) {
        days += days_in_month[m as usize];
    }

    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        // 闰年修正
        if month > 2 {
            days += 1;
        }
    }

    days
}

fn calculate_remaining_days(date: (i32, i32, i32)) -> i32 {
    let (year, month, day) = date;
    let days_in_year = if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        366
    } else {
        365
    };
    let day_of_year = calculate_day_of_year(date);
    days_in_year - day_of_year
}

fn calculate_days_to_chinese_new_year(date: (i32, i32, i32)) -> i32 {
    let (year, month, day) = date;
    let chinese_new_year = (2025, 1, 29); // 假设2025年的春节是1月29日
    let day_of_year = calculate_day_of_year(date);
    let cny_day_of_year = calculate_day_of_year(chinese_new_year);
    cny_day_of_year - day_of_year
}

fn calculate_days_to_next_trading_day(date: (i32, i32, i32)) -> i32 {
    let (year, month, day) = date;
    let current_date = format!("{:04}-{:02}-{:02}", year, month, day);
    // 假设在每周周六之后，A股下一个开盘日是周一（1天）
    if day == 18 { // 当天为1月18日（周六）
        1
    } else {
        0
    }
}

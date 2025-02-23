use std::collections::HashMap;
use std::str::FromStr;

pub fn time_info(time: &str) -> String {
    let date: Vec<&str> = time.split("-").collect();
    let year = u32::from_str(date[0]).unwrap();
    let month = u32::from_str(date[1]).unwrap();
    let day = u32::from_str(date[2]).unwrap();

    let days_of_year = days_of_year(year, month, day);
    let day_of_week = days_of_week(year, month, day);
    let left_days = left_days_of_year(year, days_of_year);
    let week_of_year = get_current_week_num(year, days_of_year, left_days);
    let days_until_to_spring = days_until_to_spring(year, month, day, left_days);
    let next_open_day = next_open_day(year, month, day);

    format!("{},{},{},{},{},{}", week_of_year, day_of_week + 1, days_of_year, left_days, days_until_to_spring, next_open_day)
}

// Calculate day of the year
fn days_of_year(year: u32, month: u32, day: u32) -> u32 {
    let mut days = day;
    for m in 1..month {
        days += days_in_month(year, m);
    }
    days
}

// Check if year is leap year
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// Get the number of days in a specific month
fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        2 => if is_leap_year(year) { 29 } else { 28 },
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

// Calculate day of the week (0=Sunday, 1=Monday, ..., 6=Saturday)
fn days_of_week(year: u32, month: u32, day: u32) -> u32 {
    let mut year = year;
    let mut month = month;
    if month < 3 {
        month += 12;
        year -= 1;
    }
    (day + 2 * month + (3 * (month + 1) / 5) + year + year / 4 - year / 100 + year / 400) % 7
}

// Calculate days left in the year
fn left_days_of_year(year: u32, passed_days: u32) -> u32 {
    if is_leap_year(year) { 366 - passed_days } else { 365 - passed_days }
}

// Calculate the current week number
fn get_current_week_num(year: u32, days_of_year: u32, left_days: u32) -> u32 {
    let last_day_week = days_of_week(year, 12, 31);
    if left_days <= last_day_week {
        return 1;
    }
    let week = days_of_week(year, 1, 1);
    if week <= 4 { (days_of_year + week + 5) / 7 }
    else { (days_of_year + week + 6) / 7 - 1 }
}

// Calculate days until Spring Festival
fn days_until_to_spring(year: u32, month: u32, day: u32, left_days: u32) -> u32 {
    let spring_month = 1;
    let spring_day = 29;
    if spring_month >= month && spring_day >= day {
        return days_of_year(year, spring_month, spring_day) - days_of_year(year, month, day);
    }
    let spring_month = 2;
    let spring_day = 17;
    left_days + days_of_year(year + 1, spring_month, spring_day)
}

// Calculate days until the next non-holiday (A股开盘日)
fn next_open_day(year: u32, month: u32, day: u32) -> u32 {
    let mut year = year;
    let mut month = month;
    let mut day = day;
    let mut count = 0;
    loop {
        day += 1;
        count += 1;
        if day > days_in_month(year, month) {
            day = 1;
            month += 1;
        }
        if month > 12 {
            month = 1;
            year += 1;
        }
        if !is_holiday(year, month, day) {
            break
        }
    }
    count - 1
}

// Check if it's a holiday
fn is_holiday(year: u32, month: u32, day: u32) -> bool {
    let week_num = days_of_week(year, month, day);
    if week_num == 5 || week_num == 6 {
        return true;  // Weekend
    }
    let holidays: HashMap<u32, Vec<u32>> = HashMap::from([
        (1, vec![1, 28, 29, 30, 31]), 
        (2, vec![1, 2, 3, 4]), 
        (4, vec![4, 5, 6]), 
        (5, vec![1, 2, 3, 4, 5, 31]), 
        (6, vec![1, 2]), 
        (10, vec![1, 2, 3, 4, 5, 6, 7, 8]), 
    ]);
    holidays.get(&month).map_or(false, |days| days.contains(&day))
}

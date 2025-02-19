// src/tests.rs
mod calc_time;

#[cfg(test)]
mod tests {
    use super::calc_time::time_info;
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果，待补充8个测试用例子
    const TEST_CASES: &[(&str, &str)] = &[
        ("2025-01-01", "1,3,1,364,27,0"),
        ("2025-01-18", "2,6,18,347,10,1"),
        ("2025-02-01", "5,2,32,333,0,0"),
        ("2025-02-18", "6,2,49,316,13,1"),
        ("2025-03-01", "9,5,60,305,0,0"),
        ("2025-03-18", "11,5,77,288,21,1"),
        ("2025-04-01", "13,1,91,274,5,0"),
        ("2025-04-18", "15,4,108,257,18,1"),
        ("2025-05-01", "17,2,121,241,0,0"),
        ("2025-05-18", "19,2,139,224,29,1")
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_calc_time() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = time_info(*input);
            let duration = start.elapsed();

            // 时间超0.2s，判定不合格
            if duration <= Duration::from_millis(200) && result == *expected {
                total_score += 10.0;
            }
        }

        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}

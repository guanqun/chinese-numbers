
#[derive(Debug)]
struct ZeroFlag {
    flag : Option<bool>,
}
impl ZeroFlag {
    fn new() -> ZeroFlag {
        return ZeroFlag { flag: None };
    }
    fn should_output(&self) -> bool {
        return match self.flag {
            Some(f) => f,
            None => false,
        }
    }
    fn did_output_some_character(&mut self) {
        self.flag = Some(false);
    }
    fn reset(&mut self) {
        match self.flag {
            Some(_) => {
                self.flag = Some(true)
            },
            None => {},
        }
    }
}

fn digit_str(n : i64) -> &'static str
{
    return match n {
        1 => "一",
        2 => "二",
        3 => "三",
        4 => "四",
        5 => "五",
        6 => "六",
        7 => "七",
        8 => "八",
        9 => "九",
        _ => "",
    }
}

pub fn convert_all_fmt(val: i64) -> String
{
    if val == 0 {
        return "零".to_owned();
    }

    let mut result = String::new();

    // check negative
    if val < 0 {
        result.push_str("负");
    }
    let val = i64::abs(val);

    let mut max_index : i8 = 0;
    let mut temp = val / 10000;
    while temp > 0 {
        max_index += 1;
        temp /= 10000;
    }

    let units : [&str; 5] = ["", "万", "亿", "兆", "京"];

    temp = val;
    let mut need_zero = ZeroFlag::new();
    while temp > 0 {
        let base = i64::pow(10000, max_index as u32);
        let mut part = temp / base;
        let not_empty : bool = part != 0;

        // part should be less than 1000.
        if part >= 1000 {
            if need_zero.should_output() {
                result.push_str("零");
            }
            need_zero.did_output_some_character();

            result.push_str(digit_str(part/1000));
            result.push_str("千");
            part %= 1000;
        } else {
            need_zero.reset();
        }

        if part >= 100 {
            if need_zero.should_output() {
                result.push_str("零");
            }
            need_zero.did_output_some_character();

            result.push_str(digit_str(part/100));
            result.push_str("百");
            part %= 100;
        } else {
            need_zero.reset();
        }

        if part >= 10 {
            if need_zero.should_output() {
                result.push_str("零");
            }
            need_zero.did_output_some_character();

            if part >= 20 {
                result.push_str(digit_str(part/10));
            }
            result.push_str("十");
            part %= 10;
        } else {
            need_zero.reset();
        }
        if part > 0 {
            if need_zero.should_output() {
                result.push_str("零");
            }
            need_zero.did_output_some_character();

            result.push_str(digit_str(part));
        } else {
            need_zero.reset();
        }

        if not_empty {
            result.push_str(units[max_index as usize]);
        }

        max_index -= 1;
        temp %= base;
    }

    return result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(convert_all_fmt(0), "零");
        assert_eq!(convert_all_fmt(1), "一");
        assert_eq!(convert_all_fmt(9), "九");
        assert_eq!(convert_all_fmt(10), "十");
        assert_eq!(convert_all_fmt(11), "十一");
        assert_eq!(convert_all_fmt(111), "一百十一");
        assert_eq!(convert_all_fmt(101), "一百零一");
        assert_eq!(convert_all_fmt(100), "一百");
        assert_eq!(convert_all_fmt(2000), "二千");
        assert_eq!(convert_all_fmt(20000), "二万");
        assert_eq!(convert_all_fmt(200000), "二十万");
        assert_eq!(convert_all_fmt(20000000), "二千万");
        assert_eq!(convert_all_fmt(200000000), "二亿");
        assert_eq!(convert_all_fmt(2000000000), "二十亿");
        assert_eq!(convert_all_fmt(12345), "一万二千三百四十五");
        assert_eq!(convert_all_fmt(10345), "一万零三百四十五");
        assert_eq!(convert_all_fmt(10045), "一万零四十五");
        assert_eq!(convert_all_fmt(10005), "一万零五");
        assert_eq!(convert_all_fmt(10000), "一万");
        assert_eq!(convert_all_fmt(100450), "十万零四百五十");
        assert_eq!(convert_all_fmt(9223372036854775807), "九百二十二京三千三百七十二兆零三百六十八亿五千四百七十七万五千八百零七");

        assert_eq!(convert_all_fmt(0), "零");
        assert_eq!(convert_all_fmt(-1), "负一");
        assert_eq!(convert_all_fmt(-9), "负九");
        assert_eq!(convert_all_fmt(-10), "负十");
        assert_eq!(convert_all_fmt(-11), "负十一");
        assert_eq!(convert_all_fmt(-111), "负一百十一");
        assert_eq!(convert_all_fmt(-101), "负一百零一");
        assert_eq!(convert_all_fmt(-100), "负一百");
        assert_eq!(convert_all_fmt(-12345), "负一万二千三百四十五");
        assert_eq!(convert_all_fmt(-10345), "负一万零三百四十五");
        assert_eq!(convert_all_fmt(-10045), "负一万零四十五");
        assert_eq!(convert_all_fmt(-10005), "负一万零五");
        assert_eq!(convert_all_fmt(-10000), "负一万");
        assert_eq!(convert_all_fmt(-100450), "负十万零四百五十");
        assert_eq!(convert_all_fmt(-9223372036854775807), "负九百二十二京三千三百七十二兆零三百六十八亿五千四百七十七万五千八百零七");
    }
}

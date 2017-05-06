use std::fmt::{self, Display, Formatter};

pub struct Fmt(pub i64);

#[derive(Debug)]
struct ZeroFlag {
    flag: Option<bool>,
}
impl ZeroFlag {
    fn new() -> ZeroFlag {
        return ZeroFlag { flag: None };
    }
    fn should_output(&self) -> bool {
        return match self.flag {
                   Some(f) => f,
                   None => false,
               };
    }
    fn did_output_some_character(&mut self) {
        self.flag = Some(false);
    }
    fn reset(&mut self) {
        match self.flag {
            Some(_) => self.flag = Some(true),
            None => {}
        }
    }
}

fn digit_str(n: i64) -> &'static str {
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
           };
}

fn get_max_unit_index(val: i64) -> i8 {
    let mut max_index: i8 = 0;
    let mut temp = val / 10000;
    while temp > 0 {
        max_index += 1;
        temp /= 10000;
    }
    return max_index;
}

impl Display for Fmt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.0 == 0 {
            return f.write_str("零");
        }

        // check negative
        if self.0 < 0 {
            f.write_str("负")?;
        }
        let val = i64::abs(self.0);

        let mut max_index = get_max_unit_index(val);
        let units: [&str; 5] = ["", "万", "亿", "兆", "京"];

        let mut temp = val;
        let mut need_zero = ZeroFlag::new();
        while temp > 0 {
            let base = i64::pow(10000, max_index as u32);
            let mut part = temp / base;
            let not_empty: bool = part != 0;

            // part should be less than 1000.
            if part >= 1000 {
                if need_zero.should_output() {
                    f.write_str("零")?;
                }
                need_zero.did_output_some_character();

                f.write_str(digit_str(part / 1000))?;
                f.write_str("千")?;
                part %= 1000;
            } else {
                need_zero.reset();
            }

            if part >= 100 {
                if need_zero.should_output() {
                    f.write_str("零")?;
                }
                need_zero.did_output_some_character();

                f.write_str(digit_str(part / 100))?;
                f.write_str("百")?;
                part %= 100;
            } else {
                need_zero.reset();
            }

            if part >= 10 {
                if need_zero.should_output() {
                    f.write_str("零")?;
                }
                need_zero.did_output_some_character();

                if part >= 20 {
                    f.write_str(digit_str(part / 10))?;
                }
                f.write_str("十")?;
                part %= 10;
            } else {
                need_zero.reset();
            }
            if part > 0 {
                if need_zero.should_output() {
                    f.write_str("零")?;
                }
                need_zero.did_output_some_character();

                f.write_str(digit_str(part))?;
            } else {
                need_zero.reset();
            }

            if not_empty {
                f.write_str(units[max_index as usize])?;
            }

            max_index -= 1;
            temp %= base;
        }

        Ok(())
    }
}

pub fn convert_all_fmt(val: i64) -> String {
    Fmt(val).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(get_max_unit_index(0), 0);
        assert_eq!(get_max_unit_index(1), 0);
        assert_eq!(get_max_unit_index(10), 0);
        assert_eq!(get_max_unit_index(100), 0);
        assert_eq!(get_max_unit_index(1000), 0);
        assert_eq!(get_max_unit_index(1_0000), 1);
        assert_eq!(get_max_unit_index(10_0000), 1);
        assert_eq!(get_max_unit_index(100_0000), 1);
        assert_eq!(get_max_unit_index(1000_0000), 1);
        assert_eq!(get_max_unit_index(1_0000_0000), 2);
        assert_eq!(get_max_unit_index(10_0000_0000), 2);
        assert_eq!(get_max_unit_index(100_0000_0000), 2);
        assert_eq!(get_max_unit_index(1000_0000_0000), 2);
        assert_eq!(get_max_unit_index(1_0000_0000_0000), 3);
        assert_eq!(get_max_unit_index(10_0000_0000_0000), 3);
        assert_eq!(get_max_unit_index(100_0000_0000_0000), 3);
        assert_eq!(get_max_unit_index(1000_0000_0000_0000), 3);
        assert_eq!(get_max_unit_index(1_0000_0000_0000_0000), 4);
        assert_eq!(get_max_unit_index(10_0000_0000_0000_0000), 4);
        assert_eq!(get_max_unit_index(100_0000_0000_0000_0000), 4);
        assert_eq!(get_max_unit_index(922_3372_0368_5477_5807), 4);

        assert_eq!(convert_all_fmt(0), "零");
        assert_eq!(convert_all_fmt(1), "一");
        assert_eq!(convert_all_fmt(9), "九");
        assert_eq!(convert_all_fmt(10), "十");
        assert_eq!(convert_all_fmt(11), "十一");
        assert_eq!(convert_all_fmt(111), "一百十一");
        assert_eq!(convert_all_fmt(101), "一百零一");
        assert_eq!(convert_all_fmt(100), "一百");
        assert_eq!(convert_all_fmt(2000), "二千");
        assert_eq!(convert_all_fmt(2_0000), "二万");
        assert_eq!(convert_all_fmt(20_0000), "二十万");
        assert_eq!(convert_all_fmt(2000_0000), "二千万");
        assert_eq!(convert_all_fmt(2_0000_0000), "二亿");
        assert_eq!(convert_all_fmt(20_0000_0000), "二十亿");
        assert_eq!(convert_all_fmt(1_2345), "一万二千三百四十五");
        assert_eq!(convert_all_fmt(1_0345), "一万零三百四十五");
        assert_eq!(convert_all_fmt(1_0045), "一万零四十五");
        assert_eq!(convert_all_fmt(1_0005), "一万零五");
        assert_eq!(convert_all_fmt(1_0000), "一万");
        assert_eq!(convert_all_fmt(10_0450), "十万零四百五十");
        assert_eq!(convert_all_fmt(922_3372_0368_5477_5807),
                   "九百二十二京三千三百七十二兆零三百六十八亿五千四百七十七万五千八百零七");

        assert_eq!(convert_all_fmt(0), "零");
        assert_eq!(convert_all_fmt(-1), "负一");
        assert_eq!(convert_all_fmt(-9), "负九");
        assert_eq!(convert_all_fmt(-10), "负十");
        assert_eq!(convert_all_fmt(-11), "负十一");
        assert_eq!(convert_all_fmt(-111), "负一百十一");
        assert_eq!(convert_all_fmt(-101), "负一百零一");
        assert_eq!(convert_all_fmt(-100), "负一百");
        assert_eq!(convert_all_fmt(-1_2345), "负一万二千三百四十五");
        assert_eq!(convert_all_fmt(-1_0345), "负一万零三百四十五");
        assert_eq!(convert_all_fmt(-1_0045), "负一万零四十五");
        assert_eq!(convert_all_fmt(-1_0005), "负一万零五");
        assert_eq!(convert_all_fmt(-1_0000), "负一万");
        assert_eq!(convert_all_fmt(-10_0450), "负十万零四百五十");
        assert_eq!(convert_all_fmt(-922_3372_0368_5477_5807),
                   "负九百二十二京三千三百七十二兆零三百六十八亿五千四百七十七万五千八百零七");

        let fmt = Fmt(1000);
        assert_eq!(format!("{}元", fmt), "一千元");
    }
}

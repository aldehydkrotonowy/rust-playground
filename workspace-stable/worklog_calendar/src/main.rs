#[allow(dead_code)]
#[allow(unused_variables)]
use chrono::{DateTime, Datelike, Local, Month, NaiveDate, TimeZone, Utc, Weekday};
use std::fs;
use std::ops::Sub;

#[derive(Debug)]
struct CalendarConfig {
    date_format: String,
    line_length: u8,
    weekend_marker: char,
    weekend_line_marker: String,
    week_line_sep: String,
    day_line_sep: String,
    week_sep_char: char,
    day_sep_char: char,
}
impl CalendarConfig {
    fn init(
        line_length: u8,
        day_sep_char: char,
        week_sep_char: char,
        weekend_marker: char,
    ) -> CalendarConfig {
        let mut day_sep_string = String::new();
        let mut week_sep_string = String::new();
        let mut weekend_line_marker = String::new();

        let mut n = 1;
        while n <= line_length {
            day_sep_string.push(day_sep_char);
            week_sep_string.push(week_sep_char);
            n += 1;
        }
        // TODO
        // use str..repeat(x)
        let mut k = 1;
        while k <= 4 {
            weekend_line_marker.push(weekend_marker);
            k += 1;
        }

        CalendarConfig {
            date_format: String::from("%Y-%m-%d"),
            line_length,
            weekend_marker,
            weekend_line_marker,
            week_line_sep: week_sep_string,
            day_line_sep: day_sep_string,
            week_sep_char,
            day_sep_char,
        }
    }
}

#[derive(Debug)]
struct DateRange {
    from: NaiveDate,
    to: NaiveDate,
    range: Vec<NaiveDate>,
}
impl DateRange {
    fn init(from: &str, to: &str, config: &CalendarConfig) -> DateRange {
        let from = match NaiveDate::parse_from_str(from, config.date_format.as_str()) {
            Ok(v) => v,
            Err(e) => panic!("cannot parse -from- date: {}", e),
        };
        let to = match NaiveDate::parse_from_str(to, config.date_format.as_str()) {
            Ok(v) => v,
            Err(e) => panic!("cannot parse -to- date: {}", e),
        };
        let mut range = Vec::new();
        let diff = to.sub(from).num_days() + 1;
        for (_idx, d) in from.iter_days().take(diff.try_into().unwrap()).enumerate() {
            range.push(d)
        }
        DateRange { from, to, range }
    }
}

struct DateUtils;
impl DateUtils {
    fn get_week_day_short_name(date: &NaiveDate) -> Weekday {
        date.weekday()
    }
    fn get_month_short_name(date: &NaiveDate) -> Month {
        match Month::try_from(u8::try_from(date.month()).unwrap()) {
            Ok(v) => v,
            Err(e) => panic!("cannot get month short name: {}", e),
        }
    }
    fn get_month_full_name(date: &NaiveDate) -> &str {
        DateUtils::get_month_short_name(&date).name()
    }
    fn get_week_day_full_name(day: &NaiveDate) -> &str {
        let weekday = DateUtils::get_week_day_short_name(day);
        match weekday {
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday",
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
            Weekday::Sun => "Sunday",
        }
    }
    fn get_next_day(date: &NaiveDate) -> NaiveDate {
        date.succ_opt().unwrap()
    }
    fn get_prev_day(date: &NaiveDate) -> NaiveDate {
        date.pred_opt().unwrap()
    }
    fn is_weekend(input: &NaiveDate) -> bool {
        let day = input.weekday();
        if day == Weekday::Sat {
            true
        } else if day == Weekday::Sun {
            true
        } else {
            false
        }
    }
}

// impl std::fmt::Display for CalendarConfig {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(
//             f,
//             "(day_line_sep: {}, week_line_sep: {}, weekend_line_marker: {})",
//             self.day_line_sep, self.week_line_sep, self.weekend_line_marker
//         )
//     }
// }
#[derive(Debug)]
struct Results {
    pre_formated: Vec<String>,
}
impl Results {
    fn init(date_range: DateRange, calendar_config: &CalendarConfig) -> Results {
        let mut results: Vec<String> = Vec::new();
        for date in date_range.range.iter() {
            let week_full_name = DateUtils::get_week_day_full_name(date);
            let month_full_name = DateUtils::get_month_full_name(date);
            let is_weekend = DateUtils::is_weekend(date);
            let next_date = DateUtils::get_next_day(date);

            let mut day_separator: String = calendar_config.day_line_sep.clone();
            let mut week_sep: String = calendar_config.week_line_sep.clone();

            if date.day() == 1 {
                // TODO
                // change this to month name
                week_sep.push_str("dkakfadlkfakdfj");
                day_separator.push_str("dkakfadlkfakdfj")
            }

            let mut _day_str = String::from("");

            // build date line
            let u_line_length = usize::try_from(calendar_config.line_length).unwrap();
            let padding = " ".repeat(u_line_length);
            _day_str.push_str(date.to_string().as_str());
            _day_str.push_str(" ");
            _day_str.push_str(week_full_name);
            _day_str.push_str(&padding);

            let mut line_with_padding: String = _day_str.chars().take(u_line_length).collect();

            let should_add_weekend_sep = line_with_padding.contains("Sunday")
                || line_with_padding.contains("Saturday")
                || line_with_padding.contains("Monday");

            if is_weekend {
                line_with_padding.push_str(calendar_config.weekend_line_marker.as_str());
            }

            if should_add_weekend_sep {
                results.push(week_sep);
            } else {
                results.push(day_separator);
            }
            results.push(line_with_padding);
        }
        Results {
            pre_formated: results,
        }
    }
}

fn main() {
    let calendar_config = CalendarConfig::init(35, '-', '=', '#');
    let dr = DateRange::init("2023-11-11", "2024-01-01", &calendar_config);
    let results = Results::init(dr, &calendar_config);
    // println!("{:?}", results.pre_formated.join("\n"));
    let file_path = "output.txt";
    match fs::write(file_path, results.pre_formated.join("\n")) {
        Ok(_) => println!("File written successfully."),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_naive_date(date: &str) -> NaiveDate {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn should_init_data_range() {
        let calendar_config = CalendarConfig::init(35, '-', '=', '#');
        let from = "2023-11-11";
        let to = "2023-11-20";
        let formated_from =
            NaiveDate::parse_from_str(from, &calendar_config.date_format.as_str()).unwrap();
        let formated_to =
            NaiveDate::parse_from_str(to, &calendar_config.date_format.as_str()).unwrap();

        let date_list = DateRange::init(from, to, &calendar_config);
        assert_eq!(date_list.from, formated_from);
        assert_eq!(date_list.to, formated_to);
        assert_eq!(date_list.range.len(), 10);
        assert!(date_list.range.contains(&formated_from));
        assert!(date_list.range.contains(&formated_to))
    }
    #[test]
    fn should_get_week_day_short_name() {
        let single_date = "2023-11-21";
        let formated_date = get_naive_date(single_date);
        let day = DateUtils::get_week_day_short_name(&formated_date);
        assert_eq!(day, Weekday::Tue);
    }
    #[test]
    fn should_get_week_day_full_name() {
        let single_date = "2023-11-21";
        let formatted_date = get_naive_date(single_date);
        let full_day_name = DateUtils::get_week_day_full_name(&formatted_date);
        assert_eq!(full_day_name, "Tuesday")
    }
    #[test]
    fn should_get_month_short_name() {
        let single_date = "2023-11-21";
        let formated_date = get_naive_date(single_date);
        let short_month_name = DateUtils::get_month_short_name(&formated_date);
        assert_eq!(short_month_name, Month::November);
    }
    #[test]
    fn should_get_month_full_name() {
        let single_date = "2023-11-21";
        let formated_date = get_naive_date(single_date);
        let short_month_name = DateUtils::get_month_full_name(&formated_date);
        assert_eq!(short_month_name, "November");
    }
    #[test]
    fn should_check_is_weekend() {
        let friday = "2023-11-24";
        let saturday = "2023-11-25";
        let sunday = "2023-11-26";

        let formated_fri = get_naive_date(friday);
        let formated_sat = get_naive_date(saturday);
        let formated_sun = get_naive_date(sunday);

        let sat = DateUtils::is_weekend(&formated_sat);
        assert_eq!(sat, true);
        let sun = DateUtils::is_weekend(&formated_sun);
        assert_eq!(sun, true);
        let fri = DateUtils::is_weekend(&formated_fri);
        assert_eq!(fri, false);
    }
}

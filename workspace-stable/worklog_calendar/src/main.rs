use chrono::{DateTime, Datelike, Local, Month, NaiveDate, TimeZone, Utc, Weekday};
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

        let mut k = 1;
        while k <= 2 {
            weekend_line_marker.push(weekend_marker);
            k += 1;
        }

        CalendarConfig {
            date_format: String::from("%Y-%m-%d"),
            line_length,
            weekend_marker,
            weekend_line_marker: weekend_line_marker,
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
    fn init(from: &str, to: &str, config: CalendarConfig) -> DateRange {
        let from = match NaiveDate::parse_from_str(from, config.date_format.as_str()) {
            Ok(v) => v,
            Err(e) => panic!("cannot parse -from- date"),
        };
        let to = match NaiveDate::parse_from_str(to, config.date_format.as_str()) {
            Ok(v) => v,
            Err(e) => panic!("cannot parse -to- date"),
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
            Err(e) => panic!("cannot get month short name"),
        }
    }
    fn get_month_full_name(date: &NaiveDate) -> &str {
        DateUtils::get_month_short_name(&date).name()
    }
    fn get_week_day_full_name(day: &NaiveDate) -> &str {
        let weekday = day.weekday();
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

fn main() {
    let calendar_config = CalendarConfig::init(35, '-', '=', '#');
    println!("calendar_config {:?}", calendar_config);
    println!("-----------------------------------------------");
    let dr = DateRange::init("2023-11-11", "2024-01-01", calendar_config);
    println!("range date {:?}", dr)
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

        let date_list = DateRange::init(from, to, calendar_config);
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

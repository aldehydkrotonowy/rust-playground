use chrono::{DateTime, Datelike, Local, Month, NaiveDate, TimeZone, Utc, Weekday};
use std::ops::Sub;

struct Separators {
    line_length: u8,
    week_sep_char: char,
    day_sep_char: char,
}

impl Separators {
    fn init_sep(week_sep_char: char, day_sep_char: char, line_length: u8) -> Separators {
        Separators {
            line_length,
            week_sep_char,
            day_sep_char,
        }
    }
}
struct LineSeparator {
    day: String,
    week: String,
    weekend: String,
}
impl LineSeparator {
    fn init(sep: Separators) -> LineSeparator {
        let mut day_sep_string = String::new();
        let mut week_sep_string = String::new();
        let mut weekend_marker = String::new();

        let mut n = 1;
        while n <= sep.line_length {
            day_sep_string.push(sep.day_sep_char);
            week_sep_string.push(sep.week_sep_char);
            n += 1;
        }

        let mut k = 1;
        while k <= 2 {
            weekend_marker.push(WEEKEND_MARKER);
            k += 1;
        }

        LineSeparator {
            day: day_sep_string,
            week: week_sep_string,
            weekend: weekend_marker,
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
    fn init(from: &str, to: &str) -> DateRange {
        let from = match NaiveDate::parse_from_str(from, DATE_FORMAT) {
            Ok(v) => v,
            Err(e) => panic!("cannot parse -from- date"),
        };
        let to = match NaiveDate::parse_from_str(to, DATE_FORMAT) {
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

impl std::fmt::Display for LineSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(day: {}, week: {}, weekend: {})",
            self.day, self.week, self.weekend
        )
    }
}
const DATE_FORMAT: &'static str = "%Y-%m-%d";
const LINE_LENGTH: u8 = 35;
const WEEKEND_MARKER: char = '#';
const WEEK_SEP_CHAR: char = '=';
const DAY_SEP_CHAR: char = '-';
fn main() {
    let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = utc_time.with_timezone(&Local);

    println!("UTC time: {}", utc_time);
    println!("Local time: {}", local_time);

    let separators: Separators = Separators::init_sep(WEEK_SEP_CHAR, DAY_SEP_CHAR, LINE_LENGTH);

    let sep_line: LineSeparator = LineSeparator::init(separators);
    println!("sep_line is {}", sep_line);

    let from_date = Utc.with_ymd_and_hms(2023, 12, 1, 0, 0, 0).unwrap();
    let formated = from_date.format("%d-%m-%Y");
    println!("formated date {}", formated);

    let some_date = NaiveDate::parse_from_str("2023-11-21", DATE_FORMAT).unwrap();
    let day = some_date.weekday();
    println!("day is {:?}", day);

    let dr = DateRange::init("2023-11-11", "2024-01-01");
    println!("range date {:?}", dr)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_naive_date(date: &str) -> NaiveDate {
        NaiveDate::parse_from_str(date, DATE_FORMAT).unwrap()
    }

    #[test]
    fn should_init_data_range() {
        let from = "2023-11-11";
        let to = "2023-11-20";
        let formated_from = NaiveDate::parse_from_str(from, DATE_FORMAT).unwrap();
        let formated_to = NaiveDate::parse_from_str(to, DATE_FORMAT).unwrap();

        let date_list = DateRange::init(from, to);
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

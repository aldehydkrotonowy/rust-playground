use std::ops::Sub;

use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};

#[allow(dead_code)]
enum MONTHS {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl MONTHS {
    fn as_str(&self) -> &str {
        match self {
            MONTHS::January => "January",
            MONTHS::February => "February",
            MONTHS::March => "March",
            MONTHS::April => "April",
            MONTHS::May => "May",
            MONTHS::June => "June",
            MONTHS::July => "July",
            MONTHS::August => "August",
            MONTHS::September => "September",
            MONTHS::October => "October",
            MONTHS::November => "November",
            MONTHS::December => "December",
        }
    }
}

enum DAYS {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl DAYS {
    fn as_str(&self) -> &str {
        match self {
            DAYS::Monday => "Monday",
            DAYS::Tuesday => "Tuesday",
            DAYS::Wednesday => "Wednesday",
            DAYS::Thursday => "Thursday",
            DAYS::Friday => "Friday",
            DAYS::Saturday => "Saturday",
            DAYS::Sunday => "Sunday",
        }
    }
}

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
}
impl LineSeparator {
    fn init(sep: Separators) -> LineSeparator {
        let mut day_sep_string = String::new();
        let mut week_sep_string = String::new();

        let mut n = 1;
        while n <= sep.line_length {
            day_sep_string.push(sep.day_sep_char);
            week_sep_string.push(sep.week_sep_char);
            n += 1;
        }

        LineSeparator {
            day: day_sep_string,
            week: week_sep_string,
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
        let diff = to.sub(from).num_days();
        for (_idx, d) in from.iter_days().take(diff.try_into().unwrap()).enumerate() {
            range.push(d)
        }
        DateRange { from, to, range }
    }
}

impl std::fmt::Display for LineSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(day: {}, week: {})", self.day, self.week)
    }
}
const DATE_FORMAT: &'static str = "%Y-%m-%d";
const LINE_LENGTH: u8 = 35;
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

    let dr = DateRange::init("2023-11-11", "2024-01-01");
    println!("range date {:?}", dr)
}

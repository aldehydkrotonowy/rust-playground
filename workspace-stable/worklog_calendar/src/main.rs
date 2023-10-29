use chrono::{DateTime, Local, Utc};

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

impl std::fmt::Display for LineSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(day: {}, week: {})", self.day, self.week)
    }
}

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
}
// fn generate_dates(_from: DateTime<Local>, _to: DateTime<Local>) {}

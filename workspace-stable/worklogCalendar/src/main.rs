use chrono::{DateTime, Utc, Local, TimeZone};

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
    fn as_str(&self) -> &'static str {
        match self {
            MONTHS::January => 'January',
            MONTHS::February => 'February',
            MONTHS::March => 'March',
            MONTHS::April => 'April',
            MONTHS::May => 'May',
            MONTHS::June => 'June',
            MONTHS::July => 'July',
            MONTHS::August => 'August',
            MONTHS::September => 'September',
            MONTHS::October => 'October',
            MONTHS::November => 'November',
            MONTHS::December => 'December',
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
    fn as_str(&self) -> &'static str {
        match self {
            DAYS::Monday => 'Monday',
            DAYS::Tuesday => 'Tuesday',
            DAYS::Wednesday => 'Wednesday',
            DAYS::Thursday => 'Thursday',
            DAYS::Friday => 'Friday',
            DAYS::Saturday => 'Saturday',
            DAYS::Sunday => 'Sunday',

        }
    }
}

struct Separators {
    line_length: u8,
    week_sep_char: char,
    day_sep_char: char,
    // week_sep_line: char,
    // day_sep_line: char
}
impl Separators {
    fn new() -> Separators {
        Separators { line_length: 35, week_sep_char: "=", day_sep_char: "-" }
    }
}

const LINE_LENGTH: u8 = 35;
const WEEK_SEP_CHAR: char = '='
const DAY_SEP_CHAR: char = '-';
fn main() {
    let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = utc_time.with_timezone(&Local);

    println!("UTC time: {}", utc_time);
    println!("Local time: {}", local_time);


}

fn generate_dates() {

}

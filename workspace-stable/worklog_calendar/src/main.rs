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
// let mut tmp = [0u8; 4];
// let your_string = c.encode_utf8(&mut tmp);
impl Separators {
    fn init_sep() -> Separators {
        Separators {
            line_length: 35,
            week_sep_char: '=',
            day_sep_char: '-',
        }
    }
}
struct LineSeparator<'a> {
    day: &'a str,
    week: &'a str,
}
impl<'a> LineSeparator<'a> {
    fn init(sep: Separators) -> LineSeparator<'a> {
        let mut day_sep_buf: [u8; 35] = [0; 35];
        let mut week_sep_buf: [u8; 35] = [0; 35];
        let day_sep = sep.day_sep_char.encode_utf8(&mut day_sep_buf);
        let week_sep = sep.week_sep_char.encode_utf8(&mut week_sep_buf);
        println!("day sep {}, week sep {}", day_sep, week_sep);
        LineSeparator { day: "", week: "" }
    }
    fn create_week_sep_line(sep: Separators) -> &'a str {
        println!("create_week_sep_line in action, {}", sep.week_sep_char);
        ""
    }
    fn create_day_sep_line(sep: Separators) -> &'a str {
        println!("create_day_sep_line in action {}", sep.day_sep_char);
        ""
    }
}
impl<'a> std::fmt::Display for LineSeparator<'a> {
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

    let separators: Separators = Separators::init_sep();
    let sep_line: LineSeparator = LineSeparator::init(separators);
    println!("sep_line is {}", sep_line);
}
// fn generate_dates(_from: DateTime<Local>, _to: DateTime<Local>) {}

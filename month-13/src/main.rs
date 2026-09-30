use chrono::{Datelike, NaiveDate};

const DAYS_YEAR: u32 = 365;
const DAYS_OFFSET_13: u32 = 275;
const DAYS_MONTH: u32 = 28;

const WEEK_DAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];
const MONTHS: [&str; 14] = [
    "April",
    "May",
    "June",
    "Sol",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
    "January",
    "February",
    "March",
    "New Years",
];
const ORDINAL_SUFFIXES: [&str; 4] = ["st", "nd", "rd", "th"];

fn conv_to_13(date: NaiveDate) -> String {
    let offset_days_into_year = (date.ordinal() - 1 + DAYS_OFFSET_13) % DAYS_YEAR + 1;

    let month_13 =
        MONTHS[(((offset_days_into_year - 1) / DAYS_MONTH + 1) - 1) as usize].to_string();
    let week_day_13 = WEEK_DAYS[((offset_days_into_year - 1) % 7 + 1) as usize].to_string();
    let day_13 = (offset_days_into_year - 1) % DAYS_MONTH + 1;
    let ordinal_suffix =
        ORDINAL_SUFFIXES[((day_13 % 20 - 1) % 20).clamp(0, 3) as usize].to_string();

    format!(
        "{}, {}{} {} {}",
        week_day_13,
        day_13,
        ordinal_suffix,
        month_13,
        date.year()
    )
}

fn main() {
    let output: String = conv_to_13(chrono::Local::now().date_naive());
    //let output: String = conv_to_13(NaiveDate::from_ymd(2026, 7, 1));
    println!("{}", output);
}

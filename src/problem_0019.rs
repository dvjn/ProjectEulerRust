//! Counting Sundays

use derive_more::{Add, Rem, Sub};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

pub fn solve() -> u64 {
    sundays_on_first_of_the_month(1901, 2000)
}

#[derive(Add, Rem, Sub, Clone, Debug, PartialEq, Copy, Hash, Eq)]
pub struct Day(u8);

lazy_static! {
    static ref SUNDAYS_ON_FIRST: Mutex<HashMap<(Day, bool), u64>> = Mutex::new(HashMap::new());
    static ref FIRST_DAY_OF_YEAR: Mutex<HashMap<u16, Day>> = {
        let mut m = HashMap::new();
        m.insert(1900, Day(1));
        Mutex::new(m)
    };
    static ref YEAR_DAY_OFFSET: Day = Day((365 % 7) as u8);
    static ref LEAP_YEAR_DAY_OFFSET: Day = Day((366 % 7) as u8);
    static ref YEAR_MONTH_DAYS_OFFSETS: Box<[Day]> =
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            .iter()
            .map(|day| Day(day % 7))
            .collect();
    static ref LEAP_YEAR_MONTH_DAYS_OFFSETS: Box<[Day]> =
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            .iter()
            .map(|day| Day(day % 7))
            .collect();
}

pub fn is_leap(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

macro_rules! if_leap {
    ($year:expr => $if_leap:expr , $if_non_leap:expr) => {
        if is_leap($year) {
            $if_leap
        } else {
            $if_non_leap
        }
    };
}

macro_rules! offset_days {
    (add => $base_day:expr, $offset_days:expr) => {
        ($base_day + $offset_days) % 7
    };
    (sub => $base_day:expr, $offset_days:expr) => {
        ($base_day + Day(7) - $offset_days) % 7
    };
}

pub fn sundays_on_first_of_the_month(from_year: u16, to_year: u16) -> u64 {
    (from_year..=to_year).map(get_sundays_on_first).sum()
}

pub fn get_first_day_of_year(year: u16) -> Day {
    let first_day_of_year_option = FIRST_DAY_OF_YEAR.lock().unwrap().get(&year).cloned();

    return match first_day_of_year_option {
        Some(day) => day,
        None => {
            let day = match year {
                (0..=1899) => {
                    let next_year = year + 1;
                    let next_first_day = get_first_day_of_year(next_year);
                    offset_days!(
                        sub =>
                        next_first_day,
                        if_leap!(next_year => *LEAP_YEAR_DAY_OFFSET, *YEAR_DAY_OFFSET)
                    )
                }
                1900 => Day(1),
                _ => {
                    let previous_year = year - 1;
                    let previous_first_day = get_first_day_of_year(previous_year);

                    offset_days!(
                        add =>
                        previous_first_day,
                        if_leap!(previous_year => *LEAP_YEAR_DAY_OFFSET, *YEAR_DAY_OFFSET)
                    )
                }
            };

            FIRST_DAY_OF_YEAR.lock().unwrap().insert(year, day);

            day
        }
    };
}

pub fn get_sundays_on_first(year: u16) -> u64 {
    let first_day_of_year = get_first_day_of_year(year);

    let sundays_on_first_option = SUNDAYS_ON_FIRST
        .lock()
        .unwrap()
        .get(&(first_day_of_year, is_leap(year)))
        .cloned();

    return match sundays_on_first_option {
        Some(sundays_on_first) => sundays_on_first,
        None => {
            let (_, sundays_on_first) =
                if_leap!(year => LEAP_YEAR_MONTH_DAYS_OFFSETS.iter(), YEAR_MONTH_DAYS_OFFSETS.iter())
                    .fold(
                        (first_day_of_year, 0),
                        |(first_day, sundays), &month_days_offset| {
                            (
                                offset_days!(add => first_day, month_days_offset),
                                sundays + if first_day == Day(0) { 1 } else { 0 },
                            )
                        },
                    );

            SUNDAYS_ON_FIRST
                .lock()
                .unwrap()
                .insert((first_day_of_year, is_leap(year)), sundays_on_first);

            sundays_on_first
        }
    } as u64;
}

#[cfg(test)]
mod tests {
    use super::sundays_on_first_of_the_month;

    #[test]
    fn given_problem() {
        assert_eq!(sundays_on_first_of_the_month(1901, 2000), 171);
    }
}

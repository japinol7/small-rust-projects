/// Determines if a given year is a leap year
///
/// A leap year is defined as one that is divisible by 4,
/// but is not otherwise divisible by 100 unless it is also divisible by 400.
pub fn is_leap_year(year: i32) -> bool {
    if year < 1 {
        return false;
    }
    if year % 400 == 0 {
        return true;
    }
    if year % 4 == 0 {
        return !(year % 100 == 0);
    }
    false
}

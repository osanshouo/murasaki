use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Date(u16, u8, u8);

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.0, self.1, self.2)
    }
}

impl FromStr for Date {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.trim().split("-");
        let year  = input.next().unwrap_or("0").parse()?;
        let month = input.next().unwrap_or("0").parse()?;
        let day   = input.next().unwrap_or("0").parse()?;

        Ok(Date(year, month, day))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "2020-04-01";
        let date = input.parse::<Date>().unwrap();
        assert_eq!(date, Date(2020, 4, 1));
        assert_eq!(format!("{}", date), input);

        let input = "1970-1";
        let date = input.parse::<Date>().unwrap();
        assert_eq!(date, Date(1970, 1, 0));
        assert_eq!(format!("{}", date), "1970-01-00");
    }
}

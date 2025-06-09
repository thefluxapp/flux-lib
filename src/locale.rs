use strum::{Display, EnumString};

#[derive(Debug, PartialEq, Display, EnumString)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum Locale {
    EN,
    RU,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{error::Error, locale::Locale};

    #[test]
    fn serialize() -> Result<(), Error> {
        assert_eq!(Locale::EN, Locale::from_str("en")?);
        assert_eq!(Locale::EN, Locale::from_str("EN")?);
        assert_eq!(Locale::RU, Locale::from_str("ru")?);
        assert_eq!(Locale::RU, Locale::from_str("RU")?);
        assert_eq!(String::from("ru"), Locale::RU.to_string());
        assert_eq!(String::from("en"), Locale::EN.to_string());

        Ok(())
    }
}

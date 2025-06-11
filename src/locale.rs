use serde::Deserialize;
use strum::{Display, EnumString};

#[derive(Debug, PartialEq, Display, EnumString, Deserialize)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
#[serde(rename_all = "snake_case")]
pub enum Locale {
    En,
    Ru,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde::Deserialize;

    use crate::{error::Error, locale::Locale};

    #[test]
    fn serialize() -> Result<(), Error> {
        assert_eq!(Locale::En, Locale::from_str("en")?);
        assert_eq!(Locale::En, Locale::from_str("EN")?);
        assert_eq!(Locale::Ru, Locale::from_str("ru")?);
        assert_eq!(Locale::Ru, Locale::from_str("RU")?);
        assert_eq!(Locale::Ru, Locale::from_str("Ru")?);
        assert_eq!(String::from("ru"), Locale::Ru.to_string());
        assert_eq!(String::from("en"), Locale::En.to_string());

        Ok(())
    }

    #[derive(Deserialize, Debug)]
    struct Foo {
        pub locale: Locale,
    }

    #[test]
    fn deserialize() -> Result<(), Error> {
        let data = r#"
        {
            "locale": "en"
        }"#;

        let foo: Foo = serde_json::from_str(data)?;

        assert_eq!(foo.locale, Locale::En);

        Ok(())
    }
}

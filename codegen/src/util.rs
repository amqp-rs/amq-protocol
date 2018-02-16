pub fn camel_case(name: &str) -> String {
    let mut new_word = true;
    name.chars().fold("".to_string(), |mut result, ch| {
        if ch == '-' || ch == '_' || ch == ' ' {
            new_word = true;
            result
        } else {
            result.push(if new_word { ch.to_ascii_uppercase() } else { ch });
            new_word = false;
            result
        }
    })
}

pub fn snake_case(name: &str) -> String {
    match name {
        "type"   => "type_".to_string(),
        "return" => "return_".to_string(),
        name     => {
            let mut new_word       = false;
            let mut last_was_upper = false;
            name.chars().fold("".to_string(), |mut result, ch| {
                if ch == '-' || ch == '_' || ch == ' ' {
                    new_word = true;
                    result
                } else {
                    let uppercase = ch.is_uppercase();
                    if new_word || (!last_was_upper && !result.is_empty() && uppercase) {
                        result.push('_');
                        new_word = false;
                    }
                    last_was_upper = uppercase;
                    result.push(if uppercase { ch.to_ascii_lowercase() } else { ch });
                    result
                }
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!(camel_case(""),                  "");
        assert_eq!(camel_case("foobar"),            "Foobar");
        assert_eq!(camel_case("FooBar"),            "FooBar");
        assert_eq!(camel_case("foo_bar"),           "FooBar");
        assert_eq!(camel_case("_foo__bar baz-zzz"), "FooBarBazZzz");
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(snake_case(""),               "");
        assert_eq!(snake_case("Foobar"),         "foobar");
        assert_eq!(snake_case("FooBar"),         "foo_bar");
        assert_eq!(snake_case("Foo-BarBaz_zzz"), "foo_bar_baz_zzz");
    }

    #[test]
    fn test_snake_case_uint() {
        /* special case: we want UInt to be converted as uint */
        assert_eq!(snake_case("UInt"),     "uint");
        assert_eq!(snake_case("LongUInt"), "long_uint");
    }
}

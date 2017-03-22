use std::ascii::AsciiExt;

pub fn camel_case(name: &str) -> String {
    let mut new_word: bool = true;
    name.chars().fold("".to_string(), |mut result, ch| {
        if ch == '-' || ch == '_' || ch == ' ' {
            new_word = true;
            result
        } else {
            result.push(if new_word { ch.to_ascii_uppercase() } else { ch.to_ascii_lowercase() });
            new_word = false;
            result
        }
    })
}

pub fn snake_case(name: &str) -> String {
    match name {
        "type"   => "amqp_type".to_string(),
        "return" => "amqp_return".to_string(),
        name     => {
            let mut new_word: bool = false;
            name.chars().fold("".to_string(), |mut result, ch| {
                if ch == '-' || ch == '_' || ch == ' ' {
                    new_word = true;
                    result
                } else {
                    if new_word || (result.len() > 0 && ch.is_uppercase()) {
                        result.push('_');
                        new_word = false;
                    }
                    result.push(ch.to_ascii_lowercase());
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
        assert_eq!(camel_case(""), "");
        assert_eq!(camel_case("foobar"), "Foobar");
        assert_eq!(camel_case("foo_bar"), "FooBar");
        assert_eq!(camel_case("_foo__bar baz-zzz"), "FooBarBazZzz");
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(snake_case(""), "");
        assert_eq!(snake_case("Foobar"), "foobar");
        assert_eq!(snake_case("FooBar"), "foo_bar");
        assert_eq!(snake_case("Foo-BarBaz_zzz"), "foo_bar_baz_zzz");
    }
}

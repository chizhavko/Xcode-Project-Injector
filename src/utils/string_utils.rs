pub fn string_slice_from_pattern<'a>(from: &'a str, to: &'a str, line: &'a str) -> &'a str {
    let start_position = line.find(from);

    if start_position.is_some() {
        let start_position = start_position.unwrap() + from.len();
        let line = &line[start_position..];
        let end_position = line.find(to).unwrap_or_default();
        return &line[..end_position];
    }

    return "";
}

pub fn string_slice_from_start<'a>(to: &'a str, line: &'a str) -> &'a str {
    let end_bytes = line.find(to).unwrap_or(line.len());
    &line[0..end_bytes]
}

#[cfg(test)]
mod test {
    use crate::utils::string_utils::*;


    #[test]
    fn test_string_slice_from_pattern() {
        {
            let string = "hello_world";
            let result = string_slice_from_pattern("h", "_", string);
            assert_eq!(result, "ello");
            assert_ne!(result, "");
        }
        {
            let string = "hello_world";
            let result = string_slice_from_pattern("_", "o", string);
            assert_eq!(result, "w");
            assert_ne!(result, "");
        }

        {
            let string = "hello_world";
            let result = string_slice_from_pattern("_", "!", string);
            assert_eq!(result, "");
        }
    }

    #[test]
    fn test_string_slice_from_start() {
        {
            let string = "hello_world";
            let result = string_slice_from_start("_", string);
            assert_eq!(result, "hello");
            assert_ne!(result, "hello_");
            assert_ne!(result, "hello_world");
            assert_ne!(result, "");
        }
        {
            let string = "hello_world";
            let result = string_slice_from_start("!", string);
            assert_eq!(result, "hello_world");
        }
    }
}
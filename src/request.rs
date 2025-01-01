use std::any::Any;

use serde_json::Value;

use crate::constants::HttpMethod;

pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub data: Option<Box<dyn Any>>,
}

impl Request {
    pub fn new(contents: String) -> Result<Self, String> {
        let lines: Vec<&str> = contents.lines().collect();
        let first_line = lines[0];
        let first_line: Vec<&str> = first_line.split_whitespace().collect();
        let method = first_line[0];
        let method = HttpMethod::get_by_string(method);
        let path = first_line[1];
        let path = String::from(path);

        if method == HttpMethod::NONE || path.len() == 0 {
            return Err("Error parsing incoming request".to_string());
        }

        let mut data = String::from("");
        let mut should_append = false;
        for line in lines {
            if should_append {
                data.push_str(line);
            } else {
                if line == "" {
                    should_append = true;
                }
            }
        }

        let data: Option<Box<dyn Any>> = match serde_json::from_str::<Value>(&data) {
            Ok(v) => Some(Box::new(v)),
            Err(_) => None,
        };

        Ok(Self { method, path, data })
    }
}

use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn read_template(file_path: &str) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(file_path).map_err(|e| e.into())
}

pub fn replace_placeholder(template: &str, values: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in values {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_replace_placeholder() {
        let template = "請填入 {col1} 和 {col2} 和 {col3}";
        let mut values = HashMap::new();
        values.insert("col1".to_string(), "1".to_string());
        values.insert("col2".to_string(), "2".to_string());
        values.insert("col3".to_string(), "3".to_string());

        let result = replace_placeholder(template, &values);
        assert_eq!(result, "請填入 1 和 2 和 3");
    }
}

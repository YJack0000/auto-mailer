use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub fn read_csv(file_path: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.clone();

    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut map = HashMap::new();

        for (index, field) in record.iter().enumerate() {
            let header = headers.get(index).unwrap_or_default();
            map.insert(header.to_string(), field.to_string());
        }

        records.push(map);
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_read_csv() -> Result<(), Box<dyn std::error::Error>> {
        // 模拟的 CSV 数据
        let data = "name,content\nJohn Doe,\"Hello, world!\"\nJane Smith,\"Goodbye, world!\"";

        // 使用临时文件进行测试
        let mut temp_file = tempfile::NamedTempFile::new()?;
        write!(temp_file, "{}", data)?;

        // 使用 read_csv 函数读取测试数据
        let records = read_csv(temp_file.path().to_str().unwrap())?;

        // 预期结果
        let expected = vec![
            [("name", "John Doe"), ("content", "Hello, world!")]
                .iter()
                .map(|&(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            [("name", "Jane Smith"), ("content", "Goodbye, world!")]
                .iter()
                .map(|&(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        ];

        // 验证结果是否与预期相符
        assert_eq!(records, expected);

        Ok(())
    }
}

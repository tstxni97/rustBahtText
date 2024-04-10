use std::collections::HashMap;
use lazy_static::lazy_static;

fn create_thai_numbers_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("0", "ศูนย์");
    map.insert("1", "หนึ่ง");
    map.insert("2", "สอง");
    map.insert("3", "สาม");
    map.insert("4", "สี่");
    map.insert("5", "ห้า");
    map.insert("6", "หก");
    map.insert("7", "เจ็ด");
    map.insert("8", "แปด");
    map.insert("9", "เก้า");
    map
}

lazy_static! {
    static ref THAI_NUMBERS_MAP: HashMap<&'static str, &'static str> = create_thai_numbers_map();
    static ref THAI_MULTIPLIER_LIST: Vec<&'static str> = vec!["", "สิบ", "ร้อย", "พัน", "หมื่น", "แสน", "ล้าน"];
}


fn split_million_sequence(seq: &str) -> Vec<String> {
    let mlength = 6;
    let seq = seq.chars().rev().collect::<String>();
    let mut splited_seq = Vec::new();
    for i in (0..seq.len()).step_by(mlength) {
        let end = i + mlength;
        let end = if end > seq.len() { seq.len() } else { end };
        let sub_seq = seq[i..end].chars().rev().collect::<String>();
        splited_seq.push(sub_seq);
    }
    splited_seq.into_iter().rev().collect()
}

fn convert_special_two_digits(int_string: &str) -> String {
    let mut baht_string = String::new();

    if int_string.chars().nth(0) == Some('2') {
        baht_string += "ยี่";
    } else if let Some(first_char) = int_string.chars().nth(0) {
        if  first_char != '0' {
            if let Some(thai_number) = THAI_NUMBERS_MAP.get(&first_char.to_string().as_str()) {
                baht_string += thai_number;
            }
        }
    }

    if int_string.chars().nth(0) != Some('0') {
        baht_string += "สิบ";
    }

    if let Some(second_char) = int_string.chars().nth(1) {
        if let Ok(first_digit) = second_char.to_string().parse::<i64>() {
            if first_digit == 0 {
                // do nothing
            } else if int_string.chars().nth(0) == Some('0') && first_digit > 0 {
                if let Some(thai_number) = THAI_NUMBERS_MAP.get(&second_char.to_string().as_str()) {
                    baht_string += thai_number;
                }
            } else if first_digit == 1 {
                baht_string += "เอ็ด";
            } else if let Some(thai_number) = THAI_NUMBERS_MAP.get(&second_char.to_string().as_str()) {
                baht_string += thai_number;
            }
        }
    }

    baht_string
}

fn convert_multiple_millions(int_string: &str) -> String {
    let mut baht_string = String::new();

    // integer part
    if int_string.len() == 1 {
        if let Some(thai_number) = THAI_NUMBERS_MAP.get(&int_string) {
            baht_string += thai_number;
        }
    } else if int_string.len() == 2 {
        baht_string += &convert_special_two_digits(int_string);
    } else if int_string.len() >= 3 && int_string.len() <= 6 {
        for number_index in 0..int_string.len() - 2 {
            if let Ok(multiplier) = int_string[number_index..number_index + 1].parse::<usize>() {
                if multiplier != 0 {
                    if let Some(thai_number) = THAI_NUMBERS_MAP.get(&int_string[number_index..number_index + 1]) {
                        baht_string += thai_number;
                    }
                    baht_string += THAI_MULTIPLIER_LIST[int_string.len() - number_index - 1];
                }
            } else {
                return String::new();
            }
        }
        baht_string += &convert_special_two_digits(&int_string[int_string.len() - 2..]);
    }

    baht_string
}


pub fn baht_text(mut input_float: f64) -> String {
    let mut baht_string = String::new();

    if input_float < 0.0 {
        input_float *= -1.0;
        baht_string += "ลบ";
    }

    let float_string = format!("{:.2}", input_float);
    let input_string: Vec<&str> = float_string.split('.').collect();

    let int_string = input_string[0];
    let decimal_string = input_string[1];

    let million_sequence_list = split_million_sequence(int_string);

    let mut million_index = million_sequence_list.len() - 1;

    for million_group_string in million_sequence_list {
        baht_string += &convert_multiple_millions(&million_group_string);

        if million_index > 0 {
            baht_string += "ล้าน";
            million_index -= 1;
        }
    }

    baht_string += "บาท";

    // decimal part
    let decimal_int = decimal_string.parse::<i32>().unwrap();
    if decimal_int == 0 {
        baht_string += "ถ้วน";
    } else {
        if decimal_string.len() == 1 {
            baht_string += THAI_NUMBERS_MAP[&decimal_int.to_string().as_str()];
        } else if decimal_string.len() == 2 {
            baht_string += &convert_special_two_digits(decimal_string);
        }

        baht_string += "สตางค์";
    }

    baht_string
}



mod tests {
    #[test]
    fn test_bath_text() {
        let bath_text = "หนึ่งร้อยบาทยี่สิบห้าสตางค์";
        assert_eq!(bath_text, crate::baht_text::baht_text::baht_text(100.25)); // Update the function call to use the crate
    }
}
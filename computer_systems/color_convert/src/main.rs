use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut result = vec![];
    for line in buffer.lines() {
        let words = line.split(" ");
        let mut line_result = vec![];

        for word in words {
            let first = word.chars().nth(0);

            let to_add = match first {
                Some('#') => hex_to_rgb(word),
                _ => String::from(word),
            };

            line_result.push(to_add);
        }
        result.push(line_result.join(" "))
    }

    // add to stdout. Improve to:
    // stdout.write_all(b"Hello, world!")?;
    // stdout.flush()?;
    println!("{}", result.join("\n"));

    Ok(())
}

fn hex_to_rgb(color: &str) -> String {
    // remove '#' and ';'
    let hex = &color[1..color.len() - 1].to_lowercase();

    match hex.len() {
        3 => {
            let mut hex_nums = vec![];

            for i in (0..hex.len()).step_by(1) {
                let current = String::from(&hex[i..i + 1]);
                let current = current.clone() + &current;

                let decimal = convert_hex_to_decimal(&current).to_string();
                hex_nums.push(decimal);
            }

            format!("rgb({});", hex_nums.join(" "))
        }
        4 => {
            let mut hex_nums = vec![];

            for i in (0..hex.len()).step_by(1) {
                let current = String::from(&hex[i..i + 1]);
                let current = current.clone() + &current;

                let decimal = convert_hex_to_decimal(&current);
                hex_nums.push(decimal);
            }

            let opacity = hex_nums.pop().unwrap();
            let opacity_in_percentage: f32 = (opacity as f32) / 255.0;

            format!(
                "rgba({} / {:.5});",
                hex_nums
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                opacity_in_percentage
            )
        }
        6 => {
            let mut hex_nums = vec![];

            for i in (0..hex.len()).step_by(2) {
                let current = &hex[i..i + 2];
                let decimal = convert_hex_to_decimal(current).to_string();
                hex_nums.push(decimal);
            }

            format!("rgb({});", hex_nums.join(" "))
        }
        8 => {
            let mut hex_nums = vec![];

            for i in (0..hex.len()).step_by(2) {
                let current = &hex[i..i + 2];
                let decimal = convert_hex_to_decimal(current);
                hex_nums.push(decimal);
            }

            let opacity = hex_nums.pop().unwrap();
            let opacity_in_percentage: f32 = (opacity as f32) / 255.0;

            format!(
                "rgba({} / {:.5});",
                hex_nums
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                opacity_in_percentage
            )
        }
        _ => panic!("not valid hex"),
    }
}

fn convert_hex_to_decimal(hex: &str) -> u8 {
    let mut power = hex.len() as u32;
    let mut sum = 0;

    for char in hex.chars() {
        power -= 1;
        sum += convert_hex_symbol(&char) * 16_u8.pow(power);
    }

    sum
}

fn convert_hex_symbol(hex: &char) -> u8 {
    match hex {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => panic!("Not supported hex"),
    }
}

#[cfg(test)]
mod tests {
    use crate::hex_to_rgb;

    #[test]
    fn it_converts_hex_to_rgb() {
        let rgb = hex_to_rgb("#00ff00");

        assert_eq!(rgb, "rgb(0 255 0)")
    }
}

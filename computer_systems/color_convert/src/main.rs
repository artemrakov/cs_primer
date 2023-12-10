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
            let doubled: String = hex
                .chars()
                .map(|char| format!("{}{}", char, char))
                .collect();
            convert_to_rgb(&doubled)
        }
        4 => {
            let doubled: String = hex
                .chars()
                .map(|char| format!("{}{}", char, char))
                .collect();
            convert_to_rgba(&doubled)
        }
        6 => convert_to_rgb(hex),
        8 => convert_to_rgba(hex),
        _ => panic!("not valid hex"),
    }
}

fn convert_to_rgb(hex: &str) -> String {
    let mut hex_nums = vec![];

    for i in (0..hex.len()).step_by(2) {
        let current = &hex[i..i + 2];
        let decimal = convert_hex_to_decimal(current).to_string();
        hex_nums.push(decimal);
    }

    format!("rgb({});", hex_nums.join(" "))
}

fn convert_to_rgba(hex: &str) -> String {
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

fn convert_hex_to_decimal(hex: &str) -> u8 {
    let mut power = hex.len() as u32;
    let mut sum = 0;

    for char in hex.chars() {
        power -= 1;
        sum += u8::from_str_radix(&char.to_string(), 16).unwrap() * 16_u8.pow(power);
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::hex_to_rgb;

    #[test]
    fn it_converts_6_hex_to_rgb() {
        let rgb = hex_to_rgb("#00ff00;");

        assert_eq!(rgb, "rgb(0 255 0);")
    }

    #[test]
    fn it_converts_8_hex_to_rgb() {
        let rgba = hex_to_rgb("#0000FFC0;");

        assert_eq!(rgba, "rgba(0 0 255 / 0.75294);")
    }

    #[test]
    fn it_converts_3_hex_to_rgb() {
        let rgba = hex_to_rgb("#123;");

        assert_eq!(rgba, "rgb(17 34 51);")
    }

    #[test]
    fn it_converts_4_hex_to_rgb() {
        let rgba = hex_to_rgb("#00f8;");

        assert_eq!(rgba, "rgba(0 0 255 / 0.53333);")
    }
}

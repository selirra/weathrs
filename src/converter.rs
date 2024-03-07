pub enum TemperatureFormat {
    Celsius,
    Fahrenheit,
    Kelvin,
}

pub fn convert_temperature(kelvin: f32, format: TemperatureFormat) -> (f32, String) {
    match format {
        TemperatureFormat::Celsius => ((kelvin - 273.15).round(), "°C".to_string()),
        TemperatureFormat::Fahrenheit => (((kelvin - 273.15) * 9.0 / 5.0 + 32.0).round(), "°F".to_string()),
        TemperatureFormat::Kelvin => (kelvin, "K".to_string()),
    }
}

pub fn to_titlecase(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            if let Some(first_char) = word.chars().next() {
                first_char.to_uppercase().collect::<String>() + &word[1..]
            } else {
                String::new()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

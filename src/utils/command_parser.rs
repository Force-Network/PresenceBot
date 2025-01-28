pub fn convert_text_to_time_length(time: &str) -> u64 {
    let mut timelengh = 0;
    let mut currentnumber = "".to_string();
    for char in time.chars() {
        if !char.is_numeric() {
            match char {
                'w' => timelengh += currentnumber.parse::<u64>().unwrap() * 604800,
                'd' => timelengh += currentnumber.parse::<u64>().unwrap() * 86400,
                'h' => timelengh += currentnumber.parse::<u64>().unwrap() * 3600,
                'm' => timelengh += currentnumber.parse::<u64>().unwrap() * 60,
                's' => timelengh += currentnumber.parse::<u64>().unwrap(),
                _ => panic!("Invalid time unit"),
            }
            currentnumber = "".to_string();
        }
        else {
            currentnumber.push(char);
        }
    }
    timelengh
}
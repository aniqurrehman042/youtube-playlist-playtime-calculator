use std::env::args;

use regex::Regex;

fn main() {
    let playlist_url = &args().collect::<Vec<String>>()[1];
    let response = reqwest::blocking::get(playlist_url).expect("API call failed");
    let response_html = response.text().expect("Failed to extract html from response");
    let time_text_regex = Regex::new(r"(\d+:)?\d+:\d+").expect("Failed to create regex");

    let mut total_playlist_seconds = 0;
    for time_text in time_text_regex.captures_iter(&response_html) {
        let mut time_text_string = String::from(&time_text[0]);
        let contains_hours = time_text_string.matches(":").count() > 1;

        // Extract seconds
        let seconds_index = time_text_string.rfind(":").unwrap() + 1;
        let seconds: u32 = time_text_string[seconds_index..].parse().unwrap();
        total_playlist_seconds += seconds;

        // Extract minutes
        time_text_string = String::from(&time_text_string[..seconds_index-1]);
        let minutes_index = if contains_hours { time_text_string.rfind(":").unwrap() + 1 } else { 0 };
        let minutes: u32 = time_text_string[minutes_index..].parse().unwrap();
        total_playlist_seconds += minutes * 60;

        // Extract hours if available
        if contains_hours {
            let hours: u32 = time_text_string[..minutes_index-1].parse().unwrap();
            total_playlist_seconds += hours * 60 * 60;
        }
    }

    // As youtube playlist contains two instances of each video's playtime, we divide by 2
    total_playlist_seconds /= 2;

    let playlist_hours = total_playlist_seconds / 60 / 60;
    let playlist_minutes = (total_playlist_seconds - (playlist_hours * 60 * 60)) / 60;
    let playlist_seconds = total_playlist_seconds - (playlist_hours * 60 * 60) - (playlist_minutes * 60);

    println!("Total playtime of playlist: {}:{:02}:{:02}", playlist_hours, playlist_minutes, playlist_seconds);
}

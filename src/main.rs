use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use clap::{App, Arg};
use owo_colors::OwoColorize;

fn main() {
    let matches = App::new("cwc")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Generates event messages for social media")
        .arg(
            Arg::with_name("location")
                .short('l')
                .long("location")
                .value_name("LOCATION")
                .help("Sets the location for the event (Twitter or Discord)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("topics")
                .short('t')
                .long("topics")
                .value_name("TOPICS")
                .help("Sets the topics for the event")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("url")
                .short('u')
                .long("url")
                .value_name("URL")
                .help("Sets the URL location for the event")
                .takes_value(true),
        )
        .get_matches();

    // Extract location, topics, and url from the arguments
    let location = matches.value_of("location").unwrap();
    let topics = matches.value_of("topics").unwrap_or("");
    let url = matches.value_of("url").unwrap_or("");

    // Calculate the session number
    let start_date = get_start_date();
    let now = Local::now().naive_local();
    let (session_num, next_event_day) = session_number(start_date, now);

    // Generate the Unix timestamp
    let session_time = match next_event_day.weekday() {
        chrono::Weekday::Tue => NaiveTime::from_hms_opt(12, 0, 0).expect("Invalid session time"),
        chrono::Weekday::Thu => NaiveTime::from_hms_opt(16, 0, 0).expect("Invalid session time"),
        _ => unreachable!(),
    };
    let session_datetime = NaiveDateTime::new(next_event_day.date(), session_time);
    let unix_timestamp = Local
        .from_local_datetime(&session_datetime)
        .unwrap()
        .timestamp();

    // Outputs:
    println!("{}", "\n------EVENT MESSAGE-------\n".red());

    // Format and print the event message
    let event_msg = fmt_msg_event(location, session_num as u32);
    println!("{}", event_msg.green());

    println!("{}", "\n------ANNOUNCEMENT--------\n".red());

    // Format and print the announce message
    let announcement_msg =
        fmt_msg_announcement(location, session_num as u32, unix_timestamp, topics, url);
    println!("{}", announcement_msg.purple());

    println!("{}", "\n------TWEET---------------\n".red());

    // Format and print the tweet message
    let tweet_msg = fmt_msg_tweet(location, session_num as u32, topics, url);
    println!("{}", tweet_msg.blue());

    println!("{}", "\n------CAFFEINE------------\n".red());

    // For CUDOS Caffeine
    if session_datetime.weekday() == chrono::Weekday::Tue {
        let caffeine_datetime = session_datetime + chrono::Duration::hours(2);

        let caffeine_timestamp = Local
            .from_local_datetime(&caffeine_datetime)
            .unwrap()
            .timestamp();

        let caffeine_number = session_num as u32 / 2 - 69;

        let caffeine_msg = fmt_msg_caffeine(caffeine_timestamp, caffeine_number);
        println!("{}", caffeine_msg.purple());
    }
}

fn get_start_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2022, 1, 6).unwrap()
}

fn session_number(start_date: NaiveDate, now: NaiveDateTime) -> (i32, NaiveDateTime) {
    let today = now.date();
    let days_passed = (today - start_date).num_days();
    let session_count = (days_passed / 7) * 2;
    let weekday = now.weekday();
    let mut session_num = session_count;
    let mut next_event_day = now;

    match weekday {
        chrono::Weekday::Tue => {
            if now.time() < NaiveTime::from_hms_opt(12, 0, 0).unwrap() {
                session_num += 2;
            } else {
                next_event_day += chrono::Duration::days(2);
                session_num += 3;
            }
        }
        chrono::Weekday::Thu => {
            if now.time() < NaiveTime::from_hms_opt(16, 0, 0).unwrap() {
                session_num += 1;
            } else {
                next_event_day += chrono::Duration::days(5);
                session_num += 2;
            }
        }
        chrono::Weekday::Mon => {
            next_event_day += chrono::Duration::days(1);
            session_num += 2;
        }
        chrono::Weekday::Wed => {
            next_event_day += chrono::Duration::days(1);
            session_num += 3;
        }
        chrono::Weekday::Fri => {
            next_event_day += chrono::Duration::days(4);
            session_num += 2;
        }
        chrono::Weekday::Sat => {
            next_event_day += chrono::Duration::days(3);
            session_num += 2;
        }
        chrono::Weekday::Sun => {
            next_event_day += chrono::Duration::days(2);
            session_num += 2;
        }
    }

    if next_event_day.weekday() == chrono::Weekday::Tue {
        next_event_day = next_event_day.date().and_hms_opt(12, 0, 0).unwrap();
    } else if next_event_day.weekday() == chrono::Weekday::Thu {
        next_event_day = next_event_day.date().and_hms_opt(16, 0, 0).unwrap();
    }

    (session_num as i32, next_event_day)
}

fn fmt_msg_event(location: &str, session_number: u32) -> String {
    let (platform, comms_info) = match location {
        "t" => (
            "on Twitter Spaces",
            "below the Twitter Space or request to speak",
        ),
        "d" => (
            "here on Discord in the `coffee-with-cudos` voice channel",
            "into the channel chat or unmute your microphone and ask them in the voice channel",
        ),
        _ => ("Unknown Platform", "Unknown Chat Info"),
    };

    format!(
        r#"Join us for ☕Coffee with CUDOS #{session_number} {platform}.

Bring your favourite drink and come have a chat during our office-hours!
Feel free to post questions {comms_info}.
Please note, nothing covered in these sessions constitutes financial advice. 🚀"#,
        session_number = session_number,
        platform = platform,
        comms_info = comms_info,
    )
}

fn fmt_msg_announcement(
    location: &str,
    session_number: u32,
    unix_timestamp: i64,
    topics: &str,
    url: &str,
) -> String {
    let platform = match location {
        "t" => "Twitter Spaces",
        "d" => "Discord",
        _ => "Unknown Platform",
    };
    format!(
        r#"Hey @In-The-Know!

☕`coffee-with-cudos` #{session_number} office hours will start <t:{unix_timestamp}:R> at <t:{unix_timestamp}:t>!
Swing by with a hot drink and join us for a chat on {platform}.

We will be chatting about Developers, {topics}, Validators, and anything else that comes up! You can find the event here:

{url}

Keen to see you there!! 🚀"#,
        session_number = session_number,
        unix_timestamp = unix_timestamp,
        platform = platform,
        topics = topics,
        url = url
    )
}

fn fmt_msg_tweet(location: &str, session_number: u32, topics: &str, url: &str) -> String {
    let platform = match location {
        "t" => "on Twitter Spaces",
        "d" => "on Discord in the `coffee-with-cudos` voice channel",
        _ => "Unknown Platform",
    };

    format!(
        r#"Join us for ☕️ Coffee with #CUDOS #{session_number} {platform}.
We will be chatting about Developers, {topics}, and Validators.
Swing by with a hot drink! ☕️
{url}"#,
        session_number = session_number,
        platform = platform,
        topics = topics,
        url = url,
    )
}

fn fmt_msg_caffeine(caffeine_timestamp: i64, caffeine_number: u32) -> String {
    format!(
        r#"Hey @Developers!

CUDOS Caffeine #{caffeine_number} is happening <t:{caffeine_timestamp}:R> at <t:{caffeine_timestamp}:t> in the #vibe-while-you-code channel with a member or two from the CUDOS team and available @Developer-Rangers!

Bring your technical questions and we can help you get set up!"#,
        caffeine_number = caffeine_number,
        caffeine_timestamp = caffeine_timestamp
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        let start_date = get_start_date();
        assert_eq!(
            session_number(
                start_date,
                NaiveDate::from_ymd_opt(2022, 1, 6)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap()
            ),
            (
                1,
                NaiveDate::from_ymd_opt(2022, 1, 6)
                    .unwrap()
                    .and_hms_opt(16, 0, 0)
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_monday() {
        let start_date = get_start_date();
        assert_eq!(
            session_number(
                start_date,
                NaiveDate::from_ymd_opt(2023, 3, 13)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap()
            ),
            (
                124,
                NaiveDate::from_ymd_opt(2023, 3, 14)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_tuesday_pre() {
        let start_date = get_start_date();
        assert_eq!(
            session_number(
                start_date,
                NaiveDate::from_ymd_opt(2023, 3, 14)
                    .unwrap()
                    .and_hms_opt(11, 59, 59)
                    .unwrap()
            ),
            (
                124,
                NaiveDate::from_ymd_opt(2023, 3, 14)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_tuesday_post() {
        let start_date = get_start_date();
        assert_eq!(
            session_number(
                start_date,
                NaiveDate::from_ymd_opt(2023, 3, 14)
                    .unwrap()
                    .and_hms_opt(12, 1, 0)
                    .unwrap()
            ),
            (
                125,
                NaiveDate::from_ymd_opt(2023, 3, 16)
                    .unwrap()
                    .and_hms_opt(16, 0, 0)
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_wednesday() {
        let start_date = get_start_date();
        assert_eq!(
            session_number(
                start_date,
                NaiveDate::from_ymd_opt(2023, 4, 12)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap()
            ),
            (
                133,
                NaiveDate::from_ymd_opt(2023, 4, 13)
                    .unwrap()
                    .and_hms_opt(16, 0, 0)
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_thursday_pre() {
        let start_date = get_start_date();
        assert_eq!(
            session_number(
                start_date,
                NaiveDate::from_ymd_opt(2023, 4, 13)
                    .unwrap()
                    .and_hms_opt(15, 59, 59)
                    .unwrap()
            ),
            (
                133,
                NaiveDate::from_ymd_opt(2023, 4, 13)
                    .unwrap()
                    .and_hms_opt(16, 0, 0)
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_thursday_post() {
        let start_date = get_start_date();
        assert_eq!(
            session_number(
                start_date,
                NaiveDate::from_ymd_opt(2023, 4, 13)
                    .unwrap()
                    .and_hms_opt(16, 01, 0)
                    .unwrap()
            ),
            (
                134,
                NaiveDate::from_ymd_opt(2023, 4, 18)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_friday() {
        let start_date = get_start_date();
        assert_eq!(
            session_number(
                start_date,
                NaiveDate::from_ymd_opt(2023, 5, 12)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap()
            ),
            (
                142,
                NaiveDate::from_ymd_opt(2023, 5, 16)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_saturday() {
        let start_date = get_start_date();
        assert_eq!(
            session_number(
                start_date,
                NaiveDate::from_ymd_opt(2023, 5, 13)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap()
            ),
            (
                142,
                NaiveDate::from_ymd_opt(2023, 5, 16)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_sunday() {
        let start_date = get_start_date();
        assert_eq!(
            session_number(
                start_date,
                NaiveDate::from_ymd_opt(2023, 5, 14)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap()
            ),
            (
                142,
                NaiveDate::from_ymd_opt(2023, 5, 16)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
            )
        );
    }
}

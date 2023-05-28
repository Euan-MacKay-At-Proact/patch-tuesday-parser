pub mod data_structure;
pub mod report_formatter;
pub mod web_request;

use chrono::{Datelike, Months, NaiveDate, Utc, Weekday};
use clap::Parser;
use report_formatter::generate_report;
use web_request::fetch_patch_tuesday_report;
//use std::fs;

#[derive(Parser)]
#[clap(
    author,  //This will take the value from Cargo.toml
    version, //This will take the value from Cargo.toml
    about      = "A handy CLI tool to help reduce the manual labour involved with creating Patch Tuesday reports",
    long_about = "patch-tuesday-parser is a CLI tool that fetches Patch Tuesday advisories, parses them, and generates \
                 announcement/meeting ready reports and Excell files with minimal user input"
)]
struct Cli {
    /// Which Patch Tuesday release to fetch; leaving blank defaults to latest
    date: Option<String>,

    /// Optional strftime format for the date; leaving blank defaults to %Y-%b (e.g. 2023-Jul)
    format: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // If the user passed a strftime arg, accept it, else assume default
    let format = match cli.format {
        Some(s) => s,
        None => String::from("%Y-%b"),
    };

    // If the user passed a date arg, accept it, otherwise assume the current date
    let date: NaiveDate = match cli.date {
        Some(s) => match NaiveDate::parse_from_str(s.as_str(), &format) {
            Ok(i) => i,
            Err(e) => {
                println!("{}\n ...going to fetch latest instead", e);
                Utc::now().date_naive()
            }
        },
        None => Utc::now().date_naive(),
    };

    let patch_tuesday_date: NaiveDate = get_patch_tuesday_date(date);
    let response = fetch_patch_tuesday_report(patch_tuesday_date)
        .await
        .unwrap();
    let report = generate_report(response);
    println!("{}\n{}", report.to_html(), report);
    //println!("Parsed {:#?}!", response.vulnerability)
}

fn get_patch_tuesday_date(date: NaiveDate) -> NaiveDate {
    let now = Utc::now().date_naive();
    let patch_tuesday_date =
        NaiveDate::from_weekday_of_month_opt(date.year(), date.month(), Weekday::Tue, 2).unwrap();

    if patch_tuesday_date > now {
        if (patch_tuesday_date.month() == now.month()) && (patch_tuesday_date.year() == now.year())
        {
            //We're making the assumption that the user is intending to fetch the latest patch tuesday report
            //This should only fire if the current month was assumed or specified and hasn't had a Patch Tuesday release yet
            let new_date = patch_tuesday_date
                .checked_sub_months(Months::new(1))
                .unwrap();
            get_patch_tuesday_date(new_date);
        } else {
            println!(
                "Invalid date:- Current technology cannot retrieve Patch Tuesdays from the future!\n\
                Recommendation:- Try a date between 2016-Jan and {}",
                now.format("%Y-%b")
            )
        }
    } else if date.year() < 2016 {
        print!(
            "Invalid date:- Patch Tuesday is a Microsoft tradition that started in 2016; there are no reports to fetch prior to Jan 2016\n\
            Recommendation:- Try a date between 2016-Jan and {}",
            now.format("%Y-%b")
        )
    }

    patch_tuesday_date
}

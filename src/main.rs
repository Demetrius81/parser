use std::{
    fs::File,
    time::{SystemTime, UNIX_EPOCH},
};

use csv::Writer;
use reqwest::blocking::*;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

const CUSTOM_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";

fn main() {
    let url: &str = &format!(
        "https://finance.yahoo.com/quote/BTC-USD/history/?period1=1410912000&period2={}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    // println!("{}", url);

    let response = Client::new()
        .get(url)
        .header(USER_AGENT, CUSTOM_USER_AGENT)
        .send()
        .unwrap()
        .text()
        .unwrap();

    // println!("{}", response);

    let mut rows_data = Html::parse_document(&response)
        .select(&Selector::parse("tr.yf-j5d1ld").unwrap())
        .map(|el| {
            el.text()
                .filter_map(|s| {
                    let s = s.trim().replace(",", "");
                    if !s.is_empty() {
                        Some(s)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows_data_first = rows_data.get_mut(0).unwrap();
    rows_data_first.remove(7);
    rows_data_first.remove(5);

    let mut writer = Writer::from_writer(File::create("data.csv").unwrap());

    for row in &rows_data {
        println!("{:?}", row);
        writer.write_record(row).unwrap();
    }

    writer.flush().unwrap();
}

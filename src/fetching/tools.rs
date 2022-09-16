use std::fmt::Display;

use chrono::{Date, Utc, Month, TimeZone};
use log::error;
use reqwest::IntoUrl;
use scraper::{Selector, Html};

use crate::{popis_error::{PopisError, Result}, domain::Url};

pub fn selector(s: &str) -> Selector {
    Selector::parse(s).unwrap()
}

pub fn seatings_url(cadence: u32) -> String {
    format!(r"https://www.sejm.gov.pl/sejm9.nsf/agent.xsp?symbol=posglos&NrKadencji={cadence}")
}

pub fn verify_document(doc: &Html) -> Result<()> {
    if !doc.errors.is_empty() {
        error!("Error parsing doc: {}", doc.errors.iter().fold(String::default(), |mut a,b| { a.push_str(b); a }));
    }
    Ok(())
}

pub fn parse_err(s: &str) -> PopisError {
    PopisError::HtmlParsing(s.to_owned())
}

pub async fn fetch_document<U: IntoUrl + Display + Clone>(url: U) -> Result<Html> {
    let html = reqwest::get(url.clone())
        .await?
        .text()
        .await?;
    let html = html.replace('_', "x");
    let document = Html::parse_document(&html);
    match verify_document(&document) {
        Err(PopisError::HtmlParsing(str)) => Err(PopisError::HtmlParsing(format!("Error parsing {url}: {str}"))),
        _ => Ok(document),
    }
}

pub fn map_date(polish_date: &str) -> Option<Date<Utc>> {
    let mut dmy = polish_date.trim().split(' ');
    let day = dmy.next()?.parse().ok()?;
    let month = match dmy.next()? {
        "stycznia" => Month::January,
        "lutego" => Month::February,
        "marca" => Month::March,
        "kwietnia" => Month::April,
        "maja" => Month::May,
        "czerwca" => Month::June,
        "lipca" => Month::July,
        "sierpnia" => Month::August,
        "września" => Month::September,
        "października" => Month::October,
        "listopada" => Month::November,
        "grudnia" => Month::December,
        _ => panic!(),
    };
    let year = dmy.next()?.parse().ok()?;
    Some(Utc.ymd(year, month.number_from_month(), day))
}

pub fn url_from_link<S: std::fmt::Display>(href_link: S) -> Option<Url> {
    Url::try_new(format!(r"https://www.sejm.gov.pl/sejm9.nsf/{}", href_link))
}
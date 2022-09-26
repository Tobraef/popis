use std::fmt::Display;

use chrono::{DateTime, Month, NaiveDate, Utc};
use log::{debug, error};
use reqwest::IntoUrl;
use scraper::{ElementRef, Html, Selector};

use crate::popis_error::{PopisError, Result};

use super::data::Url;

pub trait Selectible {
    fn child(&self, selector: &str) -> Option<ElementRef>;
}

impl Selectible for ElementRef<'_> {
    fn child(&self, s: &str) -> Option<ElementRef> {
        self.select(&selector(s)).next()
    }
}

pub fn selector(s: &str) -> Selector {
    Selector::parse(s).unwrap()
}

pub fn seatings_url(cadence: u32) -> Url {
    Url::try_new(format!(
        r"https://www.sejm.gov.pl/sejm9.nsf/agent.xsp?symbol=posglos&NrKadencji={cadence}"
    ))
    .unwrap()
}

pub fn verify_document(doc: &Html) -> Result<()> {
    if !doc.errors.is_empty() {
        debug!(
            "Error parsing doc: {}",
            doc.errors.iter().fold(String::default(), |mut a, b| {
                a.push_str(b);
                a
            })
        );
    }
    Ok(())
}

pub fn parse_err(s: &str) -> PopisError {
    PopisError::HtmlParsing(s.to_owned())
}

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn fetch_document<U: IntoUrl + Display + Clone>(url: U) -> Result<Html> {
    let html = CLIENT.get(url.clone()).send().await?.text().await?;
    let html = html.replace('_', "x");
    let document = Html::parse_document(&html);
    match verify_document(&document) {
        Err(PopisError::HtmlParsing(str)) => Err(PopisError::HtmlParsing(format!(
            "Error parsing {url}: {str}"
        ))),
        _ => Ok(document),
    }
}

pub fn map_date(polish_date: &str) -> Option<DateTime<Utc>> {
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
        m => {
            error!("Received month: {m}, couldn't parse it into any polish month");
            return None;
        }
    };
    let year = dmy.next()?.parse().ok()?;
    Some(DateTime::<Utc>::from_utc(
        NaiveDate::from_ymd(year, month.number_from_month(), day).and_hms(0, 0, 0),
        Utc,
    ))
}

pub fn url_from_link<S: std::fmt::Display>(href_link: S) -> Option<Url> {
    Url::try_new(format!(r"https://www.sejm.gov.pl/sejm9.nsf/{}", href_link))
}

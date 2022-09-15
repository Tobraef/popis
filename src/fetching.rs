use chrono::{Date, Utc, Month, prelude::*};
use log::info;
use reqwest::IntoUrl;
use scraper::{Html, Selector, ElementRef};

use crate::{domain::{Seating, SeatingList, Url}, popis_error::{Result, PopisError}};

const MAIN_PAGE_DIV_WITH_TABLE: &str = "div#contentBody";
const MAIN_PAGE_TABLE_CONTENT: &str = "tbody";

fn seatings_url(cadence: u32) -> String {
    format!(r"https://www.sejm.gov.pl/sejm9.nsf/agent.xsp?symbol=posglos&NrKadencji={cadence}")
}

fn verify_document(doc: &Html) -> Result<()> {
    if let Some(e) = doc.errors.first() {
        Err(PopisError::HtmlParsing(e.to_string()))
    } else {
        Ok(())
    }
}

fn parse_err(s: &str) -> PopisError {
    PopisError::HtmlParsing(s.to_owned())
}

async fn fetch_document<U: IntoUrl>(url: U) -> Result<Html> {
    let html = reqwest::get(url)
        .await?
        .text()
        .await?;
    let document = Html::parse_document(&html);
    verify_document(&document)?;
    Ok(document)
}

fn map_date(polish_date: &str) -> Option<Date<Utc>> {
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

fn parse_main_row(row: ElementRef) -> Option<Seating> {
    let td_selector = Selector::parse("td").unwrap();
    let mut children = row.select(&td_selector);
    let number = children.next()?.inner_html().parse().ok()?;
    let link_with_date = children.next()?.select(&Selector::parse("a").unwrap()).next()?;
    let link = Url::try_new(format!(r"https://www.sejm.gov.pl/sejm9.nsf/{}", link_with_date.value().attr("href")?.to_owned()))?;
    let date = map_date(&link_with_date.inner_html())?;
    Some(Seating::new(link, date, number))
}

fn parse_main_table(table: ElementRef) -> Result<SeatingList> {
    let content = table.select(&Selector::parse(MAIN_PAGE_TABLE_CONTENT).unwrap())
        .next()
        .ok_or(parse_err("Didn't find content in table"))?;
    let list: Vec<_> = content
        .select(&Selector::parse("tr").unwrap())
        .filter_map(|row| parse_main_row(row))
        .collect();
    if list.is_empty() {
        Err(parse_err("Didn't find any seatings in the list"))
    } else {
        Ok(SeatingList::new(list))
    }
}

pub async fn fetch_seatings(cadence: u32) -> Result<SeatingList> {
    let url = seatings_url(cadence);
    let document = fetch_document(url)
        .await?;
    let table = document.select(&Selector::parse(MAIN_PAGE_DIV_WITH_TABLE).unwrap())
        .next()
        .ok_or(parse_err("Didn't find table with seatings on the main page"))?;
    parse_main_table(table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use scraper::Selector;

    #[test]
    fn selectors_test() {
        Selector::parse(MAIN_PAGE_DIV_WITH_TABLE).unwrap();
        Selector::parse(MAIN_PAGE_TABLE_CONTENT).unwrap();
    }
}
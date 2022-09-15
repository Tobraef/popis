use log::info;
use scraper::{ElementRef, Html};

use super::tools::*;
use crate::{domain::{Seating, SeatingList, Voting}, popis_error::{Result}};

const MAIN_PAGE_DIV_WITH_TABLE: &str = "div#contentBody";
const MAIN_PAGE_TABLE_CONTENT: &str = "tbody";

fn parse_main_row(row: ElementRef) -> Option<Seating> {
    let td_selector = selector("td");
    let mut children = row.select(&td_selector);
    let number = children.next()?.inner_html().parse().ok()?;
    let link_with_date = children.next()?.select(&selector("a")).next()?;
    let link = url_from_link(link_with_date.value().attr("href")?)?;
    let date = map_date(&link_with_date.inner_html())?;
    Some(Seating::new(link, date, number))
}

fn parse_voting_row(row: ElementRef) -> Option<Voting> {
    let td_selector = selector("td");
    let mut cells = row.select(&td_selector);
    let number_with_link = cells.next()?.select(&selector("a")).next()?;
    let link = url_from_link(number_with_link.value().attr("href")?)?;
    info!("Got link {}", link.0);
    let number = number_with_link.inner_html().parse().ok()?;
    let _hour = cells.next();
    info!("trying description");
    let description_node = cells.next()?;
    let mut description = description_node.text().next()?.to_owned();
    let second_part = description_node.select(&selector("a"))
        .next()?
        .inner_html();
    description.push_str(&second_part);
    Some(Voting::new(link, number, description))
}

pub fn parse_seatings(document: Html) -> Result<SeatingList> {
    let table = document.select(&selector(MAIN_PAGE_DIV_WITH_TABLE))
        .next()
        .ok_or(parse_err("Didn't find table with seatings on the main page"))?;
    let content = table.select(&selector(MAIN_PAGE_TABLE_CONTENT))
        .next()
        .ok_or(parse_err("Didn't find content in table"))?;
    let list: Vec<_> = content
        .select(&selector("tr"))
        .filter_map(|row| parse_main_row(row))
        .collect();
    if list.is_empty() {
        Err(parse_err("Didn't find any seatings in the list"))
    } else {
        Ok(SeatingList::new(list))
    }
}

pub fn parse_votings(document: Html) -> Result<Vec<Voting>> {
    // it so happens those are equivalent to main page ones
    let tbody = document.select(&selector(MAIN_PAGE_DIV_WITH_TABLE))
        .next()
        .map(|div| div.select(&selector(MAIN_PAGE_TABLE_CONTENT)).next())
        .flatten()
        .ok_or(parse_err("Didn't find table in seating"))?;
    let table: Vec<_> = tbody.select(&selector("tr"))
        .filter_map(parse_voting_row)
        .collect();
    if table.is_empty() {
        Err(parse_err("Didn't find any votings in the seating"))
    } else {
        Ok(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selectors_test() {
        selector(MAIN_PAGE_DIV_WITH_TABLE);
        selector(MAIN_PAGE_TABLE_CONTENT);
    }
}
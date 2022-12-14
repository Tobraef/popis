use scraper::{ElementRef, Html};

use super::{
    data::{LoadableSeatingHeader, LoadableVoting},
    tools::*,
};
use crate::{
    domain::{Party, PartyVote, SeatingHeader, Vote, VotingHeader, VotingResult},
    popis_error::Result,
};

const MAIN_PAGE_DIV_WITH_TABLE: &str = "div#contentBody";
const MAIN_PAGE_TABLE_CONTENT: &str = "tbody";

pub fn parse_seating_list(document: &Html) -> Result<Vec<LoadableSeatingHeader>> {
    let table = document
        .select(&selector(MAIN_PAGE_DIV_WITH_TABLE))
        .next()
        .ok_or_else(|| parse_err("Didn't find table with seatings on the main page"))?;
    let content = table
        .select(&selector(MAIN_PAGE_TABLE_CONTENT))
        .next()
        .ok_or_else(|| parse_err("Didn't find content in table"))?;
    let list: Vec<_> = content
        .select(&selector("tr"))
        .filter_map(parse_main_row)
        .collect();
    if list.is_empty() {
        Err(parse_err("Didn't find any seatings in the list"))
    } else {
        Ok(list)
    }
}

fn parse_main_row(row: ElementRef) -> Option<LoadableSeatingHeader> {
    let td_selector = selector("td");
    let mut children = row.select(&td_selector);
    let number = children.next()?.inner_html().parse().ok()?;
    let link_with_date = children.next()?.select(&selector("a")).next()?;
    let link = url_from_link(link_with_date.value().attr("href")?)?;
    let date = map_date(&link_with_date.inner_html())?;
    let header = SeatingHeader::new(number, date);
    Some(LoadableSeatingHeader::new(header, link))
}

pub(super) fn parse_votings(document: &Html) -> Result<Vec<LoadableVoting>> {
    // it so happens those are equivalent to main page ones
    let tbody = document
        .select(&selector(MAIN_PAGE_DIV_WITH_TABLE))
        .next()
        .and_then(|div| div.select(&selector(MAIN_PAGE_TABLE_CONTENT)).next())
        .ok_or_else(|| parse_err("Didn't find table in seating"))?;
    let table: Vec<_> = tbody
        .select(&selector("tr"))
        .filter_map(parse_voting_row)
        .collect();
    if table.is_empty() {
        Err(parse_err("Didn't find any votings in the seating"))
    } else {
        Ok(table)
    }
}

fn parse_voting_row(row: ElementRef) -> Option<LoadableVoting> {
    let td_selector = selector("td");
    let mut cells = row.select(&td_selector);
    let number_with_link = cells.next()?.select(&selector("a")).next()?;
    let link = url_from_link(number_with_link.value().attr("href")?)?;
    let number = number_with_link.inner_html().parse().ok()?;
    let _hour = cells.next();
    let description_node = cells.next()?;
    let mut description = description_node.text().next()?.to_owned();
    let second_part = description_node.select(&selector("a")).next()?.inner_html();
    description.push_str(&second_part);
    Some(LoadableVoting::new(
        VotingHeader::new(number, description),
        link,
    ))
}

fn parse_voting_result_row(row: ElementRef) -> Option<PartyVote> {
    let td_selector = selector("td");
    let mut cells = row.select(&td_selector);
    let party = cells.next()?.child("a")?.child("strong")?.inner_html();
    let _total_party_members = cells.next();
    let _total_party_votes = cells.next();
    // those are in <strong></strong>
    let get_votes = |e: ElementRef| {
        e.child("a")
            .and_then(|e| {
                e.child("strong")
                    .map(|e| e.inner_html().parse().unwrap_or(0))
            })
            .unwrap_or(0)
    };
    let votes_for = get_votes(cells.next()?);
    let votes_against = get_votes(cells.next()?);
    let votes_held = get_votes(cells.next()?);
    Some(PartyVote::new(
        Party::new(party),
        Vote::from_votes(votes_for, votes_against, votes_held),
    ))
}

pub fn parse_voting_result(document: &Html) -> Result<VotingResult> {
    let tbody = document
        .select(&selector("#main"))
        .next()
        .and_then(|div| div.select(&selector("#contentBody")).next())
        .and_then(|div| div.select(&selector("table.kluby")).next())
        .and_then(|table| table.select(&selector("tbody")).next())
        .ok_or_else(|| parse_err("Didn't find table body in voting results"))?;
    let table: Vec<_> = tbody
        .select(&selector("tr"))
        .filter_map(parse_voting_result_row)
        .collect();
    if table.is_empty() {
        Err(parse_err("Didn't find any parties votes' in the voting"))
    } else {
        Ok(VotingResult::new(table))
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

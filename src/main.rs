extern crate ncurses;
extern crate redis;
extern crate reqwest;
extern crate term_size;
extern crate termion;
mod net;
mod news;


use rss::Channel;
use std::io::BufReader;
use std::io::{stdin, stdout, Write};

use crate::net::get_and_cache_url;
use crate::news::print_headlines;
use crate::news::print_article;
use termion::raw::IntoRawMode;
use termion::input::TermRead;

const NYT_RSS: &str = "https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml";

fn main() {
    let (x, y) = term_size::dimensions().unwrap();

    let mut current: Option<String> = None;

    loop {
        match run((x, y - 2), current) {
            Some(n) => {
                current = Some(n)
            },
            None => current = None
        }
    }
}

fn run (size: (usize, usize), current: Option<String>) -> Option<String> {
    match current {
        Some(n) => {
            article_show(n, size);
            None
        },

        None => {
            match article_select(size) {
                Some(s) => Some(s),
                None => None
            }
        }
    }
}

fn article_show(url: String, dimensions: (usize, usize)) -> Option<String> {
    let data = get_and_cache_url(&url, &(60 * 60 * 60));
    let mut c = 0;
    let mut _stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = stdin().keys();

    loop {
        print_article(&data, &dimensions, c);

        let input = stdin.next();
        match input.unwrap().unwrap() {
            termion::event::Key::Char('j') => c += 1,
            termion::event::Key::Char('k') => c -= 1,
            termion::event::Key::Char('q') => {
                std::process::exit(0);
            },
            _ => { break; }
        }
    }

    return None;
}

fn article_select(dimensions: (usize, usize)) -> Option<String> {
    let t = get_rss(NYT_RSS);
    let mut c = 0;

    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut stdin = stdin().keys();

    loop {
        print!(" {}\n\r", termion::clear::All);
        print_headlines(&t, &dimensions, c);
        stdout.flush().unwrap();

        let input = stdin.next();
        match input.unwrap().unwrap() {
            termion::event::Key::Char('g') => {
                c = 0
            },
            termion::event::Key::Char('j') => {
                c += 1
            },
            termion::event::Key::Char('k') => {
                if c != 0 {
                    c -= 1
                }
            },
            termion::event::Key::Char('q') => {
                std::process::exit(0);
            },
            termion::event::Key::Char('o') => {
                let item = t.items().get(c).unwrap();
                let link = Some(String::from(item.link().unwrap()));
                return link;
            },
            _ => { break; }
        }
    }

    return None;
}

fn get_rss(url: &str) -> Channel {
    let data = get_and_cache_url(url, &(60 * 60));

    let channel = Channel::read_from(
        BufReader::new(data.as_bytes())
    ).unwrap();
    channel
}


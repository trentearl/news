extern crate ncurses;

use scraper::Html;
use scraper::Selector;
use rss::Channel;
use textwrap::fill;
use colored::*;

pub fn print_headlines(channel: &Channel, (x, y): &(usize, usize), offset: usize) {
    let mut cur: usize = 0;
    for item in channel.items().iter().skip(offset) {
        let title = item.title().unwrap();
        let description = match item.description() {
            Some(s) => s,
            None => ""
        };

        let description_filled = fill(description, x - 10).replace("\n", "\n\r");

        let fcur = cur + description_filled.lines().count() + 2;
        if fcur > *y { break; }
        cur = fcur;

        print!("{}\n\r", title.green());
        print!("{}\n\r\n", description_filled);
    }

    let size = (*y as i32 - cur as i32) - 1 + 2;

    for _i in 0..size {
        print!("\n\r");
    }
}

pub fn print_article(article_string: &String, (_x, y): &(usize, usize), offset: usize) {
    let article = Html::parse_document(
        &article_string
    );

    let body_selector = Selector::parse(
        "section[name=articleBody] p"
    ).unwrap();

    let mut string = String::from("");

    for el in article.select(&body_selector) {
        let p = el.text().collect::<Vec<_>>().join("");
        let filled_text = fill(&p, 80)
            .replace("\n", "\n\r");

        for text in filled_text.lines() {
            string += &format!("{}\n\r", text).to_string();

        }

        string += &format!("\n\r").to_string();
    }


    let mut i = 0usize;
    let mut p = 0usize;

    for line in string.lines().skip(offset) {
        if line == "" {
            p += 1;
        }

        if p % 2 == 0 {
            println!("{}", line.white());
        } else {
            println!("{}", line.cyan());
        }
        i += 1;
        if i > *y {
            break;
        }
    }
}


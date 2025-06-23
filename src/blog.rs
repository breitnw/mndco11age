use crate::database::format_date;
use chrono::Utc;
use pulldown_cmark::{html, Event, Parser, Tag};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Article {
    pub title: String,
    pub tagline: String,
    pub timestamp: i64,
    pub date: String,
    pub location: String,
    pub preview: String,
    pub html: String,
    pub markdown: String,
}

impl Article {
    pub fn new(title: &str, tagline: &str, markdown: &str) -> Self {
        // Record a timestamp based on the current time
        let timestamp = Utc::now().timestamp();

        // Generate HTML from the markdown
        let mut html = String::new();
        html::push_html(&mut html, Parser::new(markdown));

        // Generate the link location by using only alphanumeric characters and dashes
        let location: String = title
            .to_ascii_lowercase()
            .chars()
            .filter_map(|c| match c {
                'a'..='z' | '0'..='9' => Some(c),
                '_' | '-' | ' ' => Some('-'),
                _ => None,
            })
            .collect();

        // Generate a preview from the first 50 words of the markdown
        let mut preview = String::with_capacity(markdown.len() * 3 / 2);
        for event in Parser::new(markdown) {
            match &event {
                Event::Text(text) => preview.push_str(text),
                Event::Code(code) => {
                    preview.push('`');
                    preview.push_str(code);
                    preview.push('`');
                }
                Event::HardBreak | Event::SoftBreak => preview.push(' '),
                Event::Start(Tag::CodeBlock(..)) => break,
                Event::End(Tag::Paragraph) => break,
                Event::End(Tag::Heading(..)) => break,
                _ => (),
            }
        }
        preview = preview.split(" ").take(50).collect::<Vec<&str>>().join(" ");
        preview.push_str("...");

        Article {
            title: title.to_string(),
            tagline: tagline.to_string(),
            timestamp,
            date: format_date(timestamp),
            location,
            preview,
            html,
            markdown: markdown.to_string(),
        }
    }
}

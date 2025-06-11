[![github]](https://github.com/fuderis/rs-web-parser)&ensp;
[![crates-io]](https://crates.io/crates/web-parser)&ensp;
[![docs-rs]](https://docs.rs/web-parser)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# WebSites Parser

This website parser library allows asynchronous fetching and extracting data from web pages in multiple formats.

## Key features include:
* Reading an HTML document from a given URL with a randomized user agent (User::random()).
* Selecting elements via CSS selectors and retrieving their attributes and contents.
* Fetching the entire page as plain text.
* Fetching and parsing page content as JSON, with integration for handling it via serde_json.

This tool is well-suited for web scraping and data extraction tasks, supporting flexible parsing of HTML, plain text, and JSON, thereby enabling comprehensive data retrieval from various web sources.


## Examples:

```rust
use web_parser::{ prelude::*, User, Document };

#[tokio::main]
async fn main() -> Result<()> {
    // _____ READ PAGE AS HTML DOCUMENT: _____
    
    // read website page:
    let mut doc = Document::read("https://example.com/", User::random()).await?;

    // select title:
    let title = doc.select("h1")?.expect("No elements found");
    println!("Title: '{}'", title.text());

    // select descriptions:
    let mut descrs = doc.select_all("p")?.expect("No elements found");
    while let Some(descr) = descrs.next() {
        println!("Description: '{}'", descr.text())
    }


    // _____ READ PAGE AS SIMPLE TEXT: _______

    let text: String = Document::text("https://example.com/", User::random()).await?;
    println!("Text: {text}");


    // _____ READ PAGE AS JSON: ______________

    let json: serde_json::Value = Document::json("https://example.com/", User::random()).await?.expect("Failed to parse JSON");
    println!("Json: {json}");

    Ok(())
}
```

## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.

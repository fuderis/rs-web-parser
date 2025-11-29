[![github]](https://github.com/fuderis/rs-web-parser)&ensp;
[![crates-io]](https://crates.io/crates/web-parser)&ensp;
[![docs-rs]](https://docs.rs/web-parser)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Web Parser + Search

This web page parser library allows asynchronous fetching and extracting of data from web pages in multiple formats.

* Asynchronous web search using the search engines [Google, Bing, Duck, Ecosia, Yahoo, Wiki] with domain blacklisting (feature `search`).
* You can also create a custom search engine by using the `SearchEngine` trait (feature `search`).
* Reading an HTML document from a URL with a randomized user-agent (User::random()).
* Selecting elements by CSS selectors and retrieving their attributes and content.
* Fetching the full page as plain text.
* Fetching and parsing page content as JSON with [serde_json](https://docs.rs/serde_json/) support.

This tool is well-suited for web scraping and data extraction tasks, offering flexible parsing of HTML, plain text, and JSON to enable comprehensive data gathering from various web sources.

## Examples:

### Web Search (feature: 'search'):
> Requires the [chromedriver](https://developer.chrome.com/docs/chromedriver/downloads) tool installed!
```rust
use web_parser::prelude::*;
use macron::path;

#[tokio::main]
async fn main() -> Result<()> {
    // WEB SEARCH:

    let chrome_path = path!("bin/chromedriver/chromedriver.exe");
    let session_path = path!("%/ChromeDriver/WebSearch");
    
    // start search engine:
    let mut engine = SearchEngine::<Duck>::new(
        chrome_path,
        Some(session_path),
        false,
    ).await?;

    println!("Searching results..");

    // send search query:
    let results = engine.search(
        "Rust (programming language)",  // query
        &["support.google.com", "youtube.com"],  // black list
        1000  // sleep in millis
    ).await;
    
    // handle search results:
    match results {
        Ok(cites) => {
            println!("Result cites list: {:#?}", cites.get_urls());

            /*
            println!("Reading result pages..");
            let contents = cites.read(
                5,  // cites count to read
                &[  // tag name black list
                    "header", "footer", "style", "script", "noscript",
                    "iframe", "button", "img", "svg"
                ]
            ).await?;

            println!("Results: {contents:#?}");
            */
        }
        Err(e) => eprintln!("Search error: {e}")
    }

    // stop search engine:
    engine.stop().await?;

    Ok(())
}
```

### Web Parsing:
```rust
use web_parser::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // READ PAGE AS HTML DOCUMENT:
    
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

    // READ PAGE AS PLAIN TEXT:

    let text: String = Document::text("https://example.com/", User::random()).await?;
    println!("Text: {text}");

    // READ PAGE AS JSON:

    let json: serde_json::Value = Document::json("https://example.com/", User::random()).await?.expect("Failed to parse JSON");
    println!("Json: {json}");

    Ok(())
}
```

## Licensing:

Distributed under the MIT license.

## Feedback:

You can [find me here](https://t.me/fuderis), also [see my channel](https://t.me/fuderis_club).
I welcome your suggestions and feedback!

> Copyright (c) 2025 *Bulat Sh.* ([fuderis](https://t.me/fuderis))

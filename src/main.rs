extern crate web_parser;  use web_parser::{ prelude::*, };

#[tokio::main]
async fn main() -> Result<()> {
    // _____ WEB SEARCH (feature 'search'): _____
    
    #[cfg(feature = "search")]
    {
        // start search engine:
        let mut engine = DuckSearch::new(
            Some("bin/chromedriver/chromedriver.exe"),
            Some(macron::path!("$/WebSearch/Profile1").to_str().unwrap()),
            false,
        ).await?;

        println!("Searching results..");

        // send search query:
        let results = engine.search(
            "program hello world on Rust language",  // query
            &["support.google.com", "youtube.com"],  // black list
            1000  // sleep in millis
        ).await;

        // handle search results:
        match results {
            Ok(cites) => {
                println!("Reading result pages..");

                let contents = cites.read(5, &[
                    "header", "footer", "style", "script", "noscript",
                    "iframe", "button", "img", "svg"
                ]).await?;

                println!("Results: {contents:#?}");
            }
            Err(e) => eprintln!("Search error: {e}")
        }

        // stop search engine:
        engine.stop().await?;
    }

    /*
    // _____ READ PAGE AS HTML DOCUMENT: _____
    
    // read website page:
    let mut doc = Document::read("https://example.com/", User::random()).await?;

    // select 'lang' attribute:
    let html = doc.select("html")?.expect("No elements found");
    let lang = html.attr("lang").unwrap_or("en");
    println!("Language: {lang}");
    
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
    */

    Ok(())
}

extern crate web_parser;  use web_parser::{ prelude::*, };

#[tokio::main]
async fn main() -> Result<()> {
    // _____ WEB SEARCH: _____
    
    let mut google = GoogleSearch::run("bin/chromedriver/chromedriver.exe").await?;

    let results = google.search("Топ акций к покупке сегодня").await;

    match results {
        Ok(results) => {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let mut texts = vec![];
            let mut count = 0;

            for page in &results[1..] {
                let doc = page.read().await?;
                let main = doc.select("body")?.unwrap();
                
                texts.push(main.filter_text());

                count += 1;  if count > 2 { break; }
            }

            dbg!(texts);
        }
        Err(e) => eprintln!("Search error: {e}")
    }

    google.stop().await?;
    
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

extern crate web_search;  use web_search::{ prelude::*, };
use macron::*;

/*
1. Открываем google.com.
2. Имитируем ввод запроса 100мс и нажимаем "поиск".
3. Парсим список сайтов.
4. Открываем каждый сайт и считываем текст из тега main, заранее вырезав скрипты, стили и прочую хрень.
5. На выходе получаем карту текстов HashMap<URL, TEXT>, с которым уже можно взаимодействовать.

Для лучше опыта, следует разделить функции поиска и парсинга сайтов, чтобы покрыть все возможные варианты использования.
Например ищем запрос через метод search(query: &str) -> Vec<Link> и получаем массив ссылок.
А потом мы можем пройтись по всем Link и вызвать у них метод open() -> Document, и уже с ним работать.

Предлагаю для этих целей слегка доработать библиотеку web-parser, т.е. не создавать отдельный крейт web-search,
ибо смысла от этого много не меняется, всё равно надо исползовать наработки из web-parser.
*/


#[tokio::main]
async fn main() -> Result<()> {
    use chromedriver_api::Session;

    // starting chrome session:
    println!("[INFO]: Starting chromedriver session..");

    let free_port = std::net::TcpListener::bind("127.0.0.1:0")?.local_addr()?.port().to_string();
    
    let mut session = Session::run(
        &free_port,  // session server ip port
        Some("bin/chromedriver/chromedriver.exe"),  // path to chromedriver (None = to use global system Path)
        Some(path!("$/WebSearch/Profile").to_str().unwrap()),  // path to load/save profile (cookies, localStorage and etc.)
        false  // headless mode (without interface)
    ).await?;
    println!("[INFO]: the session is launched on port [{free_port}] ..");

    // searching by query:
    let query = "Погода на завтра в Смоленске 2";
    let url = str!("https://www.google.com/search?q={query}");

    let tab = session.open(url).await?;

    let results = tab.lock().await.inject::<Vec<String>>(r##"
        let domains = new Set();
        let links = [];

        document.querySelectorAll("#main *[data-rpos] a[href]").forEach(elem => {
            let href = elem.getAttribute("href");

            if (href && href.startsWith("https://")) {
                try {
                    let domain = new URL(href).hostname;
                    
                    if (!domains.has(domain)) {
                        domains.add(domain);
                        links.push(href);
                    }
                } catch (e) {
                }
            }
        });

        return links;
    "##).await;
    
    // close session:
    session.close().await?;
    println!("[INFO]: the session is closed");

    dbg!(results?);

    Ok(())
}

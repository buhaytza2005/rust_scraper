use reqwest;
use scraper;


#[derive(Debug)]
struct Product {
    url: Option<String>,
    name: Option<String>,
}

fn main() {

    // get the response from the page and unwrap text to variable
    let response = reqwest::blocking::get("https://scrapeme.live/shop/");
    let html = response.unwrap().text().unwrap();

    // parse response to a DOM
    let html_dom = scraper::Html::parse_document(&html);

    // create selector
    let product_selector = scraper::Selector::parse("li.product").unwrap();

    let html_products = html_dom.select(&product_selector);


    let mut list_of_pokes: Vec<Product> = vec![];

    for html_prod in html_products {
        let temp = Product {
            url :  html_prod
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(str::to_owned),
            name: html_prod
                .select(&scraper::Selector::parse("h2").unwrap())
                .next()
                .map(|element| element.text().collect::<String>())
        };
        list_of_pokes.push(temp);
    }

    println!("{:?}",list_of_pokes);

}

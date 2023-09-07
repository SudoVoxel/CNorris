use core::fmt;
use reqwest::Error;
use reqwest::Response;
use serde::Deserialize;
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
/// Contains all categories that the quotes can fall under.
pub enum Category {
    Animal,
    Career,
    Celebrity,
    Dev,
    Explicit,
    Fashion,
    Food,
    History,
    Money,
    Movie,
    Music,
    Political,
    Religion,
    Science,
    Sport,
    Travel,
}
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Animal => write!(f, "animal"),
            Category::Career => write!(f, "career"),
            Category::Celebrity => write!(f, "celebrity"),
            Category::Dev => write!(f, "dev"),
            Category::Explicit => write!(f, "explicit"),
            Category::Fashion => write!(f, "fashion"),
            Category::Food => write!(f, "food"),
            Category::History => write!(f, "history"),
            Category::Money => write!(f, "money"),
            Category::Movie => write!(f, "movie"),
            Category::Music => write!(f, "music"),
            Category::Political => write!(f, "political"),
            Category::Religion => write!(f, "religion"),
            Category::Science => write!(f, "science"),
            Category::Sport => write!(f, "sport"),
            Category::Travel => write!(f, "travel"),
        }
    }
}

#[derive(Deserialize, Debug)]
/// struct that contains the quote itself (value) but also its id and url (id, url, fields respectively).
pub struct Quote {
    pub id: String,
    pub url: String,
    pub value: String,
}
#[derive(Deserialize, Debug)]
/// struct that holds a list of quotes that were found given a query string, and also a count.
pub struct Quotelist {
    pub count: i64,
    pub result: Vec<Quote>,
}
/// Returns a Result containing either a Quote or a Reqwest::Error.
pub async fn random_quote() -> Result<Quote, Error> {
    let request: Result<Response, Error> =
        reqwest::get("https://api.chucknorris.io/jokes/random").await;
    let result: Quote = request?.json::<Quote>().await?;
    Ok(result)
}
/// Returns a Result containing either a Quote or a Reqwest::Error, given a specific Category.
pub async fn random_category_quote(category: Category) -> Result<Quote, Error> {
    let request: Result<Response, Error> = reqwest::get(format!(
        "https://api.chucknorris.io/jokes/random?category={}",
        category
    ))
    .await;
    let result: Quote = request?.json::<Quote>().await?;
    Ok(result)
}
/// Returns a Result containing either a Quote or a Reqwest::Error, given a query String.
pub async fn search_quotes(query: String) -> Result<Quotelist, Error> {
    let request: Result<Response, Error> = reqwest::get(format!(
        "https://api.chucknorris.io/jokes/search?query={}",
        query
    ))
    .await;
    let result: Quotelist = request?.json::<Quotelist>().await?;
    Ok(result)
}

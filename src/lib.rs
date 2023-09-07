mod quote;

#[cfg(test)]
mod tests {
    use crate::quote::*;

    async fn get_random_quote() {
        let response: Quote = random_quote().await.unwrap();
        assert!(!response.value.is_empty());
        assert!(!response.id.is_empty());
        assert!(!response.url.is_empty());
    }
    async fn get_quote_from_all_categories() {
        let categories: [Category; 16] = [
            Category::Animal,
            Category::Career,
            Category::Celebrity,
            Category::Dev,
            Category::Explicit,
            Category::Fashion,
            Category::Food,
            Category::History,
            Category::Money,
            Category::Movie,
            Category::Music,
            Category::Political,
            Category::Religion,
            Category::Science,
            Category::Sport,
            Category::Travel,
        ];
        for category in categories.iter() {
            let response: Quote = random_category_quote(*category).await.unwrap();
            assert!(!response.value.is_empty());
            assert!(!response.id.is_empty());
            assert!(!response.url.is_empty());
        }
    }
}

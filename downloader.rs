use serde_json::Value;
use std::fs;
use super::errors::DownloadError;

const BASE_URL: &str = "https://www.googleapis.com/books/v1/volumes";

pub struct BookDownloader<'a> {
    api_key: &'a str,
    authors: &'a Vec<String>,
}

impl<'a> BookDownloader<'a> {
    pub fn new(api_key: &'a str, authors: &'a Vec<String>) -> Self {
        Self { api_key, authors }
    }

    pub fn download_books(&self) -> Result<(), DownloadError> {
        for author in &self.authors {
            self.fetch_books_by_author(author)?;
        }
        Ok(())
    }

    fn fetch_books_by_author(&self, author: &str) -> Result<(), DownloadError> {
        let mut page_token: Option<String> = None;

        loop {
            let mut url = format!("{}?q=inauthor:\"{}\"&key={}", BASE_URL, author, self.api_key);
            
            if let Some(token) = &page_token {
                url.push_str(&format!("&pageToken={}", token));
            }

            let response = reqwest::blocking::get(&url)?.json::<Value>()?;

            if let Some(items) = response["items"].as_array() {
                for item in items {
                    if let Some(title) = item["volumeInfo"]["title"].as_str() {
                        let text = item["volumeInfo"]["description"].as_str().unwrap_or("");
                        let cleaned_text = self.clean_text(text);
                        self.save_to_txt(&title, &cleaned_text)?;
                    }
                }
            }

            page_token = response["nextPageToken"].as_str().map(String::from);
            if page_token.is_none() {
                break;
            }
        }

        Ok(())
    }

    fn save_to_txt(&self, title: &str, text: &str) -> Result<(), std::io::Error> {
        fs::write(format!("{}.txt", title), text)?;
        Ok(())
    }

    fn clean_text(&self, text: &str) -> String {
        let cleaned = text.replace("[", "").replace("]", "");
        remove_preface(&cleaned)
    }

    fn remove_preface(text: &str) -> String {
        let preface_keywords = ["Предисловие", "Редакционная заметка"];
        
        let mut result = text.to_string();
        for keyword in preface_keywords.iter() {
            result = result.replace(keyword, "");
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_preface() {
        let input = "Предисловие: Это введение. Основной текст начинается здесь. Редакционная заметка: Это заметка.";
        let expected = " Это введение. Основной текст начинается здесь.  Это заметка.";
        assert_eq!(remove_preface(input), expected);
    }
}

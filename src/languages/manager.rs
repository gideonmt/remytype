use super::models::Language;
use rand::prelude::IndexedRandom;

pub struct LanguageManager {
    languages: Vec<Language>,
}

impl LanguageManager {
    pub fn new() -> Self {
        let mut languages = Vec::new();
        
        if let Ok(lang) = Self::load_builtin("english_200") {
            languages.push(lang);
        }
        if let Ok(lang) = Self::load_builtin("english_1k") {
            languages.push(lang);
        }
        
        Self { languages }
    }
    
    fn load_builtin(name: &str) -> Result<Language, Box<dyn std::error::Error>> {
        let json_data = match name {
            "english_200" => include_str!("../../data/languages/english_200.json"),
            "english_1k" => include_str!("../../data/languages/english_1k.json"),
            _ => return Err("Language not found".into()),
        };
        
        let language: Language = serde_json::from_str(json_data)?;
        Ok(language)
    }
    
    pub fn get_language(&self, name: &str) -> Option<&Language> {
        self.languages.iter().find(|l| l.name == name)
    }
    
    pub fn available_languages(&self) -> Vec<String> {
        self.languages.iter().map(|l| l.name.clone()).collect()
    }
    
    pub fn generate_text(&self, language_name: &str, word_count: usize) -> String {
        if let Some(language) = self.get_language(language_name) {
            let mut rng = rand::rng();
            let mut words: Vec<String> = Vec::new();
            
            for _ in 0..word_count {
                if let Some(word) = language.words.choose(&mut rng) {
                    words.push(word.clone());
                }
            }
            
            words.join(" ")
        } else {
            "the quick brown fox jumps over the lazy dog".to_string()
        }
    }
}

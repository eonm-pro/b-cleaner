#![allow(dead_code)]

use std::borrow::Cow;
use unidecode::unidecode;

#[cfg(feature = "html")]
use htmlescape;

#[cfg(feature = "stem")]
use rust_stemmers::{Algorithm, Stemmer};

/// A trait used to clean and stem data
/// 
/// Data cleaner must implement the clean method which take a mutable reference as parameter and return a reference to the original struct.
pub trait Clean {
    /// ```
    /// # fn main () {
    /// use b_cleaner::{Clean, TitleCleaner};
    /// 
    /// let tokens = vec!["lorem", "impsum", "dolor", "sit"];
    /// 
    /// let mut title_cleaner = TitleCleaner::new(&tokens);
    /// 
    /// title_cleaner.clean();
    /// # }
    /// ```
    fn clean(&mut self) -> &Self;
    #[cfg(feature = "stem")]
    /// ```
    /// # fn main () {
    /// use b_cleaner::{Clean, TitleCleaner};
    /// 
    /// let mut title_cleaner = TitleCleaner::new(&vec!["lorem", "impsum", "dolor", "sit"]);
    /// 
    /// title_cleaner.stem();
    /// # }
    /// ```
    fn stem(&mut self, lang: Algorithm) -> &Self;
}

#[derive(Debug, Clone)]
/// A struct dedicated to text cleaning
/// 
/// Cleaning process is made in this specific order :
/// * tokens smaller than three chars are removed 
/// * HTML entities are decoded (html features)
/// * tokens are transformed to lowercase
/// * tokens are unidecoded (accentued chars are replaced by their ASCII equivalent)
/// * non ASCII char are removed
/// * punctuation and digit are removed
/// * tokens are trimed (extra white space at the begining and end of each token are removed)
/// * empty tokens are removed
/// 
/// Additionally token can be stemmed, howerver stemming implies huge performance downside.
pub struct TextCleaner<'a> {
    tokens: Vec<Cow<'a, str>>,
    token_min_lenght: usize
}

/// ```
/// use b_cleaner::{TextCleaner, Clean};
/// 
/// fn main() {
///     let tokens = vec!["Lorem", "ipsum", "dolor", ":", "sit", "amet"];
///     let mut text_cleaner = TextCleaner::new(&tokens);
/// 
///     text_cleaner.clean();
/// 
///     assert_eq!(text_cleaner.tokens(), &vec!["lorem", "ipsum", "dolor", "amet"]);
/// }
/// ```
impl <'a>TextCleaner<'a> {
    /// Create a new TextCleaner
    /// 
    /// ```
    /// # use b_cleaner::{TextCleaner, Clean};
    /// # fn main() {   
    /// let tokens = vec!["Lorem", "ipsum", "dolor", ":", "sit", "amet"];
    /// let mut text_cleaner = TextCleaner::new(&tokens);
    /// # }
    /// ```
    pub fn new<R: AsRef<str>>(input: &'a [R]) -> Self {
        let mut tokens : Vec<Cow<'a, str>> = Vec::with_capacity(input.len());
        tokens = input.iter().map(|token|Cow::Borrowed(token.as_ref())).collect();

        TextCleaner {
            tokens: tokens,
            token_min_lenght: 3
        }
    }

    /// Get tokens of the TextCleaner
    /// 
    /// ```
    /// # use b_cleaner::{TextCleaner, Clean};
    /// # fn main() {   
    /// let tokens = vec!["Lorem", "ipsum", "dolor", ":", "sit", "amet"];
    /// let mut text_cleaner = TextCleaner::new(&tokens);
    /// 
    /// assert_eq!(text_cleaner.tokens(), &vec!["Lorem", "ipsum", "dolor", ":", "sit", "amet"]);
    /// # }
    /// ```
    pub fn tokens(&self) -> &Vec<Cow<'a, str>> {
        &self.tokens
    }

    /// Set the token min length treshold (inclusive). Tokens under this treshold will be filtered out
    /// 
    /// ```
    /// # use b_cleaner::{TextCleaner, Clean};
    /// # fn main() {   
    /// let tokens = vec!["Lorem", "ipsum", "dolor", ":", "sit", "amet"];
    /// let mut text_cleaner = TextCleaner::new(&tokens);
    /// text_cleaner.token_min_lenght(4);
    /// 
    /// text_cleaner.clean();
    /// 
    /// assert_eq!(text_cleaner.tokens(), &vec!["lorem", "ipsum", "dolor"]);
    /// # }
    /// ```
    pub fn token_min_lenght(&mut self, thresold: usize) -> &Self {
        self.token_min_lenght = thresold;
        self
    }
}

impl <'a>Clean for TextCleaner<'a> {
    fn clean(&mut self) -> &Self {
        let token_min_lenght = self.token_min_lenght;

        self.tokens.retain(|token| !(token.len() <= token_min_lenght));

        self.tokens.iter_mut().for_each(|mut token| {
            #[cfg(feature = "html")]
            decode_token_html_entities(&mut token);

            token_to_lowercase(&mut token);
            unidecode_token(&mut token);
            remove_token_non_ascii_chars(&mut token);
            remove_token_digit_and_punctuation(&mut token);
            token_trim(&mut token);
        });

        self.tokens.retain(|token| !token.is_empty());
        self.tokens.shrink_to_fit();

        self
    }

    #[cfg(feature = "stem")]
    fn stem(&mut self, lang: Algorithm) -> &Self {
        let stemmer = Stemmer::create(lang);

        self.tokens.iter_mut().for_each(|token|  {
            let stem = Cow::Owned(stemmer.stem(token).to_string());

            if stem != *token  {
                *token = stem;
            }
        });

        self
    }
    
}

#[derive(Debug, Clone)]
/// A struct dedicated to title cleaning
/// 
/// Cleaning process is made in this specific order :
/// * subtitles are removed by spliting the title at it's first strong punctuation mark (`.`, `:`, `?`, `!`)
/// * tokens between `(`, `)` and between `[`, `]` are removed
/// * tokens smaller than three chars are removed 
/// * HTML entities are decoded (html features)
/// * tokens are transformed to lowercase
/// * tokens are unidecoded (accentued chars are replaced by their ASCII equivalent)
/// * non ASCII char are removed
/// * punctuation and digit are removed
/// * tokens are trimed (extra white space at the begining and end of each token are removed)
/// * empty tokens are removed
/// 
/// Additionally token can be stemmed, howerver stemming implies huge performance downside. The stem feature must be enabled.
pub struct TitleCleaner<'a> {
    tokens: Vec<Cow<'a, str>>,
    token_min_lenght: usize
}

/// ```
/// # fn main() {   
/// use b_cleaner::{TitleCleaner, Clean};
/// 
/// let tokens = vec!["Lorem", "ipsum", "dolor", ":", "sit", "amet"];
/// let mut title = TitleCleaner::new(&tokens);
/// 
/// title.clean();
/// 
/// 
/// assert_eq!(title.tokens(), &vec!["lorem", "ipsum", "dolor"]);
/// # }
/// ```
impl <'a>TitleCleaner<'a> {
    /// Create a new TitleCleaner
    /// 
    /// ```
    /// # use b_cleaner::{TitleCleaner, Clean};
    /// # fn main() {   
    /// let tokens = vec!["Lorem", "ipsum", "dolor", ":", "sit", "amet"];
    /// let mut title_cleaner = TitleCleaner::new(&tokens);
    /// # }
    /// ```
    pub fn new<R: AsRef<str>>(input: &'a [R]) -> Self {
        let mut tokens : Vec<Cow<'a, str>> = Vec::with_capacity(input.len());
        tokens = input.into_iter().map(|token|Cow::Borrowed(token.as_ref())).collect();

        TitleCleaner {
            tokens: tokens,
            token_min_lenght: 3
        }
    }

    /// Get tokens out of the TitleCleaner
    /// 
    /// ```
    /// # use b_cleaner::{TitleCleaner, Clean};
    /// # fn main() {   
    /// let tokens = vec!["Lorem", "ipsum", "dolor", ":", "sit", "amet"];
    /// let mut title_cleaner = TitleCleaner::new(&tokens);
    /// 
    /// assert_eq!(title_cleaner.tokens(), &vec!["Lorem", "ipsum", "dolor", ":", "sit", "amet"]);
    /// 
    /// # }
    /// ```
    pub fn tokens(&self) -> &Vec<Cow<'a, str>> {
        &self.tokens
    }

    /// Set the token min length treshold (inclusive). Tokens under this treshold will be filtered out
    /// 
    /// ```
    /// # use b_cleaner::{TitleCleaner, Clean};
    /// # fn main() {   
    /// let tokens = vec!["Lorem", "ipsum", "dolor", "sit", "amet"];
    /// let mut title_cleaner = TitleCleaner::new(&tokens);
    /// title_cleaner.token_min_lenght(4);
    /// 
    /// title_cleaner.clean();
    /// 
    /// assert_eq!(title_cleaner.tokens(), &vec!["lorem", "ipsum", "dolor"]);
    /// # }
    /// ```
    pub fn token_min_lenght(&mut self, thresold: usize) -> &Self {
        self.token_min_lenght = thresold;
        self
    }
}

impl <'a>Clean for TitleCleaner<'a> {
    fn clean(&mut self) -> &Self {
        tokens_split_at_strong_punctuation(&mut self.tokens);
        remove_tokens_between_delimiters(&mut self.tokens, ("(", ")"));
        remove_tokens_between_delimiters(&mut self.tokens, ("[", "]"));


        let token_min_lenght = self.token_min_lenght;

        self.tokens.retain(|token| !(token.len() <= token_min_lenght));

        self.tokens.iter_mut().for_each(|mut token| {
            #[cfg(feature = "html")]
            decode_token_html_entities(&mut token);

            token_to_lowercase(&mut token);
            unidecode_token(&mut token);
            remove_token_non_ascii_chars(&mut token);
            remove_token_digit_and_punctuation(&mut token);
            token_trim(&mut token);
        });

        self.tokens.retain(|token| !token.is_empty());
        self.tokens.shrink_to_fit();

        self
    }

    #[cfg(feature = "stem")]
    fn stem(&mut self, lang: Algorithm) -> &Self {
        let stemmer = Stemmer::create(lang);

        self.tokens.iter_mut().for_each(|token|  {
            let stem = Cow::Owned(stemmer.stem(token).to_string());

            if stem != *token  {
                *token = stem;
            }
        });

        self
    }
}

#[derive(Debug, Clone)]
/// A struct dedicated to cleaning author
/// 
/// Cleaning process is made in this specific order :
/// * HTML entities are decoded (html features)
/// * tokens are transformed to lowercase
/// * tokens between `(`, `)` and between `[`, `]` are removed
/// * tokens are unidecoded (accentued chars are replaced by their ASCII equivalent)
/// * non ASCII char are removed
/// * punctuation and digit are removed
/// * tokens are trimed (extra white space at the begining and end of each token are removed)
/// * empty tokens are removed
/// 
/// Additionally token can be stemmed, howerver stemming implies huge performance downside. The stem feature must be enabled.
pub struct AuthorCleaner<'a> {
    tokens: Vec<Cow<'a, str>>
}

/// ```
/// # fn main() {   
/// use b_cleaner::{AuthorCleaner, Clean};
/// 
/// let tokens = vec!["John", "W.", "Doe", "(1950-2020)"];
/// let mut author = AuthorCleaner::new(&tokens);
/// 
/// author.clean();
/// 
/// assert_eq!(author.tokens(), &vec!["john", "w", "doe"]);
/// # }
/// ```
impl <'a>AuthorCleaner<'a> {
    /// Create a new AuthorCleaner
    /// 
    /// ```
    /// # use b_cleaner::{AuthorCleaner, Clean};
    /// # fn main() {   
    /// let mut author_cleaner = AuthorCleaner::new(&vec!["John", "W.", "Doe", "(1950-2020)"]);
    /// # }
    /// ```
    pub fn new<R: AsRef<str>>(input: &'a [R]) -> Self {
        let mut tokens : Vec<Cow<'a, str>> = Vec::with_capacity(input.len());
        tokens = input.into_iter().map(|token|Cow::Borrowed(token.as_ref())).collect();

        AuthorCleaner {
            tokens: tokens
        }
    }

    /// Get tokens out of the AuthorCleaner
    /// 
    /// ```
    /// # use b_cleaner::{AuthorCleaner, Clean};
    /// # fn main() {   
    /// let tokens = vec!["John", "W.", "Doe", "(1950-2020)"];
    /// let mut author_cleaner = AuthorCleaner::new(&tokens);
    /// 
    /// assert_eq!(author_cleaner.tokens(), &vec!["John", "W.", "Doe", "(1950-2020)"]);
    /// # }
    /// ```
    pub fn tokens(&self) -> &Vec<Cow<'a, str>> {
        &self.tokens
    }
}

impl <'a>Clean for AuthorCleaner<'a> {
    fn clean(&mut self) -> &Self {
        remove_tokens_between_delimiters(&mut self.tokens, ("(", ")"));
        remove_tokens_between_delimiters(&mut self.tokens, ("[", "]"));

        self.tokens.iter_mut().for_each(|mut token| {
            #[cfg(feature = "html")]
            decode_token_html_entities(&mut token);
            
            token_to_lowercase(&mut token);
            
            remove_token_digit_and_punctuation(&mut token);
            unidecode_token(&mut token);
            remove_token_non_ascii_chars(&mut token);
            token_trim(&mut token);
        });

        self.tokens.retain(|token| !token.is_empty());
        self.tokens.shrink_to_fit();

        self
    }

    #[cfg(feature = "stem")]
     fn stem(&mut self, lang: Algorithm) -> &Self {
        let stemmer = Stemmer::create(lang);

        self.tokens.iter_mut().for_each(|token|  {
            let stem = Cow::Owned(stemmer.stem(token).to_string());

            if stem != *token  {
                *token = stem;
            }
        });

        self
    }
} 

fn token_to_lowercase<'a>(token: &mut Cow<'a, str>) {
    if token.chars().filter(|c| c.is_ascii_alphabetic()).any(|char| !char.is_ascii_lowercase()) {
        match token {
            Cow::Borrowed(_) => *token = Cow::Owned(token.to_lowercase()),
            Cow::Owned(t) => t.make_ascii_lowercase() 
        }
    }
}

fn token_trim<'a>(token: &mut Cow<'a, str>) {
    let chars : Vec<char> = token.chars().collect();

    match (chars.last(),  chars.first()) {
        (None, None) => (),
        (Some(c), _) => {
            if c.is_ascii_whitespace() {
                *token = token.trim().to_string().into();        
            }
        },
        (_, Some(c)) => {
            if c.is_ascii_whitespace() {
                *token = token.trim().to_string().into();
            }
        }
    }
}

/// Removes the subtitle of a list of tokens
fn tokens_split_at_strong_punctuation<'a>(tokens: &mut Vec<Cow<'a, str>>) {
    let hard_punct = tokens.iter().position(|e| e.ends_with('.') || e.ends_with(':') || e.ends_with('?') || e.ends_with('!'));
        
    if let Some(hard_punct) = hard_punct {
        let (temp_tokens,_) = tokens.split_at(hard_punct);

        *tokens = temp_tokens.to_vec();

        tokens.shrink_to_fit();
    }
}

/// Replace accented chars in a token by their unidecoded counterpart
pub fn unidecode_token<'a>(token: &mut Cow<'a, str>) {
    if token.chars().any(|char| !char.is_ascii()) {
        *token = unidecode(&token).into();
    }
}

/// Removes all digits of a token
fn remove_token_digit<'a>(token: &mut Cow<'a, str>) {
    if token.chars().any(|char| char.is_ascii_digit()) {
        *token = token.chars().filter(|char| !char.is_ascii_digit()).collect();
    }
}

/// Removes all punctuation mark of a token except `-` which can be used to join words
pub fn remove_token_punctuation<'a>(token: &mut Cow<'a, str>) {
    if token.chars().any(|char| char.is_ascii_punctuation()) {
        *token = token.chars().enumerate().filter(|(index, c)| !c.is_ascii_punctuation() || (*c == '-' && *index > 0 && *index < token.len())).map(|(_, c)| c).collect();
    }
}

/// Removes digit and punctuation of a token except `-` which can be used to join words
pub fn remove_token_digit_and_punctuation<'a>(token:  &mut Cow<'a, str>) {
    if token.chars().any(|char| (char.is_ascii_digit() || char.is_ascii_punctuation())) {
        *token = token.chars().enumerate().filter(|(index, c)| !c.is_ascii_punctuation() && !c.is_ascii_digit() || (*c == '-' && *index > 0 && *index < token.len())).map(|(_, c)| c).collect();
    }
}

/// Removes non ASCII chars from a token
pub fn remove_token_non_ascii_chars<'a>(token: &mut Cow<'a, str>) {
    let mut chars = token.chars();
    if chars.any(|char| !char.is_ascii()) {
        *token = chars.filter(|char| char.is_ascii()).collect();
    }
}

/// Removes tokens between delimiters
/// ```
/// # use std::borrow::Cow;
/// # use b_cleaner::remove_tokens_between_delimiters;
/// # fn main() {
/// let mut tokens: Vec<Cow<str>> = vec!["lorem" , "(ipsum", "dolor)", "sit", "amet"].into_iter().map(|token| Cow::Borrowed(token)).collect();
/// remove_tokens_between_delimiters(&mut tokens, ("(", ")"));
/// 
/// assert_eq!(tokens, vec!["lorem", "sit", "amet"]);
/// 
/// 
/// let mut tokens: Vec<Cow<str>> = vec!["lorem" , "(", "ipsum", "dolor", ")", "sit", "amet"].into_iter().map(|token| Cow::Borrowed(token)).collect();
/// remove_tokens_between_delimiters(&mut tokens, ("(", ")"));
/// 
/// assert_eq!(tokens, vec!["lorem", "sit", "amet"]);
/// # }
/// ```
pub fn remove_tokens_between_delimiters<'a>(tokens: &mut Vec<Cow<'a, str>>, delimiters: (&'a str, &'a str)) {
    while let Some(start) = tokens.iter().position(|token| token.starts_with(delimiters.0)) {
        if let Some(end) = tokens[start ..].iter().position(|token| token.ends_with(delimiters.1)) {
            tokens.drain(start.. end + start + 1);
        } else {
            break
        }
    }
}

#[cfg(feature = "html")]
/// Replace HTML encoded entities with their decoded counterpart
pub fn decode_token_html_entities<'a>(token: &mut Cow<'a, str>) {
    if token.starts_with('&') && token.ends_with(';') {
        match htmlescape::decode_html(&token) {
            Ok(escaped) => {
                if &escaped != token {
                    *token = Cow::Owned(escaped);          
                }
            },
            Err(_) => ()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_digit_and_punctuation() {
        let mut tokens : Cow<str>= Cow::Borrowed("W.#");        

        remove_token_digit_and_punctuation(&mut tokens);
        let t = tokens.to_owned();
        assert_eq!(t, Cow::Owned::<String>("W".into()));
        
    }

    #[test]

    fn test_remove_tokens_between_delimiters() {
        let mut input = vec!["abcdef", "(ezrà)", "sdfq", "(sss)"].into_iter().map(|e| Cow::Borrowed(e)).collect();
        
        remove_tokens_between_delimiters(&mut input, ("(", ")"));
        assert_eq!(input, vec!["abcdef", "sdfq"].into_iter().map(|e| Cow::Borrowed(e)).collect::<Vec<Cow<str>>>());
    }

    #[test]
    fn test_author_cleaner() {
        let tokens = vec!["John", "W.", "Doe", "(1950-2018)"];
        let mut author = AuthorCleaner::new(&tokens);

        author.clean();

        assert_eq!(author.tokens(), &vec!["john", "w", "doe"].into_iter().map(|e| Cow::Borrowed(e)).collect::<Vec<Cow<str>>>());
    }

    #[test]
    #[cfg(feature = "html")]
    fn test_decode_token_html_entities() {
        let mut token = Cow::Borrowed("&amp;");
        decode_token_html_entities(&mut token);

        assert_eq!(token, "&");
    }
}
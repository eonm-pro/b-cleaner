use std::fs::File;
use std::io::{BufReader, BufRead};
mod cleaners;
use std::borrow::Cow;
use std::time::Instant;
use std::error::Error;

use cleaners::{TitleCleaner};
use cleaners::Clean;

fn main() -> Result<(), Box<dyn Error>>{
    let file = File::open("titres.tsv")?;
    let buffered = BufReader::new(file);
    
    let mut owned_count : usize = 0;
    let mut borrowed_count : usize = 0;
    let mut total_tokens : usize = 0;

    let now = Instant::now();
    buffered.lines().map(|line| {
        let l = line?;
        let title = l.split_whitespace().collect::<Vec<&str>>();
        let mut t = TitleCleaner::new(&title);

        total_tokens += title.len();
        
        t.clean();

        t.tokens().iter().for_each(|elem| {
            match elem {
                Cow::Borrowed(_) => borrowed_count += 1,
                Cow::Owned(_) => owned_count += 1,
            }
        });
        
        Ok(())
    }).collect::<Result<(), Box<dyn Error>>>()?;

    println!("Total number of tokens {:?}", total_tokens);
    println!("Owned count {}", owned_count);
    println!("Borrowed count {}", borrowed_count);

    println!("{:?}", now.elapsed());

    Ok(())
}

<div align=center>

# B-cleaner (Bibliographical data cleaner)

[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)]()
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Feonm-abes%2Fb-cleaner.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Feonm-abes%2Fb-cleaner?ref=badge_shield)
[![Dependabot Status](https://badgen.net/dependabot/eonm-abes/b-cleaner?icon=dependabot)]()

</div>

B-cleaner is a Rust library dedicated to bibliographical data preprocessing (simplification, normalization). This library is used for preprocessing data in alignement tasks. B-cleaner is designed to have a small memory footprint and high performances.

B-cleaner offers **binding with Python 3**.
To compile b-cleaner as a Python library make sure you are building this library with the python features enabled: `cargo build --release --lib --features=python`. You can also use [maturin](https://github.com/PyO3/maturin).

## Usage
 
B-cleaner works with tokenized data. Tokenized data should contain punctuation.

B-cleaner is able to clean:

* titles
* authors
* any text

### Rust usage

```rust
use b_cleaner::{TitleCleaner, Clean};

fn main() {
    let raw_data: Vec<&str> = "Lorem ipsum dolor: sit amet".split_whitespace().collect();
    let mut title = TitleCleaner::new(&raw_data);
     
    title.clean();
          
    assert_eq!(title.tokens(), &vec!["lorem", "ipsum", "dolor"]);
 }
```
 
### Python usage

```python
>>> import b_cleaner as bc

>>> bc.clean_title(["Lorem", "ipsum", "dolor", "sit", "amet"])
#['lorem', 'ipsum', 'dolor', 'amet']

>>> bc.clean_author(["John", "W.", "Doe", "(1950-2018)"])
#['john', 'w', 'doe']
```

## Build B-cleaner for python

B-cleaner can be build and installed with [maturin](https://github.com/PyO3/maturin), a tool dedicated to build python native modules written in rust with [pyo3](https://github.com/PyO3/pyo3).

Make sure maturin is installed on your system:

```sh
pip install maturin
```

Maturin and pyo3 might require some developement dependencies to build the native module:

```sh
sudo apt install python3-dev python-dev
```

Then build and install b_cleaner with:

```sh
pip install .
```

use std::vec;

use crate::prelude::{*, tag::Tag};

#[test]
fn tag_test() {
    // let mut tag = Tag::new("artists".to_string(), "Mercury Rev".to_string());
    // tag.append("The Beatles".to_string());
    // let v = tag.value.clone();
    // tag.append(v);
    // println!("{:?}", tag);
}

#[test]
fn scanner_test() {
    let mut scan = Scanner::new("C:/Users/hrl/OneDrive/Music");
    scan.start(|| {
        scan.scan(None);
    });
}
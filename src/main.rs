#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]
#![allow(non_snake_case)]

mod doc;

use {std::fs, roxmltree, crate::doc::Doc};

fn main()
{
  println!("Hello, world!");
  println!("max u16: {}", u16::MAX);
  println!();

  let mut documents: Vec<Doc> = vec![];
  let mut queries: Vec<Doc> = vec![];

  let mut queries_str = fs::read_to_string("data/queries.trec").unwrap();
  queries_str = queries_str.replace("\n", "").replace("\r", "");
  // Reduce multiple whitespaces to one
  queries_str = queries_str.split(" ")
    .filter(|string| !string.is_empty())
    .collect::<Vec<_>>()
    .join(" ");
  queries_str = queries_str.replace("> <", "><");

  // https://docs.rs/roxmltree/0.20.0/roxmltree/index.html
  let doc = roxmltree::Document::parse(&queries_str).unwrap();
  doc.root()
    .first_child().unwrap()
    .children()
    .for_each(|DOC|
    {
      queries.push(Doc::new(
        // recordId
        DOC.first_child().unwrap().text().unwrap().parse().unwrap(),
        // text
        DOC.last_child().unwrap().text().unwrap().to_string()
      ));
    });

  println!("{:#?}", queries.first_chunk::<5>().unwrap());

  // Note: newlines are also treated as nodes
}

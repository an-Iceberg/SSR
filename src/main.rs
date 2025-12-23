#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]
#![allow(non_snake_case)]

mod doc;

use {crate::doc::Doc, hashbrown::HashMap, indicatif::{ProgressBar, ProgressStyle}, std::{fs::{self, File}, io::Write}, strsim::levenshtein};
use colored::Colorize;

fn bar_factory(len: u64, message: String) -> ProgressBar
{
  let bar = ProgressBar::new(len);
  bar.set_style(
    ProgressStyle::with_template("{spinner} {msg} {percent:>2}% {pos:>5}/{len:5} [{elapsed_precise}] {wide_bar:.cyan}")
      .unwrap()
      .progress_chars("#>-")
      .tick_chars("🌑🌒🌓🌔🌕🌖🌗🌘")
  );
  bar.set_message(message);

  return bar;
}

// Todo: this is full of .unwrap()s but we don't really care b/c we're too exhausted
fn load_docs(file_path: String) -> HashMap<u16, Doc>
{
  let mut data_strings = fs::read_to_string(&file_path).unwrap();
  data_strings = data_strings.replace("\n", "").replace("\r", "");
  // Reduce multiple whitespaces to one
  data_strings = data_strings.split(" ")
    .filter(|string| !string.is_empty())
    .collect::<Vec<_>>()
    .join(" ");
  data_strings = data_strings.replace("> <", "><");

  let mut hashmap: HashMap<u16, Doc> = HashMap::with_capacity(21_000);

  // Todo: add progress meter with indicatif
  // https://docs.rs/roxmltree/0.20.0/roxmltree/index.html
  let doc = roxmltree::Document::parse(&data_strings).unwrap();

  let bar = bar_factory(
    doc.root().first_child().unwrap().children().count() as u64,
    format!("extracting {}", &file_path[5..].to_string())
  );

  doc.root()
    .first_child().unwrap()
    .children()
    .for_each(|DOC|
    {
      let record_id = DOC.first_child().unwrap().text().unwrap().parse().unwrap();
      let text = DOC.last_child().unwrap().text().unwrap().to_string();
      let doc = Doc::new(record_id, text);
      hashmap.insert(record_id, doc);
      bar.inc(1);
    });

  bar.finish_and_clear();

  return hashmap;
}

fn main()
{
  // Todo: put these documents into polars dataframes. Is that really a good idea tho?
  // Todo: remove polars, egui, eframe (not needed)

  // Todo: these need to be hashmaps: ID -> document
  let mut documents = load_docs("data/documents.trec".to_string());
  let mut queries = load_docs("data/queries.trec".to_string());

  documents.shrink_to_fit();
  queries.shrink_to_fit();

  let documents = documents;
  let queries = queries;

  // dbg!(queries.len());
  // dbg!(documents.len());

  // dbg!(documents.get(&245));

  let path = "trec_eval";
  let mut file = File::create(path).unwrap();

  for query in queries
  {
    let mut similarity_scores = Vec::with_capacity(21_000);
    for document in &documents
    {
      let mut score = 0;
      for query_word in query.1.longest_words()
      {
        for doc_word in document.1.longest_words()
        {
          if query_word.len().abs_diff(doc_word.len()) <= 3
          {
            score += levenshtein(query_word, doc_word);
          }
        }
      }
      similarity_scores.push((*document.0, score));
    }
    similarity_scores.sort_by_key(|entry| entry.1);

    for (rank, (document_id, score)) in similarity_scores.iter().enumerate()
    {
      file.write_fmt(
        format_args!(
          "{} Q0 {} {} {} levenshtein\n",
          query.0,
          document_id,
          rank,
          score,
        )
      ).unwrap();
      if rank >= 999 { break; }
    }
  }
}

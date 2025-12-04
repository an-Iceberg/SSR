#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]
#![allow(non_snake_case)]

mod doc;

use {crate::doc::Doc, hashbrown::HashMap, indicatif::{ProgressBar, ProgressStyle}, std::fs, strsim::levenshtein};
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

  dbg!(queries.len());
  dbg!(documents.len());

  dbg!(documents.get(&245));

  // let queries_bar = bar_factory(queries.len() as u64, "Running queries…".to_string());
  for query in queries
  {
    let mut similarity_scores = Vec::with_capacity(21_000);
    // let documents_bar = bar_factory(documents.len() as u64, "Calculating similarities…".to_string());
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
      // documents_bar.inc(1);
    }
    // documents_bar.finish_and_clear();
    similarity_scores.sort_by_key(|entry| entry.1);
    println!(
      "query #{}: {:?}",
      query.0,
      similarity_scores[0..10]
        .to_vec()
        .iter()
        .map(|(id, _)| *id)
        .collect::<Vec<u16>>()
    );
    // queries_bar.inc(1);
  }
  // queries_bar.finish_and_clear();

  // FIX: this takes waaaaaaay too long
  // let other_documents = documents.iter()
  //   .map(|(_, doc)| doc.clone())
  //   .clone()
  //   .collect::<Vec<Doc>>();

  // let outer_bar = bar_factory(documents.len() as u64, "computing similar documents".to_string());
  // for document in documents.values_mut()
  // {
  //   let inner_bar = bar_factory(other_documents.len() as u64, format!("calculating levenshtein distances for doc#{}:", document.record_ID()));
  //   let mut distances: Vec<(u16, usize)> = vec![];
  //   for other_document in other_documents.iter()
  //   {
  //     if other_document.record_ID() == document.record_ID() { continue; }
  //     distances.push((
  //       document.record_ID(),
  //       levenshtein(document.text(), other_document.text())
  //     ));
  //     inner_bar.inc(1);
  //   }
  //   distances.sort_by(|a, b| a.1.cmp(&b.1));
  //   document.set_similar_doc_ids(
  //     distances.iter()
  //       .map(|(id, _)| *id)
  //       .take(10)
  //       .collect()
  //   );

  //   inner_bar.finish_and_clear();
  //   outer_bar.inc(1);
  // }
  // outer_bar.finish_and_clear();

  // dbg!(documents.get(&245));
}

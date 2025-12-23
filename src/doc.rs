#[derive(Default, Debug, PartialEq, Clone)]
pub(crate) struct Doc
{
  record_ID: u16,
  text: String,
  // Todo: this should be a simple hashset or vec (each trigram should only occur once)
  similar_doc_ids: Vec<u16>,
  longest_words: Vec<String>,
}

impl Doc
{
  pub fn new(id: u16, text: String) -> Self
  {
    // Todo: strip symbols
    let mut words: Vec<String> = text
      .split(" ")
      .filter(|word| word.len() > 3)
      .map(|word| word.to_string())
      .collect();

    words.sort_by_key(|a| a.len());

    words.reverse();

    return Doc
    {
      record_ID: id,
      text,
      longest_words: words,
      ..Default::default()
    };
  }

  pub fn text(&self) -> &String
  {
    return &self.text;
  }

  pub fn record_ID(&self) -> u16
  {
    return self.record_ID;
  }

  pub fn set_similar_doc_ids(&mut self, ids: Vec<u16>)
  {
    self.similar_doc_ids = ids.to_vec();
  }

  pub fn longest_words(&self) -> &Vec<String>
  {
    return &self.longest_words;
  }
}

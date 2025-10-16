use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Doc
{
  record_ID: u16,
  text: String
}

impl Doc
{
  pub fn new(id: u16, text: String) -> Self
  {
    return Doc
    {
      record_ID: id,
      text
    };
  }
}

use serde::Serialize;

pub enum SearchPayLoadEvent {
  Application,
  Folder
}


impl Serialize for SearchPayLoadEvent {
  fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
  where
      S: serde::Serializer,
  {
      // Implement the serialization logic for your enum variants
      unimplemented!()
  }
}

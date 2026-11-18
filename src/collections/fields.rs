use std::{
  collections::HashMap,
  collections::hash_map::{IntoIter, Iter, IterMut},
};

///////////// Fields

#[derive(Clone,Debug)]
pub struct Fields {
  pub fields: HashMap<String,String>,
}

impl IntoIterator for Fields {
  type Item = (String,String);
  type IntoIter = IntoIter<String,String>;
  fn into_iter(self) -> Self::IntoIter {
    self.fields.into_iter()
  }
}

impl<'a> IntoIterator for &'a Fields {
  type Item = (&'a String,&'a String);
  type IntoIter = Iter<'a,String,String>;
  fn into_iter(self) -> Self::IntoIter {
    self.fields.iter()
  }
}

impl<'a> IntoIterator for &'a mut Fields {
  type Item = (&'a String,&'a mut String);
  type IntoIter = IterMut<'a,String,String>;
  fn into_iter(self) -> Self::IntoIter {
    self.fields.iter_mut()
  }
}

impl Fields {
  pub fn new(source: &HashMap<&str,String>) -> Fields {
    let mut fields: HashMap<String,String> = HashMap::new();
    for (key,value) in source {
      fields.insert(key.to_string(),value.clone());
    }
    Fields { fields: fields }
  }

  pub fn contains_key(&self, key: &str) -> bool {
    self.fields.contains_key(key)
  }

  pub fn insert(&mut self, key: &str, value: String) {
    self.fields.insert(key.to_string(),value);
  }

  pub fn remove(&mut self, key: &str) -> Option<String> {
    self.fields.remove(key)
  }

  pub fn get(&self, key: &str) -> Option<String> {
    self.fields.get(key).cloned()
  }

  pub fn iter(&self) -> Iter<'_,String,String> {
    self.fields.iter()
  }

  pub fn iter_mut(&mut self) -> IterMut<'_,String,String> {
    self.fields.iter_mut()
  }
}

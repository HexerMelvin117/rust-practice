pub struct Book {
  pub title: String,
  pub author: String,
}

impl Book {
  pub fn present_book(&self) {
    println!(
      "The title of this book is: {} written by {}",
      self.title, self.author
    )
  }
}

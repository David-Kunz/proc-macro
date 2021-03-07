use derive_entity::Entity;
use entity;

#[derive(Entity)]
pub struct Book {
    id: u64,
    title: String,
    pages: u64,
    author: String,
}

fn main() {
    let select_options = entity::SelectOptions { limit: 100 };
    let select_sql = Book::select(&select_options);
    assert_eq!("select id,title,pages,author from Book limit 100;", select_sql);
}

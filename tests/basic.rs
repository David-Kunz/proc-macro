use derive_entity::Entity;

#[derive(Entity)]
pub struct Book {
    id: u64,
    title: String,
    pages: u64,
    author: String,
}

fn main() {
    let mut select = Book::select();
    select.set_limit(200).set_unique().set_columns(vec!["id", "title"]);
    assert_eq!("select id,title,pages,author from Book;", select.to_sql());
}

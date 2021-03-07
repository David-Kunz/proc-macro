use derive_entity::Entity;

#[derive(Entity)]
pub struct Book {
    id: u64,
    title: String,
    pages: u64,
    author: String,
}

fn main() {
    let select_sql = Book::select();
    assert_eq!("select id,title,pages,author from Book;", select_sql);
}

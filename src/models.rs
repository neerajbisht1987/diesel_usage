use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

//use diesel::deserialize::Queryable;
use crate::schema::books;
use crate::schema::books::dsl::books as all_books;




#[derive(Serialize,Queryable,Debug,Clone)]
pub struct Book {
    pub id : u64,
    pub title : String,
    pub author : String,
    pub published: bool,
}

#[derive(Insertable,Serialize,Deserialize)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Book {
    pub fn show(id:u64, conn: &MysqlConnection) -> Vec<Book>{
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading book")
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Book>
    {
         all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("Error loading book")
    }

    pub fn update_by_id(id: u64, conn:&MysqlConnection, book:NewBook) -> bool {
        use crate::schema::books::dsl::{author as a, published as p , title as t};
        let NewBook{
            title,
            author,
            published,
        } = book; 
        diesel::update(all_books.find(id))
        .set((a.eq(author),p.eq(published),t.eq(title)))
        .execute(conn)
        .is_ok()
        }

        pub fn insert(book: NewBook, conn:&MysqlConnection) -> bool {
            diesel::insert_into(books::table)
                .values(&book)
                .execute(conn)
                .is_ok()
        }
        pub fn delete_by_id(id : u64, conn :&MysqlConnection)->bool {
            if Book::show(id, conn).is_empty() {
                    return false;
            }
            diesel::delete(all_books.find(id)).execute(conn).is_ok()
        }
        pub fn all_by_author(author:String, conn : &MysqlConnection) ->Vec<Book> {
            all_books
                .filter(books::author.eq(author))
                .load::<Book>(conn)
                .expect("Error loading books")
        }
    }

use dotenvy::dotenv;

use lib::{
    db::{
        self,
        models::clip::InsertClip,
        query::clip::{get_clip, insert_clip},
    },
    domain::clip::Clip,
};
use sqlx::Executor;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database = db::Database::default().await;

    let pool = database.get_pool();

    let abc = pool.execute("SELECT * FROM clip;").await;
    println!("{:?}", abc);

    println!(
        "{:?}",
        insert_clip(
            InsertClip {
                short_code: String::from("123"),
                title: Some(String::from("Hello World")),
                content: String::from("Hello World Content"),
                password: None,
                expires_at: None
            },
            pool
        )
        .await
    );
    println!("{:?}", get_clip(String::from("123"), pool).await);

    println!("{:?}", dotenvy::var("DATABASE_URL"));
}

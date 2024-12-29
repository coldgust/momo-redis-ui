use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use tauri::{App, Manager};

pub type Db = Pool<Sqlite>;

pub async fn setup_db(app: &App) -> Db {
    let mut app_path = app.path().app_data_dir().expect("failed to get app_path");

    dbg!(app_path.clone());

    match std::fs::create_dir_all(app_path.clone()) {
        Ok(_) => {}
        Err(e) => {
            panic!("failed to create app_path: {:?}", e);
        }
    }

    app_path.push("db.sqlite");

    Sqlite::create_database(format!("sqlite:{}", app_path.to_str().unwrap()).as_str())
        .await
        .expect("failed to create Sqlite database");

    let db = SqlitePoolOptions::new()
        .connect(app_path.to_str().unwrap())
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    db
}

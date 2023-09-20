use dotenv;

use sqlx::query;
use sqlx::PgPool;
use sqlx::Pool;
use tide::Request;
use tide::Server; //friendly HTTP server

#[async_std::main]
async fn main() -> Result<(), Error> {
	dotenv::dotenv().ok();
	pretty_env_logger::init();

	let db_url = std::env::var("DATABASE_URL")?;
	// dbg!(db_url);

	let db_pool: PgPool = Pool::new(&db_url).await?;
	let rows = query!("select 1 as one").fetch_one(&db_pool).await?;
	dbg!(rows);
	let my_name = "ngajda".to_string();
	let state = State { db_pool, my_name };
	let mut app: Server<State> = Server::with_state(state);
	app.at("/").get(|req: Request<State>| async move {
		let db_pool: &PgPool = &req.state().db_pool;

		query!("select 1 as one").fetch_one(db_pool).await?;
		Ok(format!("Hello, world. {}!", &req.state().my_name))
	});

	app.listen("127.0.0.1:9000").await?;

	Ok(())
}

#[derive(Debug, Clone)]
struct State {
	db_pool: PgPool,
	my_name: String,
}

#[derive(thiserror::Error, Debug)]
enum Error {
	#[error(transparent)]
	DbError(#[from] sqlx::Error),

	#[error(transparent)]
	IoError(#[from] std::io::Error),
	#[error(transparent)]
	VarError(#[from] std::env::VarError),
}

use std::env;
use std::error::Error;

use rand::rngs::ThreadRng;
use rand::Rng;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::QueryBuilder;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
	// connect to database
	let pool = MySqlPoolOptions::new().connect(&env::var("DATABASE_URL")?).await?;

	println!("testing bulk inserts: start");

	let bulk_count = 1000;

	let mut bulk_buf = Vec::with_capacity(bulk_count);
	let mut rng = rand::thread_rng();

	for i in 0..3000 {
		bulk_buf.clear();

		for _ in 0..bulk_count {
			bulk_buf.push((
				randnum(&mut rng),
				randstr(&mut rng),
				randnum(&mut rng),
				randstr(&mut rng),
				randnum(&mut rng),
				randstr(&mut rng),
			));
		}

		println!("  bulk insert #{}", i + 1);

		let mut query_builder = QueryBuilder::new(
			"INSERT INTO mysqlleak.rands (randnum1, randstr1, randnum2, randstr2, randnum3, randstr3) ",
		);

		query_builder.push_values(&bulk_buf, |mut buff, (n1, s1, n2, s2, n3, s3)| {
			buff.push_bind(n1)
				.push_bind(s1.as_str())
				.push_bind(n2)
				.push_bind(s2.as_str())
				.push_bind(n3)
				.push_bind(s3.as_str());
		});

		query_builder.build().execute(&pool).await?;
	}

	println!("testing bulk inserts: stop");

	Ok(())
}

fn randnum(rng: &mut ThreadRng) -> i32 {
	rng.gen_range(-1000..=1000)
}

fn randstr(rng: &mut ThreadRng) -> String {
	let mut buf = Vec::with_capacity(100);
	for _ in 0..rng.gen_range(1..=100) {
		buf.push(rng.gen_range(b'a'..=b'z'));
	}
	String::from_utf8(buf).unwrap()
}

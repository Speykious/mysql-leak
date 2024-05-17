use std::env;
use std::error::Error;
use std::time::Instant;

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

	let start = Instant::now();

	for i in 0..1000 {
		bulk_buf.clear();

		for _ in 0..bulk_count {
			bulk_buf.push((randnum(&mut rng), randnum(&mut rng), randstr(&mut rng)));
		}

		println!("  bulk insert #{}", i + 1);

		// build query for values
		{
			let mut query_builder = QueryBuilder::new("INSERT INTO mysqlleak_demo.values (rand_a, rand_b, rand_s) ");

			query_builder.push_values(&bulk_buf, |mut buff, (a, b, s)| {
				buff.push_bind(a).push_bind(b).push_bind(s.as_str());
			});

			query_builder.build().execute(&pool).await?;
		}

		// build query for times
		{
			let mut query_builder = QueryBuilder::new("INSERT INTO mysqlleak_demo.times (rand_a, millis) ");

			let millis = start.elapsed().as_millis() as u64;
			query_builder.push_values(&bulk_buf, |mut b, (a, _, _)| {
				b.push_bind(a).push_bind(millis);
			});

			query_builder.push("ON DUPLICATE KEY UPDATE millis = VALUES(millis)");

			query_builder.build().execute(&pool).await?;
		}
	}

	println!("testing bulk inserts: stop");

	Ok(())
}

fn randnum(rng: &mut ThreadRng) -> i32 {
	rng.gen_range(-1_000_000..=1_000_000)
}

fn randstr(rng: &mut ThreadRng) -> String {
	let mut buf = Vec::with_capacity(100);
	for _ in 0..rng.gen_range(1..=100) {
		buf.push(rng.gen_range(b'a'..=b'z'));
	}
	String::from_utf8(buf).unwrap()
}

# MySQL Leak?

This is a simple program that demonstrates a constant growth in `mysqld`'s memory consumption, first in Rust with the `sqlx` crate, second in Java with `mysql-connector-j`, when doing multiple bulk inserts.

> For the record, this is just a single-instance database and not one with master-slave replication.

Here is a graph of this growth using the `mysql:8.2` Docker image:

- Rust with `sqlx`
  ![Graph from Docker Desktop showing constant memory growth with the Rust script](/docker-graph-rust-sqlx.png)

- Java with `mysql-connector-j`
  ![Graph from Docker Desktop showing constant memory growth with the Java script](/docker-graph-java-sql.png)

> **Important note:** these graphs are provided by the Portainer extension of Docker Desktop and not by Docker Desktop itself. Docker Desktop doesn't seem to show a growth in memory consumption, but this doesn't correlate with local runs of this program which eventually crash the server when left unsupervised.

Restarting the `mysqld` daemon resets the memory consumption to about 520 MB.

Since the linear memory growth happens both in Java and Rust, it is highly likely to be related to MySQL. That being said, I have looked into MySQL's statement cache settings, inspected connections to make sure `sqlx` wasn't creating tons of connections without properly releasing them, and found no concrete path to the root of the issue.

MySQL `.cnf` file used in the Docker container:

```ini
[mysqld]
character-set-server=utf8mb4
collation-server=utf8mb4_unicode_ci
log-bin-trust-function-creators=1

[client]
default-character-set=utf8mb4
```

## What we tried

Everything we have tried below has failed to prevent this constant growth (sometimes combining them):

- sleep for a second between each query (Rust, Java)
- having only one SQL connection (Rust, Java)
- reduce the bulk size in lines (Rust, Java)
- not having a prepared statement (Java - less secure)
- configuring InnoDB parameters like `innodb_buffer_pool_size = 1G`
- configuring `mysqld` parameters like `tmp-table-size = 200M` and `max-heap-table-size = 200M`

## How to run the program

Both the Rust and Java programs insert continuously in bulks of 1000 lines, 10,000 times. The memory consumption typically reaches 2 GB after having inserted 3500 bulks.

Before running either program, first create an empty database called `mysqlleak_demo` for this script, then run the [mysqlleak.sql](/mysqlleak.sql) script on it to create the table.

Then simply run the following:

- Rust (using Cargo, Rust 1.78.0 stable)
  ```sh
  cd rust-sqlx
  DATABASE_URL=$insert_db_url_here cargo run
  ```

- Java (using Maven, Java 11)
  ```sh
  cd java-sql
  mvn clean install
  DATABASE_URL=$insert_db_url_here mvn exec:java
  ```

Stop it at any point in time when you have assessed the presence or absence of a constant growth in `mysqld`'s memory consumption.

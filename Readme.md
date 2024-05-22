# MySQL Leak?

This is a simple program that demonstrates a constant growth in `mysqld`'s memory consumption with the `sqlx` crate when doing multiple bulk inserts.

Here is a graph of this growth using the `mysql:8.2` Docker image:

![Graph from Docker Desktop showing leaks](/docker-graph-leak.png)

I have no idea if it is a problem related to `sqlx` or related to MySQL.
I have looked into MySQL's statement cache settings, inspected connections to make sure `sqlx` wasn't creating tons of connections without properly releasing them, and found no concrete path to the root of the issue.

MySQL `.cnf` file used in the Docker container:

```toml
[mysqld]
character-set-server=utf8mb4
collation-server=utf8mb4_unicode_ci
log-bin-trust-function-creators=1

[client]
default-character-set=utf8mb4
```

## How to run the program

To run the program, first create an empty database for this script, then run the [mysqlleak.sql](/mysqlleak.sql) script on it to create the table.

Then simply run the following:

```sh
DATABASE_URL=$insert_db_url_here cargo run
```

Stop it at any point in time when you have assessed the presence or absence of a constant growth in `mysqld`'s memory consumption.

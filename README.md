## Steps to create the project
1. run the command `cargo new <project name>`
2. configure your project by editing the toml file
3. write the code
4. compile the code using `cargo build`
5. run the code using `cargo run`

## Database
- for the database I am using MySql

## Caching

I am using redis for implementing caching.  
on Windows redis cannot be used directly.  
An alternative is available called `memurai`. It runs a redis server on windows.  
you can use redis insight to visually manage the redis datastore running locally or on cloud. 

The caching strat is simple when we try to find a user if :  
1. it is in the cache we return from there
2. if not in cache fallback to database and save that user's data to database and return it

when deleting or updating a user invalidate the cache i.e. remove it from redis.
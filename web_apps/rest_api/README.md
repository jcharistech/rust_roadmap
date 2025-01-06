#### Project Structure
```
|- Cargo.toml
|- src
    |- main.rs
    |- db
        |- mod.rs
        |- connection.rs
        |- models.rs
        |- queries.rs
    |- api
        |- mod.rs
        |- routes.rs
        |- errors.rs
        |- docs.rs
    |- utils
        |- mod.rs
        |- env.rs

```


#### SQLx commands
```
sqlx migrate add create_blog_posts_table
sqlx migrate run --database-url "postgres://postgres:yourpassword@localhost:5432/rustdb"  
````
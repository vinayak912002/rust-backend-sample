use sqlx::mysql::MySqlPool;
use sqlx_core::error::Error;
use crate::models::user::{User, CreateUser};
use uuid::Uuid;

pub struct UserRepository{
    pool:MySqlPool,
}

impl UserRepository{

    pub fn new(pool:MySqlPool)->Self{
        Self{pool}
    }

    //Create- insert new user into the database
    pub async fn create(&self, user:CreateUser)->Result<User, Error>{
        let id = Uuid::new_v4().to_string();

        let result = sqlx::query(
            "INSERT INTO users (id, name, age) VALUES (?, ?, ?)"
        )
        .bind(&id)
        .bind(&user.name)
        .bind(&user.age)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id(); // Get the auto-generated ID
    
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, age FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    // READ - get by ID
    pub async fn find_by_id(&self, id:&str)-> Result<User, Error>{
        
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    //READ - get all users
    pub async fn find_all(&self)->Result<Vec<User>, Error>{
        let users = sqlx::query_as::<_, User>(
            "Select * from users"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(users)
    }

    //UPDATE - update an existing user
    pub async fn update(&self, id:&str, user:CreateUser)->Result<User, Error>{
        sqlx::query(
            "UPDATE users SET name = ?, age = ? WHERE id = ?"
            //                       ^       ^              ^
            //                              MySQL uses ?
        )
        .bind(&user.name)
        .bind(&user.age)
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        //fetch the updated row seperately
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id=?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    //Delete - delete an existing user
    pub async fn delete(&self, id:&str)->Result<(), Error>{
        sqlx::query("DELETE FROM users WHERE id =?")
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

}
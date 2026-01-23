use crate::models::user::{User, CreateUser};
use crate::repositories::user_repository::UserRepository;

use sqlx::Error;
use redis::{Client as RedisClient, RedisResult, TypedCommands};

pub struct UserService{
    repository:UserRepository,
    redis:RedisClient,
}

impl UserService{
    
    pub fn new(
        repository:UserRepository,
        redis:RedisClient,
    )->Self{
        Self{
            repository,
            redis,
        }
    }

    pub async fn create_user(&self, name:String, age:i32)->Result<User, Error>{

        if name.trim().is_empty(){
            return Err(Error::RowNotFound);
        }

        if age<=0 || age>=150{
            return Err(Error::RowNotFound);
        }

        let create_user = CreateUser{name, age};
        self.repository.create(create_user).await
    }

    pub async fn get_user(&self, id:&str)->Result<User, Error>{
        // The match keyword is used for robust error handling
        // 1. connect with redis
        let mut redis_con = match self.redis.get_connection(){
            Ok(c)=>c,
            Err(_) => {
                println!("Failed to connect to Redis falling back to DB");
                return self.repository.find_by_id(id).await;
            }
        };

        let cache_key = format!("user:{}", id);

        // 2. try finding data in redis
        // Some is part of rust's Option enum, you can use it to unwrap the Option
        /* 
        enum Option<T> {
            Some(T),    // Contains a value of type T
            None,       // Contains no value
            }
        */
        let str_json: Option<String> = match redis_con.get(&cache_key) {
            Ok(Some(value)) => {
                // Cache hit - we got a value
                println!("Cache hit for key: {}", cache_key);
                Some(value)
            }
            Ok(None) =>{
                // Cache miss
                println!("Cache miss falling back to DB");
                None
            }
            Err(_) => {
                // Either cache miss or Redis error
                // Note: This assumes any error means "not found" - you might want more specific error handling
                println!("some error occured for key: {}", cache_key);
                None
            }
        };

        if str_json.is_some() {
            match str_json {
                Some(cached_json)=>{
                    match serde_json::from_str::<User>(&cached_json){
                        Ok(user) => return Ok(user),
                        Err(_)=>() 
                    }
                },
                None =>()
            }
        }

        //3. cache miss - load from database
        let user = self.repository.find_by_id(id).await?;
        
        // save the cache in the database
        match self.redis.get_connection() {
            Ok(mut conn) => {
                let user_json = serde_json::to_string(&user).unwrap_or_default();
                let _: RedisResult<()> = conn.set_ex(&cache_key, user_json, 86400);
            },
            Err(_) => {
                // Connection failed - handle the error or just ignore
                // You might want to log this error
            }
        }
        Ok(user)
    }


    pub async fn get_all_users(&self)->Result<Vec<User>, Error>{

        self.repository.find_all().await
    }

    pub async fn update_user(&self, id:&str, name:String, age:i32)->Result<User, Error>{
        if name.trim().is_empty(){
            return Err(Error::RowNotFound);
        }
        let update_user = CreateUser{name, age};
        let user = self.repository.update(id, update_user).await?;

        // we want to invalidate the cache if we are updating or deleting an user
        // i.e. we remove the key of the user from cache
        match self.redis.get_connection() {
            Ok(mut conn)=>{
                let key = format!("user:{}", id);
                let _ = conn.del(key);
            }
            Err(_)=>{
                println!("not able to remove from cache id : {}", id);
            }
        }
        Ok(user)
    }

    pub async fn delete_user(&self, id:&str)->Result<(), Error>{

        match self.redis.get_connection() {
            
            Ok(mut conn)=>{
                let key = format!("user:{}", id);
                let _ = conn.del(key);
            }
            Err(_)=>{
                println!("not able to remove from cache id : {}", id);
            }
        }
        
        self.repository.delete(id).await
    }

}
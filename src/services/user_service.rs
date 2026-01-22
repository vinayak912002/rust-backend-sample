use crate::models::user::{User, CreateUser};
use crate::repositories::user_repository::UserRepository;
use sqlx::Error;

pub struct UserService{
    repository:UserRepository,
}

impl UserService{
    
    pub fn new(repository:UserRepository)->Self{
        Self{repository}
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

        self.repository.find_by_id(id).await
    }


    pub async fn get_all_users(&self)->Result<Vec<User>, Error>{

        self.repository.find_all().await
    }

    pub async fn update_user(&self, id:&str, name:String, age:i32)->Result<User, Error>{
        if name.trim().is_empty(){
            return Err(Error::RowNotFound);
        }
        let update_user = CreateUser{name, age};
        self.repository.update(id, update_user).await
    }

    pub async fn delete_user(&self, id:&str)->Result<(), Error>{
        self.repository.delete(id).await
    }

}
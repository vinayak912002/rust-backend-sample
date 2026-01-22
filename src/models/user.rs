use serde::{Serialize, Deserialize};
use sqlx::FromRow;

//Serialize and Deserialize help in converting structs to json and vice versa
// the trait Debug helps to print the struct
// Clone enables creating deep copy of the model
// FromRow helps in mapping the database rows to the struct fields 
#[derive(Serialize, Deserialize, Debug, Clone,FromRow)]
pub struct User{
    pub id:String,
    pub name:String,
    pub age:i32,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CreateUser{
    pub name:String,
    pub age:i32,
}
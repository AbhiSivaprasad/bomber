use bson::{doc, oid::ObjectId};
use failure::Fail;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Fail)]
pub enum UserError {
    #[fail(display = "NotFound")]
    NotFound,
    #[fail(display = "Database")]
    DB,
    #[fail(display = "Other")]
    Other,
}

type Result<T> = std::result::Result<T, UserError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    id: ObjectId,
    pub username: String,
    password: String,
}

fn hash(password: &str) -> String {
    Sha256::new()
        .chain(password)
        .result()
        .iter()
        .map(|x| format!("{:x?}", x))
        .collect()
}

impl User {
    pub fn new(username: &str, password: &str) -> Result<User> {
        let hash = hash(password);
        Ok(User {
            id: ObjectId::new().map_err(|_| UserError::Other)?,
            username: username.to_string(),
            password: hash,
        })
    }

    pub fn load(db: &Database, username: &str) -> Result<User> {
        let collection = db.collection("users");
        let filter = doc! { "username": username };
        let document = collection
            .find_one(filter, None)
            .map_err(|_| UserError::DB)?;
        if let None = document {
            Err(UserError::NotFound)?;
        }
        Ok(bson::from_bson(bson::Bson::Document(document.unwrap()))
            .map_err(|_| UserError::Other)?)
    }

    pub fn login(db: &Database, username: &str, password: &str) -> Result<User> {
        let hash = hash(password);
        let collection = db.collection("users");
        let filter = doc! { "username": username, "password": hash };
        let document = collection
            .find_one(filter, None)
            .map_err(|_| UserError::DB)?;
        if let None = document {
            Err(UserError::NotFound)?;
        }
        Ok(bson::from_bson(bson::Bson::Document(document.unwrap()))
            .map_err(|_| UserError::Other)?)
    }

    pub fn save(&self, db: &Database) -> Result<()> {
        let collection = db.collection("users");
        let serialized = bson::to_bson(self).map_err(|_| UserError::Other)?;
        if let bson::Bson::Document(document) = serialized {
            collection
                .insert_one(document, None)
                .map_err(|_| UserError::DB)?;
        } else {
            Err(UserError::Other)?;
        }
        Ok(())
    }

    pub fn delete(&self, db: &Database) -> Result<()> {
        let collection = db.collection("users");
        let filter = doc! { "_id": self.id };
        let result = collection
            .delete_one(filter, None)
            .map_err(|_| UserError::DB)?;
        if result.deleted_count == 0 {
            Err(UserError::NotFound)?;
        }
        Ok(())
    }

    pub fn set_password(&mut self, password: &str) {
        let hash = hash(password);
        self.password = hash;
    }
}

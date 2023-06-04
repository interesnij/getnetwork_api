use crate::schema::{
    chats,
    messages,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::schema;
use crate::errors::Error;
use crate::models::SmallFile;


#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Chat {
    pub id:      i32,
    pub user_id: i32,
    pub created: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="chats"]
pub struct NewChat {
    pub user_id: i32,
    pub created: chrono::NaiveDateTime,
}

// view
// 1. создано
// 2. показано
// 3. прочитано

// types
// 1. обычное
// 2. измененное
// 3. удаленное

#[derive(Debug ,Queryable, Serialize, Identifiable)]
pub struct Message {
    pub id:      i32,
    pub user_id: i32,
    pub chat_id: i32,
    pub created: chrono::NaiveDateTime,
    pub content: Option<String>,
    pub view:    i16,
    pub types:   i16,
}

impl Message {
    pub fn get_files(&self) -> (Vec<&SmallFile>, Vec<&SmallFile>, Vec<&SmallFile>, Vec<&SmallFile>) { 
        use schema::files::dsl::files;

        let mut photos = Vec::new();
        let mut videos = Vec::new();
        let mut audios = Vec::new();
        let mut docs = Vec::new();

        let _connection = establish_connection();
        let _files = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .select((
                schema::files::id,
                schema::files::types,
                schema::files::src,
                schema::files::description.nullable()
            )) 
            .load::<SmallFile>(&_connection)
            .expect("E");
        
        for file in _files.iter() {
            match file.types {
                11 => photos.push(file),
                12 => videos.push(file),
                13 => audios.push(file),
                14 => docs.push(file),
            };
        }
        return (photos, videos, audios, docs);
    }

    pub fn create (
        &self,
        user_id: i32,
        chat_id: i32,
        content: Option<String>,
        photos:  Option<Vec<String>>,
        videos:  Option<Vec<String>>,
        audios:  Option<Vec<String>>,
        docs:    Option<Vec<String>>,
    ) -> Result<Message, Error> {
        use chrono::Duration;
        use crate::models::NewFile;

        let _connection = establish_connection();

        let new_message_form = NewMessage {
            user_id: user_id,
            chat_id: chat_id,
            created: chrono::Local::now().naive_utc() + Duration::hours(3),
            content: content,
            view:    1,
            types:   1,
        };

        let _message = diesel::insert_into(schema::messages::table)
            .values(&new_message_form)
            .get_result::<Message>(&_connection)?;
        let _id = _message.id;
        if photos.is_some() {
            for i in photos.unwrap() {
                NewFile::create(user_id, _id, 11, 1, i.clone());
            }
        }
        if videos.is_some() {
            for i in videos.unwrap() {
                NewFile::create(user_id, _id, 11, 2, i.clone());
            }
        }
        if audios.is_some() {
            for i in audios.unwrap() {
                NewFile::create(user_id, _id, 11, 3, i.clone());
            }
        }
        if docs.is_some() {
            for i in docs.unwrap() {
                NewFile::create(user_id, _id, 11, 4, i.clone());
            }
        }
        return Ok(_message);
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="messages"]
pub struct NewMessage {
    pub user_id: i32,
    pub chat_id: i32,
    pub created: chrono::NaiveDateTime,
    pub content: Option<String>,
    pub view:    i16,
    pub types:   i16,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="messages"]
pub struct EditMessage {
    pub content: Option<String>,
}

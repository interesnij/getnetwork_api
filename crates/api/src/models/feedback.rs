use crate::schema;
use crate::schema::feedbacks;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::errors::Error;


#[derive(Debug ,Queryable, Serialize, Identifiable)]
pub struct Feedback {
    pub id:       i32,
    pub user_id:  i32,    // cookie user
    pub username: String,
    pub email:    String,
    pub message:  String,
}
impl Feedback {
    pub fn get_list(page: i32, limit: i32) -> Result<(Vec<Feedback>, i32), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Feedback>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = schema::feedbacks::table
                .limit(limit.into())
                .offset(step.into())
                .load::<Feedback>(&_connection)
                .expect("E");
        } 
        else {
            have_next = limit + 1;
            object_list = schema::feedbacks::table
                .limit(limit.into())
                .offset(0)
                .load::<Feedback>(&_connection)
                .expect("E");
        }
        if schema::feedbacks::table
            .limit(1)
            .offset(have_next.into())
            .select(schema::feedbacks::id)
            .load::<i32>(&_connection)
            .expect("E")
            .len() > 0 {
                next_page_number = page + 1;
        }
        let _tuple = (object_list, next_page_number);
        Ok(_tuple)
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="feedbacks"]
pub struct NewFeedback {
    pub user_id:  i32,
    pub username: String,
    pub email:    String,
    pub message:  String,
}

use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
    NullableExpressionMethods,
    PgTextExpressionMethods,
};
use serde::{Serialize,Deserialize};
use crate::models::{
    User,
    Tag,
    TechCategories,
    Serve,
    SmallTag,
    SmallFile,
};

use crate::schema::{
    categories,
    items,
    category,
    item_comments,
    item_contents,
};
use crate::utils::{establish_connection, OwnerResp,};
use crate::errors::Error;


/////////// 
// types:
// 1. блог
// 2. услуга
// 3. товар
// 4. wiki
// 5. работа
// 6. помощь
// 7. заказ

#[derive(Serialize, Queryable)]
pub struct CatDetail {
    pub name:    String,
    pub slug:    String,
    pub count:   i16,
    pub id:      i32,
    pub image:   Option<String>,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
impl CatDetail {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
}

#[derive(Serialize, Queryable, Clone)]
pub struct Cat {
    pub name:  String,
    pub slug:  String,
    pub count: i16,
    pub id:    i32,
    pub image: Option<String>,
    pub types: i16,
}
impl Cat {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_items_list (
        &self,
        limit:    i64,
        types:    i16,
        is_admin: bool
    ) -> Vec<Item> {
        let _connection = establish_connection();
        let ids = schema::category::table
            .filter(schema::category::categories_id.eq(self.id))
            .filter(schema::category::types.eq(types))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .load::<Item>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .load::<Item>(&_connection)
                .expect("E.");
        }
    }
}

#[derive(Serialize, Queryable)]
pub struct SmallCat {
    pub name:  String,
    pub slug:  String,
    pub count: i16,
}

#[derive(Serialize, Queryable)]
pub struct Blog {
    pub id:          i32,
    pub slug:        String,
    pub image:       Option<String>,
    pub description: Option<String>,
    pub types:       i16,
    pub title:       String,
    pub created:     chrono::NaiveDateTime,
}

#[derive(Serialize, Queryable)]
pub struct ContentBlock {
    pub id:       i32,
    pub title:    String,
    pub content:  String,
    pub user:     OwnerResp,
    pub position: i16,
    pub created:  String,
}

impl Blog {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self) -> Vec<SmallTag> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(1))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _tags = tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("E");
        return _tags;
    }
}

#[derive(Serialize, Queryable)]
pub struct Service {
    pub id:          i32,
    pub slug:        String,
    pub image:       Option<String>,
    pub description: Option<String>,
    pub types:       i16,
    pub title:       String,
}
impl Service {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self) -> Vec<SmallTag> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(2))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _tags = tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("E");
        return _tags;
    }
}

#[derive(Serialize, Queryable)]
pub struct Store {
    pub id:          i32,
    pub slug:        String,
    pub image:       Option<String>,
    pub description: Option<String>,
    pub types:       i16,
    pub title:       String,
    pub price:       i32,
    pub price_acc:   Option<i32>,
}
impl Store {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self) -> Vec<SmallTag> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(3))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _tags = tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("E");
        return _tags;
    }
}

#[derive(Serialize, Queryable)]
pub struct Wiki {
    pub id:          i32,
    pub slug:        String,
    pub image:       Option<String>,
    pub description: Option<String>,
    pub types:       i16,
    pub title:       String,
    pub created:     chrono::NaiveDateTime,
}
impl Wiki {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self) -> Vec<SmallTag> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(4))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _tags = tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("E");
        return _tags;
    }
}

#[derive(Serialize, Queryable)]
pub struct Work {
    pub id:          i32,
    pub slug:        String,
    pub image:       Option<String>,
    pub description: Option<String>,
    pub types:       i16,
    pub title:       String,
}
impl Work {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self) -> Vec<SmallTag> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(5))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _tags = tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("E");
        return _tags;
    } 
}

#[derive(Serialize, Queryable)]
pub struct Help {
    pub id:          i32,
    pub title:       String,
    pub description: Option<String>,
    pub types:       i16,
}
impl Help {
    pub fn get_category(&self) -> SmallCat {
        use crate::schema::{
            category::dsl::category,
            categories::dsl::categories,
        };
        let _connection = establish_connection();
        let _id = category
            .filter(schema::category::item_id.eq(self.id))
            .filter(schema::category::types.eq(6))
            .select(schema::category::categories_id)
            .first::<i32>(&_connection)
            .expect("E");

        let _category = categories
            .filter(schema::categories::id.eq(_id))
            .select((
                schema::categories::name,
                schema::categories::slug,
                schema::categories::count
            ))
            .first::<SmallCat>(&_connection)
            .expect("E");
        return _category;
    }
    pub fn get_tags(&self) -> Vec<SmallTag> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(6))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _tags = tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("E");
        return _tags;
    }
}

#[derive(Serialize, Queryable)]
pub struct FeaturedItem {
    pub slug:  String,
    pub title: String,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[table_name="categories"]
pub struct Categories {
    pub id:          i32,
    pub name:        String,
    pub user_id:     i32,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
    pub count:       i16,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
    pub types:       i16,
    pub slug:        String,
}

impl Categories {
    pub fn get_tags(types: i16) -> Result<Vec<SmallTag>, Error> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::types.eq(types))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _tags = tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("E");
        return Ok(_tags);
    }
    pub fn get_featured_items(&self, types: i16, id: i32) -> (Option<FeaturedItem>, Option<FeaturedItem>) {
        use crate::schema::{
            category::dsl::category,
            items::dsl::items,
        };

        let _connection = establish_connection();

        let mut prev: Option<FeaturedItem> = None;
        let mut next: Option<FeaturedItem> = None;

        let _category_items = category
            .filter(schema::category::categories_id.eq(self.id))
            .filter(schema::category::types.eq(types))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _category_items_len = _category_items.len();
        for (i, item) in _category_items.iter().enumerate().rev() {
            if item == &id {
                if (i + 1) != _category_items_len {
                    let _next = Some(&_category_items[i + 1]);
                    next = Some(items
                        .filter(schema::items::id.eq(_next.unwrap()))
                        .filter(schema::items::types.eq(types))
                        .filter(schema::items::item_types.lt(10))
                        .select((
                            schema::items::slug,
                            schema::items::title,
                        ))
                        .first::<FeaturedItem>(&_connection)
                        .expect("E."));
                };
                if i != 0 {
                    let _prev = Some(&_category_items[i - 1]);
                    prev = Some(items
                        .filter(schema::items::id.eq(_prev.unwrap()))
                        .filter(schema::items::types.eq(types))
                        .filter(schema::items::item_types.lt(10))
                        .select((
                            schema::items::slug,
                            schema::items::title,
                        ))
                        .first::<FeaturedItem>(&_connection)
                        .expect("E."));
                };
                break;
            }
        };
        return (prev, next);
    }
    pub fn get_type(&self) -> String {
        return match self.types {
            1 => "блог".to_string(),
            2 => "услуга".to_string(),
            3 => "товар".to_string(),
            4 => "wiki".to_string(),
            5 => "работа".to_string(),
            6 => "помощь".to_string(),
            _ => "Непонятно".to_string(),
        };
    }
    pub fn get_stat_type(types: i16) -> i16 {
        return match self.types {
            1 => 41,
            2 => 61,
            3 => 71,
            4 => 81,
            5 => 91,
            6 => 101,
            _ => 41,
        };
    } 

    pub fn get_blogs_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> Result<(Vec<Blog>, i16), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Blog>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_blogs(cat_id, limit.into(), step.into(), is_admin)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_blogs(cat_id, limit.into(), 0, is_admin)?;
        }
        if Categories::get_blogs(cat_id, 1, have_next.into(), is_admin)?.len() > 0 {
            next_page_number = page + 1;
        }
        let _tuple = (object_list, next_page_number);
        Ok(_tuple)
    }
    pub fn get_blogs (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Result<Vec<Blog>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let _items: Vec<Blog>;
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(1))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             _items = items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        } else {
            _items = items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
        return Ok(_items);
    }
    pub fn get_services_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> Result<(Vec<Service>, i16), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Service>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_services(cat_id, limit.into(), step.into(), is_admin)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_services(cat_id, limit.into(), 0, is_admin)?;
        }
        if Categories::get_services(cat_id, 1, have_next.into(), is_admin)?.len() > 0 {
            next_page_number = page + 1;
        }

        return Ok((object_list, next_page_number));
    }
    pub fn get_services (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Result<Vec<Service>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let _items: Vec<Service>;
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(2))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             _items = items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        } else {
            _items = items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        }
        return Ok(_items);
    }

    pub fn get_stores_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> Result<(Vec<Store>, i16), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Store>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_stores(cat_id, limit.into(), step.into(), is_admin)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_stores(cat_id, limit.into(), 0, is_admin)?;
        }
        if Categories::get_stores(cat_id, 1, have_next.into(), is_admin)?.len() > 0 {
            next_page_number = page + 1;
        }

        return Ok((object_list, next_page_number));
    }
    pub fn get_stores (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Result<Vec<Store>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let _items: Vec<Store>;
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(3))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             _items = items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::price,
                    schema::items::price_acc.nullable(),
                ))
                .load::<Store>(&_connection)
                .expect("E.");
        } else {
            _items = items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::price,
                    schema::items::price_acc.nullable(),
                ))
                .load::<Store>(&_connection)
                .expect("E.");
        }
        return Ok(_items);
    }

    pub fn get_wikis_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> Result<(Vec<Wiki>, i16), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Wiki>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_wikis(cat_id, limit.into(), step.into(), is_admin)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_wikis(cat_id, limit.into(), 0, is_admin)?;
        }
        if Categories::get_wikis(cat_id, 1, have_next.into(), is_admin)?.len() > 0 {
            next_page_number = page + 1;
        }

        return Ok((object_list, next_page_number));
    }
    pub fn get_wikis (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Result<Vec<Wiki>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let _items: Vec<Wiki>;
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(4))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             _items = items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created
                )) 
                .load::<Wiki>(&_connection)
                .expect("E.");
        } else {
            _items = items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created
                ))
                .load::<Wiki>(&_connection)
                .expect("E.");
        }
        return Ok(_items);
    }

    pub fn get_works_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> Result<(Vec<Work>, i16), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Work>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_works(cat_id, limit.into(), step.into(), is_admin)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_works(cat_id, limit.into(), 0, is_admin)?;
        }
        if Categories::get_works(cat_id, 1, have_next.into(), is_admin)?.len() > 0 {
            next_page_number = page + 1;
        }

        return Ok((object_list, next_page_number));
    }
    pub fn get_works (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Result<Vec<Work>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let _items: Vec<Work>;
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(5))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             _items = items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Work>(&_connection)
                .expect("E.");
        } else {
            _items = items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Work>(&_connection)
                .expect("E.");
        }
        return Ok(_items);
    }

    pub fn get_helps_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool
    ) -> Result<(Vec<Help>, i16), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Help>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_helps(cat_id, limit.into(), step.into(), is_admin)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_helps(cat_id, limit.into(), 0, is_admin)?;
        }
        if Categories::get_helps(cat_id, 1, have_next.into(), is_admin)?.len() > 0 {
            next_page_number = page + 1;
        }

        return Ok((object_list, next_page_number));
    }
    pub fn get_helps (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Result<Vec<Help>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let _items: Vec<Help>;
        let ids = category
            .filter(schema::category::categories_id.eq(cat_id))
            .filter(schema::category::types.eq(6))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
             _items = items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::position.asc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::item_types,
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        } else {
            _items = items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::position.asc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::item_types,
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        }
        return Ok(_items);
    }

    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_categories_for_types(types: i16) -> Result<Vec<Cat>, Error> {
        use crate::schema::categories::dsl::categories;
        let _connection = establish_connection();
        let cats = categories
            .filter(schema::categories::types.eq(types))
            .select((
                schema::categories::name,
                schema::categories::slug,
                schema::categories::count,
                schema::categories::id,
                schema::categories::image,
                schema::categories::types,
            ))
            .load::<Cat>(&_connection)
            .expect("E");
        return Ok(cats);
    }
    pub fn get_categories() -> Result<Vec<Cat>, Error> {
        use crate::schema::categories::dsl::categories;
        let _connection = establish_connection();
        let cats = categories
            .select((
                schema::categories::name,
                schema::categories::slug,
                schema::categories::count,
                schema::categories::id,
                schema::categories::image,
                schema::categories::types,
            ))
            .load::<Cat>(&_connection)
            .expect("E");
        return Ok(cats);
    }
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategories {
    pub name:        String,
    pub user_id:     i32,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
    pub count:       i16,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
    pub types:       i16,
    pub slug:        String,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="categories"]
pub struct EditCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
    pub slug:        String,
}

#[derive(Debug, Serialize, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Item {
    pub id:          i32,
    pub title:       String,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub description: Option<String>,
    pub item_types:  i16,
    pub price:       i32,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
    pub position:    i16,
    pub price_acc:   Option<i32>,
    pub types:       i16,
    pub slug:        String,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}

impl Item {
    pub fn get_owner(id: i32) -> OwnerResp {
        use schema::users::dsl::users;

        let user_ok = users
            .filter(schema::users::id.eq(id))
            .select((
                schema::users::first_name, 
                schema::users::last_name, 
                schema::users::username, 
                schema::users::image, 
                schema::users::perm
            ))
            .first::<OwnerResp>(&_connection);
        if user_ok.is_ok() {
            return user_ok.expect("E");
        }
        return OwnerResp {
            first_name: "".to_string(),
            last_name:  "".to_string(),
            username:   "".to_string(),
            image:      None,
            perm:       0,
        };
    }
    pub fn get_contents(&self) -> Vec<ContentBlock> {
        use schema::item_contents::dsl::item_contents;

        let mut stack = Vec::new();
        let contents = item_contents
            .filter(schema::items::item_id.eq(self.id))
            .select(( 
                schema::users::id,
                schema::users::title,
                schema::users::content,
                schema::users::user_id,
                schema::users::position,
                schema::users::created,
            ))
            .load::<(i32, String, String, i32, i16, NaiveDateTime)>(&_connection)
            .expect("E");
        for c in contents.iter() {
            stack.push(ContentBlock {
                id:       c.0,
                title:    c.1.clone(),
                content:  c.2.clone(),
                user:     Item::get_owner(c.3),
                position: c.4,
                created:  c.5.to_string(),
            });
        }
        return stack;
    }
    pub fn get_type(&self) -> String {
        return match self.types {
            1 => "блог".to_string(),
            2 => "услуга".to_string(),
            3 => "товар".to_string(),
            4 => "wiki".to_string(),
            5 => "работа".to_string(),
            6 => "помощь".to_string(),
            _ => "Непонятно".to_string(),
        };
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
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
                1 => photos.push(file),
                2 => videos.push(file),
                3 => audios.push(file),
                4 => docs.push(file),
            };
        }
        return (photos, videos, audios, docs);
    }
    pub fn get_images_ids(&self) -> Vec<i32> {
        use schema::files::dsl::files;

        let _connection = establish_connection();
        return files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::types.eq(1))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E");
    }

    pub fn get_categories(&self) -> Result<Vec<SmallCat>, Error> {
        use crate::schema::{
            category::dsl::category,
            categories::dsl::categories,
        };
        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::item_id.eq(self.id))
            .filter(schema::category::types.eq(self.types))
            .select(schema::category::categories_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _categories = categories
            .filter(schema::categories::id.eq_any(ids))
            .select((
                schema::categories::name,
                schema::categories::slug,
                schema::categories::count
            ))
            .load::<SmallCat>(&_connection)
            .expect("E");
        return Ok(_categories);
    }
    pub fn get_categories_obj(&self) -> Result<Vec<Categories>, Error> {
        use crate::schema::{
            category::dsl::category,
            categories::dsl::categories,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::item_id.eq(self.id))
            .filter(schema::category::types.eq(self.types))
            .select(schema::category::categories_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _categories = categories
            .filter(schema::categories::id.eq_any(ids))
            .load::<Categories>(&_connection)
            .expect("E");
        return Ok(_categories);
    }

    pub fn get_tags(&self) -> Result<Vec<SmallTag>, Error> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(&self.id))
            .filter(schema::tags_items::types.eq(self.types))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _tags = tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .select((schema::tags::name, schema::tags::count))
            .load::<SmallTag>(&_connection)
            .expect("E");
        return Ok(_tags);
    }
    pub fn get_tags_obj(&self) -> Result<Vec<Tag>, Error> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(&self.id))
            .filter(schema::tags_items::types.eq(self.types))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _tags = tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .load::<Tag>(&_connection)
            .expect("E");
        return Ok(_tags);
    }

    pub fn get_blogs (
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Blog> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::types.eq(1))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(1))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
    }
    pub fn search_blogs (
        q:        &String,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> (Vec<Blog>, usize) {
        use crate::schema::{
            items::dsl::items,
            item_contents::dsl::item_contents,
        };

        let _connection = establish_connection();
        let item_ids = item_contents
            .filter(schema::item_contents::item_title.ilike(&q))
            .or_filter(schema::item_contents::title.ilike(&q))
            .or_filter(schema::item_contents::content.ilike(&q))
            .filter(schema::item_contents::item_types.eq(1))
            .select(schema::item_contents::item_id)
            .distinct()
            .load::<i32>(&_connection)
            .expect("E.");

        if is_admin {
             return (items
                .filter(schema::items::eq_any(item_ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Blog>(&_connection)
                .expect("E."), item_ids.len());
        } else {
            return (items
                .filter(schema::items::eq_any(item_ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Blog>(&_connection)
                .expect("E."), item_ids.len());
        }
    }

    pub fn get_services (
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Service> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::types.eq(2))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(2))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        }
    }
    pub fn search_services (
        q:        &String,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> (Vec<Service>, usize) {
        use crate::schema::{
            items::dsl::items,
            item_contents::dsl::item_contents,
        };

        let _connection = establish_connection();
        let item_ids = item_contents
            .filter(schema::item_contents::item_title.ilike(&q))
            .or_filter(schema::item_contents::title.ilike(&q))
            .or_filter(schema::item_contents::content.ilike(&q))
            .filter(schema::item_contents::item_types.eq(2))
            .select(schema::item_contents::item_id)
            .distinct()
            .load::<i32>(&_connection)
            .expect("E.");

        if is_admin {
             return (items
                .filter(schema::items::eq_any(item_ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Service>(&_connection)
                .expect("E."), item_ids.len());
        } else {
            return (items
                .filter(schema::items::eq_any(item_ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Service>(&_connection)
                .expect("E."), item_ids.len());
        }
    }

    pub fn get_stores (
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Store> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::types.eq(3))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::price,
                    schema::items::price_acc.nullable(),
                ))
                .load::<Store>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(3))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::price,
                    schema::items::price_acc.nullable(),
                ))
                .load::<Store>(&_connection)
                .expect("E.");
          }
    }
    pub fn search_stores (
          q:        &String,
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> (Vec<Store>, usize) {
        use crate::schema::{
            items::dsl::items,
            item_contents::dsl::item_contents,
        };

        let _connection = establish_connection();
        let item_ids = item_contents
            .filter(schema::item_contents::item_title.ilike(&q))
            .or_filter(schema::item_contents::title.ilike(&q))
            .or_filter(schema::item_contents::content.ilike(&q))
            .filter(schema::item_contents::item_types.eq(3))
            .select(schema::item_contents::item_id)
            .distinct()
            .load::<i32>(&_connection)
            .expect("E.");
        if is_admin {
            return (items
                .filter(schema::items::eq_any(item_ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::price,
                    schema::items::price_acc.nullable(),
                ))
                .load::<Store>(&_connection)
                .expect("E."), item_ids.len());
        } else {
            return (items
                .filter(schema::items::eq_any(item_ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::price,
                    schema::items::price_acc.nullable(),
                ))
                .load::<Store>(&_connection)
                .expect("E."), item_ids.len());
          }
    }

    pub fn get_works (
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Work> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::types.eq(5))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Work>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(5))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Work>(&_connection)
                .expect("E.");
        }
    }
    pub fn search_works (
          q:        &String,
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> (Vec<Work>, usize) {
        use crate::schema::{
            items::dsl::items,
            item_contents::dsl::item_contents,
        };

        let _connection = establish_connection();
        let item_ids = item_contents
            .filter(schema::item_contents::item_title.ilike(&q))
            .or_filter(schema::item_contents::title.ilike(&q))
            .or_filter(schema::item_contents::content.ilike(&q))
            .filter(schema::item_contents::item_types.eq(5))
            .select(schema::item_contents::item_id)
            .distinct()
            .load::<i32>(&_connection)
            .expect("E.");
        if is_admin {
            return (items
                .filter(schema::items::eq_any(item_ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Work>(&_connection)
                .expect("E."), item_ids.len());
        } else {
            return (items
                .filter(schema::items::eq_any(item_ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Work>(&_connection)
                .expect("E."), item_ids.len());
        }
    }

    pub fn get_wikis (
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Wiki> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::types.eq(4))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Wiki>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(4))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created
                ))
                .load::<Wiki>(&_connection)
                .expect("E.");
        }
    }
    pub fn search_wikis (
          q:        &String,
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> (Vec<Wiki>, usize) {
        use crate::schema::{
            items::dsl::items,
            item_contents::dsl::item_contents,
        };

        let _connection = establish_connection();
        let item_ids = item_contents
            .filter(schema::item_contents::item_title.ilike(&q))
            .or_filter(schema::item_contents::title.ilike(&q))
            .or_filter(schema::item_contents::content.ilike(&q))
            .filter(schema::item_contents::item_types.eq(4))
            .select(schema::item_contents::item_id)
            .distinct()
            .load::<i32>(&_connection)
            .expect("E.");
        if is_admin {
            return (items
                .filter(schema::items::eq_any(item_ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created
                ))
                .load::<Wiki>(&_connection)
                .expect("E."), item_ids.len());
        } else {
            return (items
                .filter(schema::items::eq_any(item_ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Wiki>(&_connection)
                .expect("E."), item_ids.len());
        }
    }

    pub fn get_helps (
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Help> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::types.eq(6))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::item_types,
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(6))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::item_types,
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        }
    }

    pub fn search_helps (
        q:        &String,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> (Vec<Help>, usize) {
        use crate::schema::{
            items::dsl::items,
            item_contents::dsl::item_contents,
        };

        let _connection = establish_connection();
        let item_ids = item_contents
            .filter(schema::item_contents::item_title.ilike(&q))
            .or_filter(schema::item_contents::title.ilike(&q))
            .or_filter(schema::item_contents::content.ilike(&q))
            .filter(schema::item_contents::item_types.eq(6))
            .select(schema::item_contents::item_id)
            .distinct()
            .load::<i32>(&_connection)
            .expect("E.");
        if is_admin {
            return (items
                .filter(schema::items::eq_any(item_ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::item_types,
                ))
                .load::<Help>(&_connection)
                .expect("E."), item_ids.len());
        } else {
            return (items
                .filter(schema::items::eq_any(item_ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::item_types,
                ))
                .load::<Help>(&_connection)
                .expect("E."), item_ids.len());
        }
    }

    pub fn get_blogs_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Blog>, i16) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Blog>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_blogs_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_blogs_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_blogs_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_blogs_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Blog> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_services_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Service>, i16) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Service>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_services_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_services_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_services_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_services_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Service> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_stores_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Store>, i16) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Store>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_stores_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_stores_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_stores_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_stores_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Store> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::price,
                    schema::items::price_acc.nullable()
                ))
                .load::<Store>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::price,
                    schema::items::price_acc.nullable()
                ))
                .load::<Store>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_wikis_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Wiki>, i16) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Wiki>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_wikis_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_wikis_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_wikis_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_wikis_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Wiki> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Wiki>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                    schema::items::created,
                ))
                .load::<Wiki>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_works_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Work>, i16) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Work>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_works_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_works_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_works_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_works_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Work> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Work>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::description.nullable(),
                    schema::items::item_types,
                    schema::items::title,
                ))
                .load::<Work>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_helps_list_for_ids (
        page:  i32,
        limit: i32,
        ids:   &Vec<i32>,
        is_admin: bool
    ) -> (Vec<Help>, i16) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Help>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_helps_for_ids(limit.into(), step.into(), &ids, is_admin);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_helps_for_ids(limit.into(), 0, &ids, is_admin);
        }
        if Item::get_helps_for_ids(1, have_next.into(), &ids, is_admin).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_helps_for_ids (
        limit:  i64,
        offset: i64,
        ids:    &Vec<i32>,
        is_admin: bool
    ) -> Vec<Help> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
            return items
                .filter(schema::items::id.eq_any(ids))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::item_types,
                )) 
                .load::<Help>(&_connection)
                .expect("E.");
        }
        else {
            return items
                .filter(schema::items::id.eq_any(ids))
                .filter(schema::items::item_types.lt(10))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::title,
                    schema::items::description.nullable(),
                    schema::items::item_types,
                )) 
                .load::<Help>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_serves_ids(&self) -> Vec<i32> {
        use schema::serve_items::dsl::serve_items;

        let _connection = establish_connection();
        return serve_items
            .filter(schema::serve_items::item_id.eq(&self.id))
            .filter(schema::serve_items::types.eq(self.types))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_serves(&self) -> Vec<Serve> {
        use schema::{
            serve_items::dsl::serve_items,
            serve::dsl::serve,
        };

        let _connection = establish_connection();
        let _items = serve_items
            .filter(schema::serve_items::item_id.eq(&self.id))
            .filter(schema::serve_items::types.eq(self.types))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");

        return serve
            .filter(schema::serve::id.eq_any(_items))
            .load::<Serve>(&_connection)
            .expect("E");
    }
    pub fn get_open_tech_categories(&self, types: i16) -> Vec<TechCategories> {
        // получаем открытые тех.категории элемента
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(types))
            .filter(schema::tech_categories_items::is_active.eq(1))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return tech_categories
            .filter(schema::tech_categories::id.eq_any(ids))
            .order(schema::tech_categories::position.desc())
            .load::<TechCategories>(&_connection)
            .expect("E");
    }
    pub fn get_close_tech_categories(&self, types: i16) -> Vec<TechCategories> {
        // получаем закрытые тех.категории элемента
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(types))
            .filter(schema::tech_categories_items::is_active.eq(2))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return tech_categories
            .filter(schema::tech_categories::id.eq_any(ids))
            .order(schema::tech_categories::position.desc())
            .load::<TechCategories>(&_connection)
            .expect("E");
    }
    pub fn get_close_tech_cats_ids(&self, types: i16) -> Vec<i32> {
        use schema::tech_categories_items::dsl::tech_categories_items;

        let _connection = establish_connection();
        return tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(types))
            .filter(schema::tech_categories_items::is_active.eq(2))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");
    }

    pub fn make_hide(&self) -> i16 {
        let _connection = establish_connection();
        let _case = match self.item_types {
            1 => 11,
            2 => 12,
            3 => 13,
            4 => 14,
            5 => 15,
            _ => 11,
        };
        let o = diesel::update(self)
            .set(schema::items::item_types.eq(_case))
            .execute(&_connection);

        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
    pub fn make_publish(&self) -> i16 {
        let _connection = establish_connection();
        let _case = match self.item_types {
            11 => 1,
            12 => 2,
            13 => 3,
            14 => 4,
            15 => 5,
            _ => 1,
        };
        let o = diesel::update(self)
            .set(schema::items::item_types.eq(_case))
            .execute(&_connection);

        if o.is_ok() {
            return 1;
        }
        else {
            return 0;
        }
    }
}

#[derive(Serialize, Insertable)]
#[table_name="items"]
pub struct NewItem {
    pub title:       String,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub description: Option<String>,
    pub item_types:  i16,
    pub price:       i32,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
    pub position:    i16,
    pub price_acc:   Option<i32>,
    pub types:       i16,
    pub slug:        String,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}

impl NewItem {
    pub fn create (
        title:    String,
        link:     Option<String>,
        image:    Option<String>,
        user_id:  i32,
        position: i16,
        types:    i16,
        slug:     String
    ) -> Self {
        use chrono::Duration;

        NewItem {
            title:       title,
            link:        link,
            image:       image,
            description: None,
            item_types:  1,
            price:       0,
            user_id:     user_id,
            created:     chrono::Local::now().naive_utc() + Duration::hours(3),
            position:    position,
            price_acc:   None,
            types:       types,
            slug:        slug,
            view:        0,
            height:      0.0,
            seconds:     0,
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="items"]
pub struct EditItem {
    pub title:       String,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub position:    i16,
    pub slug:        String,
}


///////////
// types:
// 1. блог
// 2. услуга
// 3. товар
// 4. wiki
// 5. работа
// 6. помощь
// 7. заказ
// 8. веб-сервис
// 9. язык / технология
// 10. опция
#[derive(Identifiable, Queryable, Associations)]
#[table_name="category"]
pub struct Category {
    pub id:            i32,
    pub categories_id: i32,
    pub item_id:       i32,
    pub types:         i16,
}

#[derive(Insertable)]
#[table_name="category"]
pub struct NewCategory {
    pub categories_id: i32,
    pub item_id:       i32,
    pub types:         i16,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
pub struct ItemComment {
    pub id:        i32,
    pub comment:   String,
    pub item_id:   i32,
    pub user_id:   i32,
    pub parent_id: Option<i32>,
    pub created:   chrono::NaiveDateTime,
}

#[derive(Serialize, Insertable)]
#[table_name="item_comments"]
pub struct NewItemComment {
    pub comment:   String,
    pub item_id:   i32,
    pub user_id:   i32,
    pub parent_id: Option<i32>,
    pub created:   chrono::NaiveDateTime,
}

impl NewItemComment {
    pub fn new (comment: String, item_id: i32,
        user_id: i32, parent_id: Option<i32>) -> Self {
        use chrono::Duration;

        NewItemComment {
            comment:   comment,
            item_id:   item_id,
            user_id:   user_id,
            parent_id: parent_id,
            created:   chrono::Local::now().naive_utc() + Duration::hours(3),
        }
    }
}

////////////
#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
pub struct ItemContent {
    pub id:         i32,
    pub title:      String,
    pub item_title: String,
    pub content:    String,
    pub types:      i16,
    pub item_id:    i32,
    pub item_types: i16,
    pub user_id:    i32,
    pub position:   i16,
    pub created:    chrono::NaiveDateTime,
}

#[derive(Serialize, Insertable)]
#[table_name="item_contents"]
pub struct NewItemContent {
    pub title:      String,
    pub item_title: String,
    pub content:    String,
    pub types:      i16,
    pub item_id:    i32,
    pub item_types: i16,
    pub user_id:    i32,
    pub position:   i16,
    pub created:    chrono::NaiveDateTime,
}

impl NewItemContent {
    pub fn new (
        title:      String,
        item_title: String,
        content:    String,
        item_id:    i32,
        item_types: i16,
        user_id:    i32,
        position:   i16,
    ) -> Self {
        use chrono::Duration; 

        NewItemContent {
            title:      title,
            item_title: item_title,
            content:    content,
            types:      1,
            item_id:    item_id,
            item_types: item_types,
            user_id:    user_id,
            position:   position,
            created:    chrono::Local::now().naive_utc() + Duration::hours(3),
        }
    }
}

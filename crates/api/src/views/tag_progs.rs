use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::{block, Data, Json},
};
use serde::{Deserialize, Serialize};
use crate::models::User;
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::utils::{
    establish_connection, get_is_ajax,
    get_request_user, get_stat_page,
    get_is_ajax_page,
    ErrorParams, TOKEN, UserResp,
};
use crate::schema;
use crate::models::{
    Tag, SmallTag, NewTag, TagItems, StatPage,
    Item, Blog, Service, Store, Wiki, Work, Help,
    EditTag,
};
use crate::errors::Error;


pub fn tag_routes(config: &mut web::ServiceConfig) {
    config.route("/tags", web::get().to(tags_page));
    config.route("/tag", web::get().to(tag_page));
    config.route("/tag_blogs", web::get().to(tag_blogs_page));
    config.route("/tag_services", web::get().to(tag_services_page));
    config.route("/tag_stores", web::get().to(tag_stores_page));
    config.route("/tag_wikis", web::get().to(tag_wikis_page));
    config.route("/tag_works", web::get().to(tag_works_page));
    config.route("/tag_helps", web::get().to(tag_helps_page));
    config.service(web::resource("/create_tag")
        .route(web::get().to(create_tag_page))
        .route(web::post().to(create_tag))
    );
    config.service(web::resource("/edit_tag")
        .route(web::get().to(edit_tag_page))
        .route(web::post().to(edit_tag))
    );
    config.route("/delete_tag", web::get().to(delete_tag));
}


#[derive(Serialize)]
pub struct CreateTagPageResp {
    pub request_user: User,
    pub all_tags:     Vec<Tag>,
}
pub async fn create_tag_page(req: HttpRequest) -> Result<Json<CreateTagPageResp>, Error> {
    let _request_user = get_request_user(&req, get_is_ajax(&req)).await;
    if _request_user.id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let all_tags = schema::tags::table
        .load::<Tag>(&_connection)
        .expect("Error.");
    
    return Ok(Json(CreateTagPageResp {
        request_user: _request_user,
        all_tags:     all_tags,
    }));
}


pub async fn create_tag(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    let _request_user = get_request_user(&req, 3).await;
    let user_id = _request_user.id;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let form = crate::utils::category_form(payload.borrow_mut(), user_id).await;
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
            
    let new_tag = NewTag {
        name:     form.name.clone(),
        position: form.position,
        count:    0,
        user_id:  _request_user.id,
        view:     0,
        height:   0.0,
        seconds:  0,
    };
    let _new_tag = diesel::insert_into(schema::tags::table)
        .values(&new_tag)
        .execute(&_connection)
        .expect("E.");

    return Ok(Json(1)); 
}

#[derive(Deserialize)]
pub struct TagPageData {
    pub slug:    Option<String>,
    pub is_ajax: Option<i16>,
    pub page:    Option<i32>,
}
#[derive(Serialize)]
pub struct TagPageResp {
    pub request_user:   UserResp,
    pub tag:            Tag,
    pub works_list:     Vec<Work>,
    pub services_list:  Vec<Service>,
    pub wikis_list:     Vec<Wiki>,
    pub blogs_list:     Vec<Blog>,
    pub stores_list:    Vec<Store>,
    pub helps_list:     Vec<Help>,
    pub works_count:    usize,
    pub services_count: usize,
    pub wikis_count:    usize,
    pub blogs_count:    usize,
    pub stores_count:   usize,
    pub helps_count:    usize,
}
pub async fn tag_page(req: HttpRequest) -> Result<Json<TagPageResp>, Error> {
    let params_some = web::Query::<TagPageData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.slug.is_none() && params.slug.as_deref().unwrap() == "" {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }

    let _connection = establish_connection();
    let _request_user = get_request_user(&req, is_ajax).await;
    let is_admin = _request_user.perm == 60;
    let _tag = schema::tags::table
        .filter(schema::tags::name.eq(params.slug.as_deref().unwrap()))
        .first::<Tag>(&_connection)
        .expect("E");
    let _tag_items = schema::tags_items::table
        .filter(schema::tags_items::tag_id.eq(&_tag.id))
        .load::<TagItems>(&_connection)
        .expect("E");

    let mut blog_stack = Vec::new();
    let mut service_stack = Vec::new();
    let mut store_stack = Vec::new();
    let mut wiki_stack = Vec::new();
    let mut work_stack = Vec::new();
    let mut help_stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        match _tag_item.types {
            1 => blog_stack.push(_tag_item.item_id),
            2 => service_stack.push(_tag_item.item_id),
            3 => store_stack.push(_tag_item.item_id),
            4 => wiki_stack.push(_tag_item.item_id),
            5 => work_stack.push(_tag_item.item_id),
            6 => help_stack.push(_tag_item.item_id),
            _ => println!("no value"),
        };
    };

    return Ok(Json(TagPageResp {
        request_user:   _request_user,
        tag:            _tag,
        works_list:     Item::get_works_for_ids(3, 0, &work_stack, is_admin),
        services_list:  Item::get_services_for_ids(3, 0, &service_stack, is_admin),
        wikis_list:     Item::get_wikis_for_ids(3, 0, &wiki_stack, is_admin),
        blogs_list:     Item::get_blogs_for_ids(3, 0, &blog_stack, is_admin),
        stores_list:    Item::get_stores_for_ids(3, 0, &store_stack, is_admin),
        helps_list:     Item::get_helps_for_ids(3, 0, &help_stack, is_admin),
        works_count:    work_stack.len(),
        services_count: service_stack.len(),
        wikis_count:    wiki_stack.len(),
        blogs_count:    blog_stack.len(),
        stores_count:   store_stack.len(),
        helps_count:    help_stack.len(),
    }));
}


#[derive(Serialize)]
pub struct TagBlogsPageResp {
    pub request_user:     UserResp,
    pub tag:              Tag,
    pub blogs_list:       Vec<Blog>,
    pub blogs_count:      usize,
    pub next_page_number: i32
}
pub async fn tag_blogs_page(req: HttpRequest) -> Result<Json<TagBlogsPageResp>, Error> {
    let params_some = web::Query::<TagPageData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.slug.is_none() && params.slug.as_deref().unwrap() == "" {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    let page: i32;
    if params.page.is_some() && params.page.unwrap() > 1 {
        page = params.page.unwrap();
    }
    else {
        page = 1;
    }

    let _connection = establish_connection();
    let _request_user = get_request_user(&req, is_ajax).await;
    let _tag = schema::tags::table
        .filter(schema::tags::name.eq(params.slug.as_deref().unwrap()))
        .first::<Tag>(&_connection)
        .expect("E");
    let _tag_items = schema::tags_items::table
        .filter(schema::tags_items::tag_id.eq(&_tag.id))
        .filter(schema::tags_items::types.eq(1))
        .select(schema::tags_items::item_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_blogs, next_page_number) = Item::get_blogs_list_for_ids(page, 20, &_tag_items, _request_user.perm == 60);
    let blogs_count = _tag_items.len();

    return Ok(Json(TagBlogsPageResp {
        request_user:     _request_user,
        tag:              _tag,
        blogs_list:       _blogs,
        blogs_count:      _tag_items.len(),
        next_page_number: next_page_number,
    }));
}

#[derive(Serialize)]
pub struct TagServicesPageResp {
    pub request_user:     UserResp,
    pub tag:              Tag,
    pub services_list:    Vec<Service>,
    pub services_count:   usize,
    pub next_page_number: i32
}
pub async fn tag_services_page(req: HttpRequest) -> Result<Json<TagServicesPageResp>, Error> {
    let params_some = web::Query::<TagPageData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.slug.is_none() && params.slug.as_deref().unwrap() == "" {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    let page: i32;
    if params.page.is_some() && params.page.unwrap() > 1 {
        page = params.page.unwrap();
    }
    else {
        page = 1;
    }

    let _connection = establish_connection();
    let _request_user = get_request_user(&req, is_ajax).await;
    let _tag = schema::tags::table
        .filter(schema::tags::name.eq(params.slug.as_deref().unwrap()))
        .first::<Tag>(&_connection)
        .expect("E");
    let _tag_items = schema::tags_items::table
        .filter(schema::tags_items::tag_id.eq(&_tag.id))
        .filter(schema::tags_items::types.eq(2))
        .select(schema::tags_items::item_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_services, next_page_number) = Item::get_services_list_for_ids(page, 20, &_tag_items, _request_user.perm == 60);
    let services_count = _tag_items.len();

    return Ok(Json(TagServicesPageResp {
        request_user:     _request_user,
        tag:              _tag,
        services_list:    _services,
        services_count:   _tag_items.len(),
        next_page_number: next_page_number,
    }));
}

#[derive(Serialize)]
pub struct TagStoresPageResp {
    pub request_user:     UserResp,
    pub tag:              Tag,
    pub stores_list:      Vec<Store>,
    pub stores_count:     usize,
    pub next_page_number: i32
}
pub async fn tag_stores_page(req: HttpRequest) -> Result<Json<TagStoresPageResp>, Error> {
    let params_some = web::Query::<TagPageData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.slug.is_none() && params.slug.as_deref().unwrap() == "" {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    let page: i32;
    if params.page.is_some() && params.page.unwrap() > 1 {
        page = params.page.unwrap();
    }
    else {
        page = 1;
    }

    let _connection = establish_connection();
    let _request_user = get_request_user(&req, is_ajax).await;
    let _tag = schema::tags::table
        .filter(schema::tags::name.eq(params.slug.as_deref().unwrap()))
        .first::<Tag>(&_connection)
        .expect("E");
    let _tag_items = schema::tags_items::table
        .filter(schema::tags_items::tag_id.eq(&_tag.id))
        .filter(schema::tags_items::types.eq(3))
        .select(schema::tags_items::item_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_stores, next_page_number) = Item::get_stores_list_for_ids(page, 20, &_tag_items, _request_user.perm == 60);
    let stores_count = _tag_items.len();

    return Ok(Json(TagStoresPageResp {
        request_user:     _request_user,
        tag:              _tag,
        stores_list:      _stores,
        stores_count:     _tag_items.len(),
        next_page_number: next_page_number,
    }));
}

#[derive(Serialize)]
pub struct TagWikisPageResp {
    pub request_user:     UserResp,
    pub tag:              Tag,
    pub wikis_list:       Vec<Wiki>,
    pub wikis_count:      usize,
    pub next_page_number: i32
}
pub async fn tag_wikis_page(req: HttpRequest) -> Result<Json<TagWikisPageResp>, Error> {
    let params_some = web::Query::<TagPageData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.slug.is_none() && params.slug.as_deref().unwrap() == "" {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    let page: i32;
    if params.page.is_some() && params.page.unwrap() > 1 {
        page = params.page.unwrap();
    }
    else {
        page = 1;
    }

    let _connection = establish_connection();
    let _request_user = get_request_user(&req, is_ajax).await;
    let _tag = schema::tags::table
        .filter(schema::tags::name.eq(params.slug.as_deref().unwrap()))
        .first::<Tag>(&_connection)
        .expect("E");
    let _tag_items = schema::tags_items::table
        .filter(schema::tags_items::tag_id.eq(&_tag.id))
        .filter(schema::tags_items::types.eq(4))
        .select(schema::tags_items::item_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_wikis, next_page_number) = Item::get_wikis_list_for_ids(page, 20, &_tag_items, _request_user.perm == 60);
    let wikis_count = _tag_items.len();

    return Ok(Json(TagWikisPageResp {
        request_user:     _request_user,
        tag:              _tag,
        wikis_list:       _wikis,
        wikis_count:      _tag_items.len(),
        next_page_number: next_page_number,
    }));
}

#[derive(Serialize)]
pub struct TagWorksPageResp {
    pub request_user:     UserResp,
    pub tag:              Tag,
    pub works_list:       Vec<Work>,
    pub works_count:      usize,
    pub next_page_number: i32
}
pub async fn tag_works_page(req: HttpRequest) -> Result<Json<TagWorksPageResp>, Error> {
    let params_some = web::Query::<TagPageData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.slug.is_none() && params.slug.as_deref().unwrap() == "" {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    let page: i32;
    if params.page.is_some() && params.page.unwrap() > 1 {
        page = params.page.unwrap();
    }
    else {
        page = 1;
    }

    let _connection = establish_connection();
    let _request_user = get_request_user(&req, is_ajax).await;
    let _tag = schema::tags::table
        .filter(schema::tags::name.eq(params.slug.as_deref().unwrap()))
        .first::<Tag>(&_connection)
        .expect("E");
    let _tag_items = schema::tags_items::table
        .filter(schema::tags_items::tag_id.eq(&_tag.id))
        .filter(schema::tags_items::types.eq(5))
        .select(schema::tags_items::item_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_works, next_page_number) = Item::get_works_list_for_ids(page, 20, &_tag_items, _request_user.perm == 60);
    let works_count = _tag_items.len();

    return Ok(Json(TagWorksPageResp {
        request_user:     _request_user,
        tag:              _tag,
        works_list:       _works,
        works_count:      _tag_items.len(),
        next_page_number: next_page_number,
    }));
}

#[derive(Serialize)]
pub struct TagHelpsPageResp {
    pub request_user:     UserResp,
    pub tag:              Tag,
    pub helps_list:       Vec<Help>,
    pub helps_count:      usize,
    pub next_page_number: i32
}
pub async fn tag_helps_page(req: HttpRequest) -> Result<Json<TagHelpsPageResp>, Error> {
    let params_some = web::Query::<TagPageData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.slug.is_none() && params.slug.as_deref().unwrap() == "" {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    let page: i32;
    if params.page.is_some() && params.page.unwrap() > 1 {
        page = params.page.unwrap();
    }
    else {
        page = 1;
    }

    let _connection = establish_connection();
    let _request_user = get_request_user(&req, is_ajax).await;
    let _tag = schema::tags::table
        .filter(schema::tags::name.eq(params.slug.as_deref().unwrap()))
        .first::<Tag>(&_connection)
        .expect("E");
    let _tag_items = schema::tags_items::table
        .filter(schema::tags_items::tag_id.eq(&_tag.id))
        .filter(schema::tags_items::types.eq(6))
        .select(schema::tags_items::item_id)
        .load::<i32>(&_connection)
        .expect("E");

    let (_helps, next_page_number) = Item::get_helps_list_for_ids(page, 20, &_tag_items, _request_user.perm == 60);
    let helps_count = _tag_items.len();

    return Ok(Json(TagHelpsPageResp {
        request_user:     _request_user,
        tag:              _tag,
        helps_list:       _helps,
        helps_count:      _tag_items.len(),
        next_page_number: next_page_number,
    }));
}


#[derive(Serialize)]
pub struct TagsPageResp {
    pub request_user:     UserResp,
    pub all_tags:         Vec<SmallTag>,
    pub tags_count:       usize,
    pub next_page_number: i32,
    pub stat:             StatPage,
}
pub async fn tags_page(req: HttpRequest) -> Result<Json<TagsPageResp>, Error> {
    let _stat = get_stat_page(1, 0);
    let (is_ajax, page) = get_is_ajax_page(&req);
    let _request_user = get_request_user(&req, is_ajax).await;
    let (all_tags, next_page_number) = Tag::get_tags_list(page, 20);

    return Ok(Json(TagsPageResp {
        request_user:     _request_user,
        all_tags:         all_tags,
        tags_count:       Tag::get_tags_count(),
        next_page_number: next_page_number,
        stat:             _stat,
    }));
}

#[derive(Serialize)]
pub struct EditTagResp {
    pub request_user: UserResp,
    pub tag:          Tag,
}
#[derive(Deserialize)]
pub struct EditTagData {
    pub id:      Option<i32>,
    pub is_ajax: Option<i16>,
}
pub async fn edit_tag_page(req: HttpRequest) -> Result<Json<EditTagResp>, Error> {
    let params_some = web::Query::<EditTagData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.id.is_none() && params.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    
    let _request_user = get_request_user(&req, is_ajax).await;
    let _connection = establish_connection();
    let _tag = schema::tags::table
        .filter(schema::tags::id.eq(&_tag_id))
        .first::<Tag>(&_connection)
        .expect("E");
    if _request_user.perm < 60 && _request_user.id != _tag.user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    return Ok(Json(EditTagResp {
        request_user: _request_user,
        tag:          _tag,
    }));
}


pub async fn edit_tag(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    let _request_user = get_request_user(&req, is_ajax).await;
    if _request_user.id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
    if form.id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _tag = schema::tags::table
        .filter(schema::tags::id.eq(form.id))
        .first::<Tag>(&_connection)
        .expect("E");
    
    if _request_user.id != _tag.user_id && _request_user.perm != 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let _new_tag = EditTag {
        name:     form.name.clone(),
        position: form.position,
    };

    diesel::update(&_tag)
        .set(_new_tag)
        .execute(&_connection)
        .expect("E");

    return Ok(Json(1));
}

#[derive(Deserialize)]
pub struct DeleteItemData {
    pub token: Option<String>,
    pub id:    Option<i32>,
}
pub async fn delete_tag(req: HttpRequest, data: Json<DeleteItemData>) -> Result<Json<i16>, Error> {
    let _request_user = get_request_user(&req, 3).await;
    if _request_user.id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    if data.id.is_none() || data.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    if data.token.is_none() || data.token.as_deref().unwrap() != TOKEN {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }


    let _connection = establish_connection();
    let _tag = schema::tags::table
        .filter(schema::tags::id.eq(data.id.unwrap()))
        .first::<Tag>(&_connection)
        .expect("E");

    if _request_user.id != _tag.user_id && _request_user.perm != 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    use crate::schema::tags_items::dsl::tags_items;

    diesel::delete(
        tags_items.filter(
            schema::tags_items::tag_id.eq(_tag.id))
        )
        .execute(&_connection)
        .expect("E");

    diesel::delete(&_tag)
        .execute(&_connection)
        .expect("E");

    return Ok(Json(1));
}

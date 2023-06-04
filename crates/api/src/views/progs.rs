use actix::Addr;
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::{block, Data, Json},
};
use crate::schema;
use crate::models::{
    NewServeItems,
    NewItem,
    NewCategory,
    Serve,
    NewTechCategoriesItems,
    NewTagItems,
    CookieUser,
    Categories,
    Tag,
    Item,
    CookieStat,
};
use serde::{Deserialize, Serialize};

use crate::utils::{
    establish_connection,
    get_cookie_user_id,
    get_request_user_id,
    get_request_user, get_is_ajax,
    ErrorParams, TOKEN, UserResp,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_multipart::Multipart;
use std::str;
use std::borrow::BorrowMut;
use actix_web::dev::ConnectionInfo;
use crate::errors::Error;


pub fn progs_routes(config: &mut web::ServiceConfig) {
    config.route("/create_history", web::post().to(create_history));
    config.route("/object_history", web::get().to(object_history));
    config.route("/feedback", web::post().to(create_feedback));

    config.route("/create_item", web::post().to(create_item));
    config.route("/edit_item", web::post().to(edit_item));
    config.route("/delete_item", web::post().to(delete_item));
    config.route("/publish_item", web::post().to(publish_item));
    config.route("/hide_item", web::post().to(hide_item));

    config.route("/create_category", web::post().to(create_category));
    config.route("/edit_category", web::post().to(edit_category));
    config.route("/delete_category", web::post().to(delete_category));

    config.route("/create_files", web::post().to(create_files));
    config.route("/edit_file", web::post().to(edit_file));
    config.route("/delete_file", web::post().to(delete_file));
}

pub async fn create_c_user(conn: ConnectionInfo, req: &HttpRequest) -> CookieUser {
    use crate::models::NewCookieUser;
    use chrono::Duration;

    #[derive(Debug, Deserialize)]
    pub struct UserLoc {
        pub city:    CityLoc,
        pub region:  RegionLoc,
        pub country: CountryLoc,
    }
    #[derive(Debug, Deserialize)]
    pub struct CityLoc {
        pub name_ru: String,
        pub name_en: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct RegionLoc {
        pub name_ru: String,
        pub name_en: String,
    }
    #[derive(Debug, Deserialize)]
    pub struct CountryLoc {
        pub name_ru: String,
        pub name_en: String,
    }

    let _connection = establish_connection();
    let mut device: i16 = 1;
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let str_agent = header.1.to_str().unwrap();
            if str_agent.contains("Mobile") {
                device = 2;
            };
            break;
        }
    };

    let mut ipaddr: String = String::new();
    let ip = conn.realip_remote_addr();
    if ip.is_some() {
        ipaddr = ip.unwrap().to_string();
    }
    else if let Some(val) = &req.peer_addr() {
        ipaddr = val.ip().to_string();
    };
    let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_string() + &ipaddr;
    let _geo_request = reqwest::get(_geo_url).await.expect("E.");
    let new_request = _geo_request.text().await.unwrap();
    //println!("request {:?}", new_request);

    let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
    let _user = NewCookieUser {
        ip:         ipaddr,
        device:     device,
        city_ru:    Some(location200.city.name_ru),
        city_en:    Some(location200.city.name_en),
        region_ru:  Some(location200.region.name_ru),
        region_en:  Some(location200.region.name_en),
        country_ru: Some(location200.country.name_ru),
        country_en: Some(location200.country.name_en),
        height:     0.0,
        seconds:    0,
        created:    chrono::Local::now().naive_utc() + Duration::hours(3),
    };
    let _new_user = diesel::insert_into(schema::cookie_users::table)
        .values(&_user)
        .get_result::<CookieUser>(&_connection)
        .expect("Error.");
    return _new_user;
}

pub async fn get_c_user(conn: ConnectionInfo, id: i32, req: &HttpRequest) -> CookieUser {
    if id > 0 {
        use crate::schema::cookie_users::dsl::cookie_users;

        let _connection = establish_connection();
        let _user = cookie_users
            .filter(schema::cookie_users::id.eq(id))
            .first::<CookieUser>(&_connection);

        if _user.is_ok() {
            return _user.expect("E");
        }
        else {
            return create_c_user(conn, &req).await;
        }
    }
    else {
        return create_c_user(conn, &req).await;
    }
}

#[derive(Debug, Deserialize)]
pub struct HistoryData {
    pub token:     String,
    pub user_id:   i32,
    pub object_id: i32,
    pub page_id:   i16,
    pub link:      String,
    pub title:     String,
    pub height:    f64,
    pub seconds:   i32,
    pub template:  String,
}
pub async fn create_history (
    conn: ConnectionInfo,
    data: Json<HistoryData>,
    req: HttpRequest,
) -> Result<Json<CookieStat>, Error> {
    if data.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied.".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    use crate::schema::cookie_stats::dsl::cookie_stats;
    use crate::utils::plus_page_stat;

    let _connection = establish_connection();

    let p_id = data.user_id;
    let user = get_c_user(conn, p_id, &req).await;

    let p_object_id = data.object_id;
    let p_page_id = data.page_id;
    let p_height = data.height;

    let p_seconds = data.seconds;
    let p_link = data.link.clone();
    let p_title = data.title.clone();
    let p_template = data.template.clone();

    let is_cookie_stats_exists = cookie_stats
        .filter(schema::cookie_stats::user_id.eq(p_id))
        .filter(schema::cookie_stats::link.eq(p_link.clone()))
        .select(schema::cookie_stats::id)
        .first::<i32>(&_connection)
        .is_ok();

    if is_cookie_stats_exists {
        diesel::update(&user)
            .set ((
                schema::cookie_users::height.eq(user.height + p_height),
                schema::cookie_users::seconds.eq(user.seconds + p_seconds),
            ))
            .execute(&_connection)
            .expect("Error.");
    }
    if p_object_id > 0 {
        match p_page_id {
            42 | 62 | 72 | 82 | 92 | 102 => {
                use crate::utils::plus_category_stat;
                plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            43 | 63 | 73 | 83 | 93 | 103 => {
                use crate::utils::plus_item_stat;
                plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            32 => {
                use crate::utils::plus_tag_stat;
                plus_tag_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
            },
            _ => println!("no value"),
        };
    }
    else {
        plus_page_stat(p_page_id, p_height, p_seconds, is_cookie_stats_exists)
    }
    let _res = block(move || CookieStat::create (
        user.id,
        p_page_id,
        p_link,
        p_title,
        p_height,
        p_seconds,
        p_template
    )).await?;
    let res = _res?;

    Ok(Json(res))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectResponse {
    pub id:         i32,
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
}
pub async fn object_history(conn: ConnectionInfo, req: HttpRequest, id: web::Path<i32>) -> web::Json<ObjectResponse> {
    let _user = get_c_user(conn, *id, &req).await;
    return web::Json( ObjectResponse {
        id:         _user.id,
        ip:         _user.ip,
        device:     _user.device,
        city_ru:    _user.city_ru,
        city_en:    _user.city_en,
        region_ru:  _user.region_ru,
        region_en:  _user.region_en,
        country_ru: _user.country_ru,
        country_en: _user.country_en,
    })
}

pub async fn create_feedback(conn: ConnectionInfo, mut payload: actix_multipart::Multipart) -> Result<Json<i16>, Error> {
    let form = feedback_form(payload.borrow_mut()).await;
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    use crate::schema::{feedbacks, users};
    use crate::models::NewFeedback;
    use crate::utils::feedback_form;

    let user_id = get_cookie_user_id(&req).await;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let feedbacks_count = feedbacks
        .filter(schema::feedbacks::user_id.eq(user_id))
        .select(schema::feedbacks::id)
        .load::<i32>(&_connection)
        .expect("E")
        .len();
    
    if feedbacks_count > 5 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let _connection = establish_connection();
    let message = form.message.clone();
    let email = form.email.clone();
    if message.len() < 30 || !email.contains("@") {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let new_feedback = NewFeedback {
        user_id:  user_id,
        username: form.username.clone(),
        email:    email,
        message:  message,
    };
    let _new_feedback = diesel::insert_into(feedbacks::table)
        .values(&new_feedback)
        .execute(&_connection)
        .expect("E.");
    
    return Ok(Json(1));
}


pub async fn create_item(mut payload: Multipart) -> Result<Json<i16>, Error> {
    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let form = crate::utils::item_form(payload.borrow_mut(), user_id).await;
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();

    let types = form.types;
    let new_item = NewItem::create (
        form.title.clone(),
        form.link.clone(),
        form.main_image.clone(),
        user_id,
        form.position,
        types,
        form.slug.clone(),
    );

    let _item = diesel::insert_into(schema::items::table)
        .values(&new_item)
        .get_result::<Item>(&_connection)
        .expect("E.");

    for category_id in form.category_list.iter() {
        let new_category = NewCategory {
            categories_id: *category_id,
            item_id:       _item.id,
            types:         types,
        };
        diesel::insert_into(schema::category::table)
            .values(&new_category)
            .execute(&_connection)
            .expect("E.");
    };
    for tag_id in form.tags_list.iter() {
        let new_tag = NewTagItems {
            tag_id: *tag_id,
            item_id: _item.id,
            types:   types,
            created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(schema::tags_items::table)
            .values(&new_tag)
            .execute(&_connection)
            .expect("Error.");
    }

    // создаем связь с тех категориями, которые будут
    // расширять списки опций, предлагая доп возможности и услуги
    for cat_id in form.close_tech_cats_list.iter() {
        let new_cat = NewTechCategoriesItem {
            category_id: *cat_id,
            item_id:     _item.id,
            types:       types,
            is_active:   2,
        };
        diesel::insert_into(schema::tech_categories_items::table)
            .values(&new_cat)
            .execute(&_connection)
            .expect("Error.");
    }

    // создаем опции услуги и записываем id опций в вектор.
    let mut serve_ids = Vec::new();
    for serve_id in form.serve_list.iter() {
        let new_serve_form = NewServeItems {
            serve_id: *serve_id,
            item_id:  _item.id,
            types:    types,
        };
        diesel::insert_into(schema::serve_items::table)
            .values(&new_serve_form)
            .execute(&_connection)
            .expect("Error.");
        serve_ids.push(*serve_id);
    }

    // получаем опции, чтобы создать связи с их тех. категорией.
    // это надо отрисовки тех категорий услуги, которые активны
    let _serves = schema::serve::table
        .filter(schema::serve::id.eq_any(serve_ids))
        .load::<Serve>(&_connection)
        .expect("E");

    let mut tech_cat_ids = Vec::new();
    let mut item_price = 0;
    for _serve in _serves.iter() {
        if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
            tech_cat_ids.push(_serve.tech_cat_id);
        }
        item_price += _serve.price;
    }

    for id in tech_cat_ids.iter() {
        let new_cat = NewTechCategoriesItem {
            category_id: *id,
            item_id:     _item.id,
            types:       types,
            is_active:   1,
        };
        diesel::insert_into(schema::tech_categories_items::table)
            .values(&new_cat)
            .execute(&_connection)
            .expect("Error.");
    }

    // фух. Связи созданы все, но надо еще посчитать цену
    // услуги для калькулятора. Как? А  это будет сумма всех
    // цен выбранных опций.
    let price_acc = crate::utils::get_price_acc_values(&item_price);
    diesel::update(&_item)
        .set((
            schema::items::price.eq(item_price),
            schema::items::price_acc.eq(price_acc),
        ))
        .execute(&_connection)
        .expect("Error.");

    return Ok(Json(1));
}

pub async fn edit_item(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    use crate::utils::{
        item_form,
        get_price_acc_values,
    };

    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let form = item_form(payload.borrow_mut(), user_id).await;
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    
    let _connection = establish_connection();
    let _item = items
        .filter(schema::items::id.eq(form.id))
        .first::<Item>(&_connection)
        .expect("E");
    let _item_id = _item.id;
    if user_id != _item.user_id && _item.item_types > 9 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    use crate::schema::{
        tags::dsl::tags,
        items::dsl::items,
        serve_items::dsl::serve_items,
        tags_items::dsl::tags_items,
        categories::dsl::categories,
        category::dsl::category,
        tech_categories_items::dsl::tech_categories_items,
        serve::dsl::serve,
    };

    use crate::models::{
        NewTechCategoriesItem,
        Serve,
        NewServeItems,
        NewCategory,
        NewTagItems,
        EditItem,
    };

    let _categories = _item.get_categories_obj().expect("E");
    let _tags = _item.get_tags_obj().expect("E");

    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::categories::count.eq(_category.count - 1))
            .execute(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set(schema::tags::count.eq(_tag.count - 1))
            .execute(&_connection)
            .expect("Error.");
    };

    diesel::delete (
        tags_items
            .filter(schema::tags_items::item_id.eq(_item_id))
            .filter(schema::tags_items::types.eq(_item.types))
        )
        .execute(&_connection)
        .expect("E");
    diesel::delete (
        serve_items
            .filter(schema::serve_items::item_id.eq(_item_id))
            .filter(schema::serve_items::types.eq(_item.types))
        )
        .execute(&_connection)
        .expect("E");
    diesel::delete (
        tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(_item_id))
            .filter(schema::tech_categories_items::types.eq(_item.types))
        )
        .execute(&_connection)
        .expect("E");
    diesel::delete (
        category
            .filter(schema::category::item_id.eq(_item_id))
            .filter(schema::category::types.eq(_item.types))
        )
        .execute(&_connection)
        .expect("E");

    let _new_item = EditItem {
        title:       form.title.clone(),
        link:        form.link.clone(),
        image:       form.main_image.clone(),
        position:    form.position,
        slug:        form.slug.clone(),
    };

    diesel::update(&_item)
        .set(_new_item)
        .execute(&_connection)
        .expect("E");

    for category_id in form.category_list.iter() {
        let new_category = NewCategory {
            categories_id: *category_id,
            item_id:       _item.id,
            types:         _item.types,
        };
        diesel::insert_into(schema::category::table)
            .values(&new_category)
            .execute(&_connection)
            .expect("E.");

        let _category = categories
            .filter(schema::categories::id.eq(category_id))
            .filter(schema::categories::types.eq(_item.types))
            .first::<Categories>(&_connection)
            .expect("E");
        diesel::update(&_category)
            .set(schema::categories::count.eq(_category.count + 1))
            .execute(&_connection)
            .expect("Error.");
    }

    for tag_id in form.tags_list.iter() {
        let new_tag = NewTagItems {
            tag_id: *tag_id,
            item_id: _item.id,
            types:   _item.types,
            created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(schema::tags_items::table)
            .values(&new_tag)
            .execute(&_connection)
            .expect("Error.");

        if _item.item_types < 10 {
            let _tag = tags
                .filter(schema::tags::id.eq(tag_id))
                .first::<Tag>(&_connection)
                .expect("E");

            diesel::update(&_tag)
                .set(schema::tags::count.eq(_tag.count + 1))
                .execute(&_connection)
                .expect("Error.");
        }
    }

    // создаем связь с тех категориями, которые будут
    // расширять списки опций, предлагая доп возможности и услуги
    for cat_id in form.close_tech_cats_list.iter() {
        let new_cat = NewTechCategoriesItem {
            category_id: *cat_id,
            item_id:     _item.id,
            types:       _item.types,
            is_active:   2,
        };
        diesel::insert_into(schema::tech_categories_items::table)
            .values(&new_cat)
            .execute(&_connection)
            .expect("Error.");
    }

    // создаем опции услуги и записываем id опций в вектор.
    let mut serve_ids = Vec::new();
    for serve_id in form.serve_list.iter() {
        let new_serve_form = NewServeItems {
            serve_id: *serve_id,
            item_id:  _item.id,
            types:    _item.types,
        };
        diesel::insert_into(schema::serve_items::table)
            .values(&new_serve_form)
            .execute(&_connection)
            .expect("Error.");
        serve_ids.push(*serve_id);
    }

    // получаем опции, чтобы создать связи с их тех. категорией.
    // это надо отрисовки тех категорий услуги, которые активны
    let _serves = serve
        .filter(schema::serve::id.eq_any(serve_ids))
        .load::<Serve>(&_connection)
        .expect("E");

    let mut tech_cat_ids = Vec::new();
    let mut item_price = 0;
    for _serve in _serves.iter() {
        if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
            tech_cat_ids.push(_serve.tech_cat_id);
        }
        item_price += _serve.price;
    }

    for id in tech_cat_ids.iter() {
        let new_cat = NewTechCategoriesItem {
            category_id: *id,
            item_id:     _item.id,
            types:       _item.types,
            is_active:   1,
        };
        diesel::insert_into(schema::tech_categories_items::table)
            .values(&new_cat)
            .execute(&_connection)
            .expect("Error.");
    }

    // фух. Связи созданы все, но надо еще посчитать цену
    // услуги для калькулятора. Как? А  это будет сумма всех
    // цен выбранных опций.
    let price_acc = get_price_acc_values(&item_price);
    diesel::update(&_item)
        .set((
            schema::items::price.eq(item_price),
            schema::items::price_acc.eq(price_acc),
        ))
        .execute(&_connection)
        .expect("Error.");

    return Ok(Json(1));
}


pub async fn create_category(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    use crate::utils::category_form;

    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let form = category_form(payload.borrow_mut(), user_id).await;
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let _connection = establish_connection();

    let new_cat = crate::models::NewCategories {
        name:        form.name.clone(),
        user_id:     user_id,
        description: Some(form.description.clone()),
        position:    form.position,
        image:       Some(form.image.clone()),
        count:       0,
        view:        0,
        height:      0.0,
        seconds:     0,
        types:       form.types,
        slug:        form.slug,
    };
    diesel::insert_into(schema::categories::table)
        .values(&new_cat)
        .execute(&_connection)
        .expect("E.");

    return Ok(Json(1));
}

pub async fn edit_category(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    use crate::utils::category_form;

    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let form = category_form(payload.borrow_mut(), user_id).await;
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    if form.id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _category = schema::categories::table
        .filter(schema::categories::id.eq(form.id))
        .first::<Categories>(&_connection)
        .expect("E");
    
    if user_id == _category.user_id {
        let _new_cat = crate::models::EditCategories {
            name:        form.name.clone(),
            description: Some(form.description.clone()),
            position:    form.position,
            image:       Some(form.image.clone()),
            slug:        form.slug,
        };
        diesel::update(&_category)
            .set(_new_cat)
            .execute(&_connection)
            .expect("E");
    }

    return Ok(Json(1));
}

#[derive(Deserialize)]
pub struct DeleteItemData {
    pub id:    Option<i32>,
    pub token: Option<String>,
}
pub async fn delete_item(req: HttpRequest, data: Json<DeleteItemData>) -> Result<Json<i16>, Error> {
    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    if data.token.is_none() || data.token.as_deref().unwrap() != TOKEN {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    use crate::schema::{
        items::dsl::items,
        tags_items::dsl::tags_items,
        category::dsl::category,
        files::dsl::files,
    };

    if data.id.is_none() || data.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let id = data.id.unwrap();
    let _item = items
        .filter(schema::items::id.eq(id))
        .first::<Item>(&_connection)
        .expect("E");
    
    if _item.user_id != user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _src_list = files
        .filter(schema::files::item_id.eq(id))
        .filter(schema::files::item_types.eq(_item.types))
        .select(schema::files::src)
        .load::<String>(&_connection)
        .expect("E");

    for f in _src_list.iter() {
        std::fs::remove_file(f).expect("E");
    }

    diesel::delete (
        files
            .filter(schema::files::item_id.eq(id))
            .filter(schema::files::item_types.eq(_item.types))
        )
        .execute(&_connection)
        .expect("E");
    diesel::delete (
        tags_items
            .filter(schema::tags_items::item_id.eq(id))
            .filter(schema::tags_items::types.eq(_item.types))
        )
        .execute(&_connection)
        .expect("E");
    diesel::delete (
        category
            .filter(schema::category::item_id.eq(id))
            .filter(schema::category::types.eq(_item.types))
        )
        .execute(&_connection)
        .expect("E");
    diesel::delete(&_item).execute(&_connection).expect("E");

    let _categories = _item.get_categories_obj().expect("E");
    let _tags = _item.get_tags_obj().expect("E");

    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::categories::count.eq(_category.count - 1))
            .execute(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set(schema::tags::count.eq(_tag.count - 1))
            .execute(&_connection)
            .expect("Error.");
    };
    
    return Ok(Json(1));
}


pub async fn delete_category(req: HttpRequest, data: Json<DeleteItemData>) -> Result<Json<i16>, Error> {
    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    if data.token.is_none() || data.token.as_deref().unwrap() != TOKEN {
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

    let id = data.id.unwrap();
    let _connection = establish_connection();
    let _item = schema::categories::table
        .filter(schema::categories::id.eq(id))
        .first::<Categories>(&_connection)
        .expect("E");
    
    if _item.user_id != user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    diesel::delete(&_item)
        .execute(&_connection)
        .expect("E");
    
    return Ok(Json(1));
}

pub async fn create_files(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let form = crate::utils::files_form(payload.borrow_mut(), user_id).await;
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
    
    use crate::schema::items::dsl::items;
    use crate::models::NewFile;

            
    let types = form.types;
    let item_types = form.item_types;
            
    let _item = items
        .filter(schema::items::id.eq(form.id))
        .filter(schema::items::types.eq(item_types))
        .first::<Item>(&_connection)
        .expect("E");
    
    if user_id != _item.user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    
    for file in form.files.iter() {
        let new_file = NewFile::create (
            user_id,
            _item.id,
            item_types,
            types,
            file.to_string()
        );
        diesel::insert_into(schema::files::table)
            .values(&new_file)
            .execute(&_connection)
            .expect("E.");
    };
    return Ok(Json(1));
}

pub async fn edit_file(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }  

    let form = crate::utils::category_form(payload.borrow_mut(), user_id).await;
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

    use crate::models::{EditFile, File};

    let _connection = establish_connection();
    let _file = schema::files::table
        .filter(schema::files::id.eq(form.id))
        .first::<File>(&_connection)
        .expect("E");

    if user_id != _file.user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let _new_file = EditFile {
        description: Some(form.description.clone()),
        position:    form.position,
    };

    diesel::update(&_file)
        .set(_new_file)
        .execute(&_connection)
        .expect("E");
    
    return Ok(Json(1));
}

pub async fn delete_file(req: HttpRequest, data: Json<DeleteItemData>) -> Result<Json<i16>, Error> {
    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
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

    let id = data.id.unwrap();
    let _connection = establish_connection();
    
    use crate::schema::files::dsl::files;
    use crate::models::File;

    let _file = files
        .filter(schema::files::id.eq(id))
        .first::<File>(&_connection)
        .expect("E");
    
    if user_id != _file.user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    std::fs::remove_file(_file.src).expect("E");

    diesel::delete(files.filter(schema::files::id.eq(id)))
        .execute(&_connection)
        .expect("E");

    return Ok(Json(1));
}

pub async fn publish_item(req: HttpRequest, data: Json<DeleteItemData>) -> Result<Json<i16>, Error> {
    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
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

    let id = data.id.unwrap();
    let _connection = establish_connection();
    let _item = schema::items::table
        .filter(schema::items::id.eq(id))
        .first::<Item>(&_connection)
        .expect("E");

    if user_id != _item.user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    
    _item.make_publish();

    let _categories: Vec<Categories>;
    let _tags: Vec<Tag>;

    let tags_o = _item.get_tags_obj().expect("E");
    let categories_o = _item.get_categories_obj().expect("E");
    let cats_res = block(move || categories_o).await;
    let tags_res = block(move || tags_o).await;
    _categories = match cats_res {
        Ok(_ok) => _ok,
        Err(_error) => Vec::new(),
    };
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::categories::count.eq(_category.count + 1))
            .execute(&_connection)
            .expect("Error.");
    }
    _tags = match tags_res {
        Ok(_list) => _list,
        Err(_error) => Vec::new(),
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set(schema::tags::count.eq(_tag.count + 1))
            .execute(&_connection)
            .expect("Error.");
    }

    return Ok(Json(1));
}

pub async fn hide_item(req: HttpRequest, data: Json<DeleteItemData>) -> Result<Json<i16>, Error> {
    let user_id = get_request_user_id(&req).await;
    if user_id < 1 {
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

    let id = data.id.unwrap();
    let _connection = establish_connection();
    let _item = schema::items::table
        .filter(schema::items::id.eq(id))
        .first::<Item>(&_connection)
        .expect("E");

    if user_id != _item.user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    
    _item.make_hide();

    let _categories: Vec<Categories>;
    let _tags: Vec<Tag>;

    let tags_o = _item.get_tags_obj().expect("E");
    let categories_o = _item.get_categories_obj().expect("E");
    let cats_res = block(move || categories_o).await;
    let tags_res = block(move || tags_o).await;
    _categories = match cats_res {
        Ok(_ok) => _ok,
        Err(_error) => Vec::new(),
    };
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::categories::count.eq(_category.count + 1))
            .execute(&_connection)
            .expect("Error.");
    }
    _tags = match tags_res {
        Ok(_list) => _list,
        Err(_error) => Vec::new(),
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set(schema::tags::count.eq(_tag.count + 1))
            .execute(&_connection)
            .expect("Error.");
    }

    return Ok(Json(1));
}
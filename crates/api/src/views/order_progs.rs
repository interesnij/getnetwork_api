use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::{block, Data, Json},
};
use std::borrow::BorrowMut;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::utils::{
    establish_connection,
    get_request_user,
    get_or_create_cookie_user_id,
    get_cookie_user_id,
    ErrorParams, TOKEN, UserResp, 
    ObjectPageResp, InfoParams,
};
use crate::schema;
use crate::models::{
    Order,
    NewOrder,
    OrderFile,
    NewOrderFile,
};
use serde::{Deserialize, Serialize};
use actix_multipart::Multipart;
use crate::models::User;
use actix_web::dev::ConnectionInfo;


pub fn order_routes(config: &mut web::ServiceConfig) {
    config.route("/orders", web::get().to(get_orders_page));
    config.route("/user_orders", web::get().to(get_user_orders_page));
    config.route("/order", web::get().to(get_order_page));
    config.route("/order", web::post().to(create_order));
    config.route("/delete_order", web::post().to(delete_order));
}

#[derive(Serialize)]
pub struct OrdersPageResp {
    pub request_user:     UserResp,
    pub object_list:      Vec<Order>,
    pub next_page_number: i32,
}
pub async fn get_orders_page(req: HttpRequest) -> Result<Json<OrdersPageResp>, Error> {
    use crate::utils::get_is_ajax_page;

    let (is_ajax, page) = get_is_ajax_page(&req);
    let _request_user = get_request_user(&req, is_ajax);
    if _request_user.id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let (_orders, next_page_number) = Order::get_orders_list(page, 20);

    return Ok(Json(OrdersPageResp {
        request_user:     _request_user,
        object_list:      _orders,
        next_page_number: next_page_number,
    }));
}

pub async fn get_user_orders_page(req: HttpRequest) -> Result<Json<OrdersPageResp>, Error> {
    use crate::utils::get_is_ajax_page;

    let (is_ajax, page) = get_is_ajax_page(&req);
    let _request_user = get_request_user(&req, is_ajax);
    let user_id = get_cookie_user_id(&req).await;
    if user_id == 0 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Информация о заказчике не найдена".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let (_orders, next_page_number) = Order::get_user_orders_list(user_id, page, 20);

    return Ok(Json(OrdersPageResp {
        request_user:     _request_user,
        object_list:      _orders,
        next_page_number: next_page_number,
    }));
}

#[derive(Serialize)]
pub struct OrderPageResp {
    pub request_user: UserResp,
    pub object:       Order,
    pub files:        Vec<OrderFile>,
}
#[derive(Deserialize)]
pub struct OrderPageData {
    pub is_ajax: Option<i16>,
    pub id:      Option<i32>,
}
pub async fn get_order_page(req: HttpRequest) -> Result<Json<OrderPageResp>, Error> {
    use schema::orders::dsl::orders;

    let params_some = web::Query::<OrderPageData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Информация о заказчике не найдена".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.id.is_none() || params.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let user_id = get_cookie_user_id(&req).await;
    if user_id != _order.user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Информация о заказчике не найдена".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _order = orders
        .filter(schema::orders::id.eq(params.id.unwrap()))
        .first::<Order>(&_connection)
        .expect("E");
    let _files = order_files
        .filter(schema::order_files::order_id.eq(&_order.id))
        .load::<OrderFile>(&_connection)
        .expect("E");
    
    return Ok(Json(OrderPageResp {
        request_user: _request_user,
        object:       _order,
        files:        _files,
    }));
}


#[derive(Serialize)]
pub struct NewOrderResp {
    pub order_id: i32,
}
pub async fn create_order(req: HttpRequest, mut payload: Multipart) -> Result<Json<NewOrderResp>, Error> {
    use crate::schema::serve::dsl::serve;
    use crate::models::{
        NewTechCategoriesItem,
        Serve,
        NewServeItems,
    };
    use crate::utils::{
        order_form,
        get_price_acc_values,
    };

    let _connection = establish_connection();
    let user_id = get_or_create_cookie_user_id(_connection, &req).await;
    let form = order_form(payload.borrow_mut(), user_id).await;
    
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if user_id == 0 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Информация о заказчике не найдена".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else {
        let new_order = NewOrder::create (
            form.title.clone(),
            form.types,
            form.object_id,
            form.username.clone(),
            form.email.clone(),
            form.description.clone(),
            user_id,
        );

        let _order = diesel::insert_into(schema::orders::table)
            .values(&new_order)
            .get_result::<Order>(&_connection)
            .expect("E.");

        for file in form.files.iter() {
            let new_file = NewOrderFile::create (
                _order.id,
                file.to_string()
            );
            diesel::insert_into(schema::order_files::table)
                .values(&new_file)
                .execute(&_connection)
                .expect("E.");
        };

        // создаем опции услуги и записываем id опций в вектор.
        let mut serve_ids = Vec::new();
        for serve_id in form.serve_list.iter() {
            let new_serve_form = NewServeItems {
                serve_id: *serve_id,
                item_id:  form.object_id,
                types:    form.types,
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
        let mut order_price = 0;
        for _serve in _serves.iter() {
            if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
                tech_cat_ids.push(_serve.tech_cat_id);
            }
            order_price += _serve.price;
        }

        for id in tech_cat_ids.iter() {
            let new_cat = NewTechCategoriesItem {
                category_id: *id,
                item_id:     form.object_id,
                types:       form.types,
                is_active:   1,
            };
            diesel::insert_into(schema::tech_categories_items::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("Error.");
        }

        // фух. Связи созданы все, но надо еще посчитать цену
        // услуги для калькулятора. Как? А это будет сумма всех
        // цен выбранных опций.
        let price_acc = get_price_acc_values(&order_price);
        diesel::update(&_order)
            .set((
                schema::orders::price.eq(order_price),
                schema::orders::price_acc.eq(price_acc),
            ))
            .get_result::<Order>(&_connection)
            .expect("Error.");

        return Ok(Json(NewOrderResp {
            order_id: _order.id,
        }));
    }
}

#[derive(Deserialize)]
pub struct DeleteOrderData {
    pub token: Option<String>,
    pub id:    Option<i32>,
}
pub async fn delete_order(req: HttpRequest, data: Json<DeleteOrderData>) -> Result<Json<i16>, Error> {
    use schema::orders::dsl::orders;

    if data.token.is_none() || data.token.as_deref().unwrap() != TOKEN {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if data.id.is_none() || data.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let _connection = establish_connection();
    let _order = orders
        .filter(schema::orders::id.eq(data.id.unwrap()))
        .first::<Order>(&_connection)
        .expect("E");

    let user_id = get_cookie_user_id(&req).await;

    if user_id == _order.user_id {
        use crate::schema::{
            serve_items::dsl::serve_items,
            tech_categories_items::dsl::tech_categories_items,
        };

        diesel::delete (
            serve_items
                .filter(schema::serve_items::item_id.eq(_order.id))
                .filter(schema::serve_items::types.eq(7))
            )
            .execute(&_connection)
            .expect("E");
        diesel::delete(
            tech_categories_items
                .filter(schema::tech_categories_items::item_id.eq(_order.id))
                .filter(schema::tech_categories_items::types.eq(7))
            )
            .execute(&_connection)
            .expect("E");
        diesel::delete(&_order).execute(&_connection).expect("E");

        return Json(1);
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
}

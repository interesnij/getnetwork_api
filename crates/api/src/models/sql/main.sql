
-- feedback -------
---------------
---------------
CREATE TABLE feedbacks (
    id       SERIAL PRIMARY KEY,
    user_id  INT NOT NULL,
    username VARCHAR(100) NOT NULL,
    email    VARCHAR(200) NOT NULL,
    message  VARCHAR(1000) NOT NULL
);

-- orders -------
---------------
---------------
CREATE TABLE orders (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    types       SMALLINT NOT NULL, -- 1 услуга, 2 товар, 3 работа
    object_id   INT NOT NULL,
    username    VARCHAR(200) NOT NULL,
    email       VARCHAR(200) NOT NULL,
    description VARCHAR(1000),
    created     TIMESTAMP NOT NULL,
    user_id     INT NOT NULL,
    price       INT NOT NULL,
    price_acc   INT
);

CREATE TABLE order_files (
    id       SERIAL PRIMARY KEY,
    order_id INT NOT NULL,
    src      VARCHAR(100) NOT NULL,

    CONSTRAINT fk_order_files
        FOREIGN KEY(order_id)
            REFERENCES orders(id)
);
CREATE INDEX order_files_id_idx ON order_files (order_id);


-- users -------
---------------
---------------
CREATE TABLE users (
    id         SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name  VARCHAR(100) NOT NULL,
    username   VARCHAR(100) NOT NULL,
    email      VARCHAR(100) NOT NULL,
    password   VARCHAR(1000) NOT NULL,
    bio        VARCHAR(500),
    image      VARCHAR(100),
    perm       SMALLINT NOT NULL,
    view       INT NOT NULL,
    height     FLOAT NOT NULL,
    seconds    INT NOT NULL,

    UNIQUE(username),
    UNIQUE(email)
); 

INSERT INTO users (id, first_name, last_name, username, email, password, bio, image, perm, view, height, seconds)
VALUES (1, 'Сергей', 'Зубарев', 'Serg', 'ochkarik1983@mail.ru', 'Ulihos46!', NULL, NULL, 60, 0, 0.0, 0) ON CONFLICT DO NOTHING;

-- chat -------
---------------
---------------

CREATE TABLE chats (
    id                SERIAL PRIMARY KEY,
    user_id           INT NOT NULL,
    created           TIMESTAMP NOT NULL,

    CONSTRAINT fk_chat_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX chats_user_id_idx ON chats (user_id);

CREATE TABLE messages (
    id      SERIAL PRIMARY KEY,     -- id объекта
    user_id INT NOT NULL,           -- id создателя
    chat_id INT NOT NULL,           -- id чата
    created TIMESTAMP NOT NULL,     -- когда создано
    content VARCHAR(5000),          -- текст
    view    SMALLINT NOT NULL,      -- создано / показано / прочитано
    types   SMALLINT NOT NULL,      -- обычное / изменено / удалено

    CONSTRAINT fk_message_creator        -- связь с создателем
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX messages_user_id_idx ON messages (user_id);



CREATE TABLE cookie_users (
    id         SERIAL PRIMARY KEY,
    ip         VARCHAR(100) NOT NULL, -- ip адрес пользователя
    device     SMALLINT NOT NULL,     -- комп - смартфон - планшет
    city_ru    VARCHAR(150),          -- город по русски
    city_en    VARCHAR(150),          -- город по английски
    region_ru  VARCHAR(150),          -- регион по русски
    region_en  VARCHAR(150),          -- регион по английски
    country_ru VARCHAR(150),          -- страна по русски
    country_en VARCHAR(150),          -- страна по английски
    height     FLOAT NOT NULL,
    seconds    INT NOT NULL,
    created    TIMESTAMP NOT NULL     -- когда создан пользователь
);
CREATE TABLE cookie_stats (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,          -- связь с пользователем куки
    page       SMALLINT NOT NULL,     -- номер страницы, которая просматривается
    link       VARCHAR(200) NOT NULL, -- ссылка страницы
    title      VARCHAR(200) NOT NULL, -- название страницы
    height     FLOAT NOT NULL,        -- высота просмотра страницы
    seconds    INT NOT NULL,          -- секунды нахождения страницы
    created    TIMESTAMP NOT NULL,    -- когда создана запись
    template   VARCHAR(100) NOT NULL DEFAULT "rhythm", -- вид шаблона

    CONSTRAINT fk_cookie_stat_user
        FOREIGN KEY(user_id)
            REFERENCES cookie_users(id)
);

-- tags -------
---------------
---------------
CREATE TABLE tags (
    id        SERIAL PRIMARY KEY,
    name      VARCHAR(100) NOT NULL,
    position  SMALLINT NOT NULL,
    count     SMALLINT NOT NULL,
    user_id   INT NOT NULL,
    view      INT NOT NULL,
    height    FLOAT NOT NULL,
    seconds   INT NOT NULL,

    CONSTRAINT fk_tag_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);


CREATE TABLE tags_items (
    id      SERIAL PRIMARY KEY,
    tag_id  INT NOT NULL,
    item_id INT NOT NULL,
    types   SMALLINT NOT NULL, -- блог, услуга, товар ......
    created TIMESTAMP NOT NULL
);


-- categories -------
---------------
---------------

CREATE TABLE categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    user_id     INT NOT NULL,
    description VARCHAR(500),
    position    SMALLINT NOT NULL,
    image       VARCHAR(100),
    count       SMALLINT NOT NULL,
    view        INT NOT NULL,
    height      FLOAT NOT NULL,
    seconds     INT NOT NULL,
    types       SMALLINT NOT NULL, -- категория блога, категория услуги ......
    slug        VARCHAR(100) NOT NULL,

    UNIQUE(slug)
);

INSERT INTO categories (id, name, user_id, description, position, image, count, view, height, seconds, types, slug)
VALUES (1, 'Все услуги', 1, NULL, 1, NULL, 0, 0, 0.0, 0, 2, 'all-services') ON CONFLICT DO NOTHING;
INSERT INTO categories (id, name, user_id, description, position, image, count, view, height, seconds, types, slug)
VALUES (2, 'Веб-разработка', 1, NULL, 2, NULL, 0, 0, 0.0, 0, 2, 'service-web') ON CONFLICT DO NOTHING;

INSERT INTO categories (id, name, user_id, description, position, image, count, view, height, seconds, types, slug)
VALUES (3, 'Все работы', 1, NULL, 1, NULL, 0, 0, 0.0, 0, 5, 'all-works') ON CONFLICT DO NOTHING;
INSERT INTO categories (id, name, user_id, description, position, image, count, view, height, seconds, types, slug)
VALUES (4, 'Веб-разработка', 1, NULL, 2, NULL, 0, 0, 0.0, 0, 5, 'web-development') ON CONFLICT DO NOTHING;

INSERT INTO categories (id, name, user_id, description, position, image, count, view, height, seconds, types, slug)
VALUES (5, 'Все товары', 1, NULL, 1, NULL, 0, 0, 0.0, 0, 3, 'all-stores') ON CONFLICT DO NOTHING;
INSERT INTO categories (id, name, user_id, description, position, image, count, view, height, seconds, types, slug)
VALUES (6, 'Все статьи', 1, NULL, 1, NULL, 0, 0, 0.0, 0, 4, 'all-wikis') ON CONFLICT DO NOTHING;
INSERT INTO categories (id, name, user_id, description, position, image, count, view, height, seconds, types, slug)
VALUES (7, 'Все статьи', 1, NULL, 1, NULL, 0, 0, 0.0, 0, 1, 'all-articles') ON CONFLICT DO NOTHING;

INSERT INTO categories (id, name, user_id, description, position, image, count, view, height, seconds, types, slug)
VALUES (8, 'Как заказать проект', 1, NULL, 1, NULL, 0, 0, 0.0, 0, 6, 'how-to-order-a-project') ON CONFLICT DO NOTHING;
INSERT INTO categories (id, name, user_id, description, position, image, count, view, height, seconds, types, slug)
VALUES (9, 'Что мы о Вас знаем', 2, NULL, 1, NULL, 0, 0, 0.0, 0, 6, 'what-do-we-know-abou-you') ON CONFLICT DO NOTHING;
INSERT INTO categories (id, name, user_id, description, position, image, count, view, height, seconds, types, slug)
VALUES (10, 'Этапы создания проектов', 3, NULL, 1, NULL, 0, 0, 0.0, 0, 6, 'stages-of-project-creation') ON CONFLICT DO NOTHING;

-- 1 обычное, 2 изменено, 3 черновик, 
-- 11 удалено обычное, 12 удалено обычное, 13 удален черновик
CREATE TABLE items ( 
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(100) NOT NULL,
    link        VARCHAR(500),
    image       VARCHAR(100),
    description VARCHAR(1000),
    item_types  SMALLINT NOT NULL,          -- обычное / изменено / удалено
    price       INT NOT NULL,
    user_id     INT NOT NULL,
    created     TIMESTAMP NOT NULL,
    position    SMALLINT NOT NULL,
    price_acc   INT,
    types       SMALLINT NOT NULL,           -- блог, услуга, товар ......
    slug        VARCHAR(100) NOT NULL,
    view        INT NOT NULL,
    height      FLOAT NOT NULL,
    seconds     INT NOT NULL,

    UNIQUE(slug),

    CONSTRAINT fk_store_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX items_creator_idx ON items (user_id);
 
CREATE TABLE item_contents ( 
    id         SERIAL PRIMARY KEY,
    title      VARCHAR(100) NOT NULL,  -- название части контента
    item_title VARCHAR(100) NOT NULL,  -- название объекта родителя
    content    VARCHAR(5000) NOT NULL, -- контент
    types      SMALLINT NOT NULL,      -- обычное / изменено / удалено
    item_id    INT NOT NULL,           -- id объекта родителя
    item_types SMALLINT NOT NULL,      -- тип объекта родителя
    user_id    INT NOT NULL,           -- id создателя
    position   SMALLINT NOT NULL,      -- позиция части контента
    created    TIMESTAMP NOT NULL,     -- когда часть создана

    CONSTRAINT fk_item_contents
        FOREIGN KEY(item_id)
            REFERENCES items(id),

    CONSTRAINT fk_user_item_contents
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX item_contents_id_idx ON item_contents (item_id);
CREATE INDEX item_contents_user_id_idx ON item_contents (user_id);


CREATE TABLE item_comments (
    id        SERIAL PRIMARY KEY,
    comment   VARCHAR(1000) NOT NULL,
    item_id   INT NOT NULL,
    user_id   INT NOT NULL,
    parent_id INT,
    created   TIMESTAMP NOT NULL,

    CONSTRAINT fk_item_comment
        FOREIGN KEY(item_id)
            REFERENCES items(id),

    CONSTRAINT fk_user_item_comment
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_item_parent_comment
        FOREIGN KEY(parent_id)
            REFERENCES item_comments(id)
);
CREATE INDEX item_comments_id_idx ON item_comments (item_id);
CREATE INDEX item_comments_user_id_idx ON item_comments (user_id);

CREATE TABLE category (
    id            SERIAL PRIMARY KEY,
    categories_id INT NOT NULL,
    item_id       INT NOT NULL,
    types         SMALLINT NOT NULL, -- блог, услуга, товар ......

   CONSTRAINT fk_category_cat
        FOREIGN KEY(categories_id)
            REFERENCES categories(id),

   CONSTRAINT fk_category_item
        FOREIGN KEY(item_id)
            REFERENCES items(id)
);

CREATE TABLE files (
    id          SERIAL PRIMARY KEY,
    user_id     INT NOT NULL,
    item_id     INT NOT NULL,
    item_types  SMALLINT NOT NULL,      -- блог, услуга, товар ......
    types       SMALLINT NOT NULL,      -- фото, видео, документ  ......
    src         VARCHAR(100) NOT NULL,
    description VARCHAR,
    position    SMALLINT NOT NULL,
    view        INT NOT NULL,
    seconds     INT NOT NULL,

    UNIQUE(src)
);

-- serve -------
---------------
---------------
-- это технические категории опций (например, большой магазин или моб приложение ресторана)
CREATE TABLE tech_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(10000),
    position    SMALLINT NOT NULL,
    count       SMALLINT NOT NULL,
    level       SMALLINT NOT NULL,
    user_id     INT NOT NULL,
    view        INT NOT NULL,
    height      FLOAT NOT NULL,
    seconds     INT NOT NULL
);

-- это категория опции (например, rust, python, react native)
CREATE TABLE serve_categories (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    description     VARCHAR(10000),
    tech_categories INT NOT NULL,
    position        SMALLINT NOT NULL,
    count           SMALLINT NOT NULL,
    default_price   INT NOT NULL, -- сумма всех опуий по умолчанию.
    user_id         INT NOT NULL,
    view            INT NOT NULL,
    height          FLOAT NOT NULL,
    seconds         INT NOT NULL,

    CONSTRAINT fk_tech_category
        FOREIGN KEY(tech_categories)
            REFERENCES tech_categories(id)
);

-- это опции (например, продвинутая админка)
CREATE TABLE serve (
    id               SERIAL PRIMARY KEY,
    name             VARCHAR(100) NOT NULL,
    description      VARCHAR(10000),
    position         SMALLINT NOT NULL,
    serve_categories INT NOT NULL,
    price            INT NOT NULL,
    man_hours        SMALLINT NOT NULL,
    is_default       BOOLEAN NOT NULL, -- опция по умолчанию, т.е. без которой работа невозможна (например, админка)
    user_id          INT NOT NULL,
    tech_cat_id      INT NOT NULL,
    serve_id         INT,
    view             INT NOT NULL,
    height           FLOAT NOT NULL,
    seconds          INT NOT NULL,

    CONSTRAINT fk_serve_category
        FOREIGN KEY(serve_categories)
            REFERENCES serve_categories(id),
    CONSTRAINT fk_serve_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- связь опции с объетками сервисов, работ, товаров
CREATE TABLE serve_items (
    id       SERIAL PRIMARY KEY,
    serve_id INT NOT NULL,
    item_id  INT NOT NULL,
    types    SMALLINT NOT NULL
);

-- это те tech_categories, которые привязываются к объеткам.
-- бывают открытые (активные) и дополнительные.
CREATE TABLE tech_categories_items (
    id          SERIAL PRIMARY KEY,
    category_id INT NOT NULL,     -- тех. категория (например, создание среднего магазина)
    item_id     INT NOT NULL,
    types       SMALLINT NOT NULL, -- блог, товар ......
    is_active   SMALLINT NOT NULL -- тип: 1 - активно, 2 - неактивно
);

CREATE TABLE stat_pages (
    id      SERIAL PRIMARY KEY,
    types   SMALLINT NOT NULL,  -- главная страница, инфо ......
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL,
);
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (1, 1, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (2, 2, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (3, 3, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (4, 4, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (5, 5, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (6, 6, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (7, 7, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (8, 8, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (9, 9, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (10, 10, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (11, 11, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (12, 12, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (13, 13, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (14, 14, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (15, 21, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (16, 22, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (17, 23, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (18, 24, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (19, 25, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (20, 26, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (21, 31, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (22, 32, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (23, 33, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (24, 34, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (25, 35, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (26, 36, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (27, 37, 0, 0.0, 0) ON CONFLICT DO NOTHING;

INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (28, 41, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (29, 51, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (30, 53, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (31, 61, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (32, 71, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (33, 81, 0, 0.0, 0) ON CONFLICT DO NOTHING;
INSERT INTO stat_pages (id, types, view, height, seconds)
VALUES (34, 91, 0, 0.0, 0) ON CONFLICT DO NOTHING;
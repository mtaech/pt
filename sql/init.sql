drop table if exists source_data;
create table source_data(
    id           integer primary key autoincrement,
    name         text not null,
    path         text not null,
    ext          text not null default '',
    size         integer       default 0,
    camera_model text
);
drop table if exists target_data;
create table target_data(
    id           integer primary key autoincrement,
    name         text not null,
    path         text not null,
    ext          text not null default '',
    size         integer       default 0,
    camera_model text
);


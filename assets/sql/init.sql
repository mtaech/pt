begin;
drop table if exists source_data;
create table source_data
(
    id           integer primary key autoincrement,
    name         text not null default '',
    path         text not null default '',
    ext          text not null default '',
    size         integer       default 0,
    camera_model text not null default '',
    len_model    text not null default '',
    focal_length text not null default ''
);
drop table if exists compare_data;
create table compare_data
(
    id           integer primary key autoincrement,
    name         text not null,
    path         text not null,
    ext          text not null default '',
    size         integer       default 0,
    camera_model text null,
    len_model    text null,
    focal_length text null
);
commit;

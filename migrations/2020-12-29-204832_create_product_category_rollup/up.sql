create table product_category_rollup(
    id serial primary key,
    upper_category_id integer not null,
    lower_category_id integer not null,
    foreign key (upper_category_id) references product_category(id),
    foreign key (lower_category_id) references product_category(id)
);
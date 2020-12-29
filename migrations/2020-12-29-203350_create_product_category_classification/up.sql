create table product_category_classification (
    id serial,
    product_id integer not null,
    product_category_id integer not null,
    primary key (id),
    foreign key (product_id) references product(id),
    foreign key (product_category_id) references product_category(id)
);
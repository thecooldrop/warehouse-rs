create table inventory_item (
    id serial primary key,
    product_id integer not null,
    instance_description text,
    foreign key (product_id) references product(id) on delete restrict
);
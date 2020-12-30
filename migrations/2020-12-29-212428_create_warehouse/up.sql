create table warehouse(
    id serial primary key,
    description text not null
);

create table item_warehouse_location(
    id serial primary key,
    warehouse_id integer not null,
    inventory_item_id integer not null,
    foreign key (warehouse_id) references warehouse(id) ,
    foreign key (inventory_item_id) references inventory_item(id)
);
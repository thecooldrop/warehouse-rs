create table warehouse(
    id serial primary key,
    description text not null
);

alter table inventory_item add column warehouse_id integer;
alter table inventory_item add constraint  warehouse_fk foreign key (warehouse_id) references warehouse(id) on delete restrict;
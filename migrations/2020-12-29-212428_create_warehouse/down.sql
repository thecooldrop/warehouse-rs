alter table inventory_item drop constraint warehouse_fk;
alter table inventory_item drop column warehouse_id;
drop table warehouse;
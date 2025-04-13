create table if not exists menu (
	name text not null primary key,
	price integer not null,
	available boolean default true not null,
	special boolean default false not null
);

create table if not exists receipts (
	time datetime not null default CURRENT_TIMESTAMP,
	method TEXT default Swish not null,
	primary key(time)
);

create table if not exists receipt_item (
	receipt datetime not null,
	item text not null,
	amount integer default 1 not null,
	foreign key(receipt) references Receipts(time),
	foreign key(item) references menu(name) on update cascade,
	primary key(receipt, item)
);

create table if not exists password (
	password text not null,
	primary key(password)
);

create view if not exists receipts_view as
	select receipts.time, receipt_item.item, receipt_item.amount, menu.price, menu.special, receipts.method
	from receipts
		inner join receipt_item on receipts.time = receipt_item.receipt
		inner join menu on receipt_item.item = menu.name;

insert or ignore into menu (name, price, available, special)
	values
		('Special', 1, true, true),
		('Rabatt', -1, true, true);

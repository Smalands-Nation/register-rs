create table if not exists menu (
	name text not null primary key,
	price integer not null,
	available boolean default true not null
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
	foreign key(item) references menu(name),
	primary key(receipt, item)
);

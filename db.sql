create table if not exists menu (
	name text not null primary key,
	price integer,
	available boolean default true
);

create table if not exists reciepts (
	time datetime default CURRENT_TIMESTAMP
	constraint receipts_pk primary key,
	items text default "{}",
	sum int,
	method TEXT default Swish not null
);



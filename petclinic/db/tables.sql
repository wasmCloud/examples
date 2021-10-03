--Note that there are no foreign keys defined here.
--This is to keep the create table syntax as db-neutral
--as possible and also to simplify the demonstration. You'll
--want to use foreign keys, indexes, etc in a real-world
--application.

create table if not exists owners (
    id bigint not null,
    address varchar(255) not null,
    city varchar(255) not null,
    email varchar(255) not null,
    firstname varchar(50) not null,
    lastname varchar(50) not null,
    telephone varchar(20) not null
);
create table if not exists pets (
    id bigint not null,
    pettype bigint not null,
    name varchar(50) not null
    bday int not null,
    bmonth int not null,
    byear int not null,
    ownerid bigint not null
);
create table if not exists pettypes (
    id bigint not null,
    name varchar(50) not null,
    
);
create table if not exists visits {
    day int not null,
    month int not null,
    year int not null,
    description varchar(255) not null,
    petid bigint not null,
    vetid bigint not null,
    ownerid bigint not null,
    hour int not null,
    minute int not null
}
create table if not exists vets {
    id bigint not null,
    firstname varchar(50) not null,
    lastname varchar(50) not null,
    specialties varchar(1000) not null
}
#!/usr/bin/env bash

docker exec -i --user postgres "$1" createdb -p "$2" balancedb

docker exec -i --user postgres "$1" psql -p "$2" balancedb -a  <<__END
create user rust_ch password 'open-bank';
__END

docker exec -i "$1" psql -Urust_ch -p "$2" balancedb -a <<__END
drop table if exists balancer;
drop table if exists cacr;
drop table if exists cmtr;

CREATE OR REPLACE FUNCTION maintain_updated_at()
RETURNS TRIGGER AS \$\$
BEGIN
   NEW.updated_at = now();
   RETURN NEW;
END;
\$\$ language 'plpgsql';

create table balancer(
    balance_id int generated by default as identity primary key,
    iban text not null,
    token text not null,
    amount bigint not null default 0,
    type_ text not null,
    lmt bigint not null default -50000,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp);

create trigger balance_updated_at before update
on balancer for each row execute procedure
maintain_updated_at();

insert into balancer (balance_id, iban, token, amount, type_, lmt) values
    (0, 'NL66OPEN0000000000', '00000000000000000000', 100000000000000000, 'AUTO', -50000);

alter table balancer alter column balance_id restart with 1;

create index balance_iban on balancer using btree (iban);

create table cacr(
    uuid UUID not null primary key,
    iban text,
    token text,
    type_ text,
    reason text,
    created_at timestamp not null default current_timestamp);

create table cmtr(
    uuid UUID not null primary key,
    reason text,
    created_at timestamp not null default current_timestamp);
__END

docker exec -i --user postgres "$1" createdb -p "$2" transactiondb

docker exec -i --user postgres "$1" psql -p "$2" transactiondb -a  <<__END
create user clojure_ge password 'open-bank';
__END

docker exec -i "$1" psql -Uclojure_ge -p "$2" transactiondb -a <<__END
drop table if exists transaction;

create table transaction(
    id int generated by default as identity primary key,
    iban text not null,
    new_balance text not null,
    changed_by text not null,
    from_to text not null,
    direction text not null,
    descr text not null);

create index transaction_iban on transaction using btree (iban);

create table account(
    username text not null primary key,
    password text not null,
    uuid UUID not null);

create index account_uuid on account using btree (uuid);
__END
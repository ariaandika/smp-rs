

create table users (
  user_id serial,
  name text not null,
  password text not null
);

alter table users add primary key (user_id);


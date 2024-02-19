CREATE TABLE IF NOT EXISTS sensor_list
(
    id          SERIAL PRIMARY KEY,
    sensor_info JSON NOT NULL

);


create table sensor_var_char(
	id varchar,
	name varchar
);
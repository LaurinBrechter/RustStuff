create table rust_test_db.sensor_readings (
    id serial primary key,
    sensor_id integer not null,
    timestamp integer not null,
    value float not null
);
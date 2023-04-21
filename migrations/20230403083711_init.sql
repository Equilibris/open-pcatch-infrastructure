-- Add migration script here
CREATE TABLE ObjectType (
    id          SERIAL primary key,
    name        varchar NOT NULL,
    ky          varchar NOT NULL,
    description varchar NOT NULL,
    locale_key  varchar NOT NULL,
    deleted     boolean NOT NULL
);

CREATE TABLE AttributeType (
    id          SERIAL primary key,

    name        varchar UNIQUE NOT NULL,
    ky          varchar NOT NULL,
    locale_key  varchar NOT NULL,

    datatype    varchar NOT NULL
);

CREATE TABLE Attribute (
    id          SERIAL primary key,

    ty          varchar NOT NULL references AttributeType(name),

    json_data   varchar NOT NULL -- could be JSON or JSONb but i cant be bothered
);

CREATE TABLE Entity (
    id                  SERIAL primary key,

    parent_id           int,

    latitude_position   real        NOT NULL,
    longitude_position  real        NOT NULL,

    created_by          varchar     NOT NULL,
    edited_by           varchar     NOT NULL,

    created_at          timestamptz NOT NULL,
    edited_at           timestamptz NOT NULL,

    geo_json            varchar     NOT NULL,
    prop_json           varchar     NOT NULL,
    ky                  varchar     NOT NULL,
    ty                  SERIAL      NOT NULL references objectType(id),
    deleted             boolean     NOT NULL
);

CREATE TABLE EntityAttributesJunction (
    id           SERIAL primary key,

    entity_id    SERIAL NOT NULL references entity(id),
    attribute_id SERIAL NOT NULL references entity(id)
);

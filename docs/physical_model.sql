CREATE EXTENSION postgis;


-- PostgreSQL version: 9.4
-- Model Author: Kamil Markiewicz <k.a.markiewicz@gmail.com>


-- Database creation must be done outside an multicommand file.
-- These commands were put in this file only for convenience.
-- -- object: black_dealer | type: DATABASE --
-- -- DROP DATABASE IF EXISTS black_dealer;
-- CREATE DATABASE black_dealer
-- 	OWNER = postgres
-- ;
-- -- ddl-end --
-- 

-- object: public.contacts | type: TABLE --
-- DROP TABLE IF EXISTS public.contacts CASCADE;
CREATE TABLE public.contacts(
	contact_id serial NOT NULL,
	forename varchar(32),
	surname varchar(64),
	created timestamp DEFAULT now(),
	CONSTRAINT contact_pk PRIMARY KEY (contact_id)

);
-- ddl-end --
ALTER TABLE public.contacts OWNER TO postgres;
-- ddl-end --

-- object: public.products | type: TABLE --
-- DROP TABLE IF EXISTS public.products CASCADE;
CREATE TABLE public.products(
	product_id serial NOT NULL,
	name varchar,
	description varchar,
	CONSTRAINT product_pk PRIMARY KEY (product_id)

);
-- ddl-end --
ALTER TABLE public.products OWNER TO postgres;
-- ddl-end --

-- object: public.deals | type: TABLE --
-- DROP TABLE IF EXISTS public.deals CASCADE;
CREATE TABLE public.deals(
	deal_id serial NOT NULL,
	offer_id integer,
	address_id integer,
	contact_id integer,
	status smallint,
	timestamp timestamp,
	CONSTRAINT deal_pk PRIMARY KEY (deal_id)

);
-- ddl-end --
ALTER TABLE public.deals OWNER TO postgres;
-- ddl-end --

-- object: public.offers | type: TABLE --
-- DROP TABLE IF EXISTS public.offers CASCADE;
CREATE TABLE public.offers(
	offer_id serial NOT NULL,
	price smallint,
	description varchar,
	CONSTRAINT offer_pk PRIMARY KEY (offer_id)

);
-- ddl-end --
ALTER TABLE public.offers OWNER TO postgres;
-- ddl-end --

-- object: public.availabilities | type: TABLE --
-- DROP TABLE IF EXISTS public.availabilities CASCADE;
CREATE TABLE public.availabilities(
	availability_id serial NOT NULL,
	product_id integer,
	quantity integer,
	description varchar,
	CONSTRAINT availability_pk PRIMARY KEY (availability_id)

);
-- ddl-end --
ALTER TABLE public.availabilities OWNER TO postgres;
-- ddl-end --

-- object: public.addresses | type: TABLE --
-- DROP TABLE IF EXISTS public.addresses CASCADE;
CREATE TABLE public.addresses(
	address_id serial NOT NULL,
	address varchar(128),
	postal_code varchar(16),
	city varchar(64),
	state varchar(32),
	country varchar(128),
	geopoint geography(POINT),
	CONSTRAINT address_pk PRIMARY KEY (address_id)

);
-- ddl-end --
ALTER TABLE public.addresses OWNER TO postgres;
-- ddl-end --

-- object: public.contact_details | type: TABLE --
-- DROP TABLE IF EXISTS public.contact_details CASCADE;
CREATE TABLE public.contact_details(
	contact_detail_id serial NOT NULL,
	contact_id integer,
	type varchar(16),
	value text,
	CONSTRAINT contact_detail_pk PRIMARY KEY (contact_detail_id)

);
-- ddl-end --
ALTER TABLE public.contact_details OWNER TO postgres;
-- ddl-end --

-- object: public.bank_accounts | type: TABLE --
-- DROP TABLE IF EXISTS public.bank_accounts CASCADE;
CREATE TABLE public.bank_accounts(
	bank_account_id serial NOT NULL,
	account_number varchar(128),
	owner_name varchar(128),
	balance money,
	CONSTRAINT bank_account_pk PRIMARY KEY (bank_account_id)

);
-- ddl-end --
COMMENT ON TABLE public.bank_accounts IS 'Used for money laundering';
-- ddl-end --
ALTER TABLE public.bank_accounts OWNER TO postgres;
-- ddl-end --

-- object: public.many_offers_has_many_products | type: TABLE --
-- DROP TABLE IF EXISTS public.many_offers_has_many_products CASCADE;
CREATE TABLE public.many_offers_has_many_products(
	offer_id integer,
	offers integer,
	price money,
	CONSTRAINT many_offers_has_many_products_pk PRIMARY KEY (offer_id,offers)

);
-- ddl-end --

-- object: offers_fk | type: CONSTRAINT --
-- ALTER TABLE public.many_offers_has_many_products DROP CONSTRAINT IF EXISTS offers_fk CASCADE;
ALTER TABLE public.many_offers_has_many_products ADD CONSTRAINT offers_fk FOREIGN KEY (offer_id)
REFERENCES public.offers (offer_id) MATCH FULL
ON DELETE RESTRICT ON UPDATE CASCADE;
-- ddl-end --

-- object: products_fk | type: CONSTRAINT --
-- ALTER TABLE public.many_offers_has_many_products DROP CONSTRAINT IF EXISTS products_fk CASCADE;
ALTER TABLE public.many_offers_has_many_products ADD CONSTRAINT products_fk FOREIGN KEY (offers)
REFERENCES public.products (product_id) MATCH FULL
ON DELETE RESTRICT ON UPDATE CASCADE;
-- ddl-end --

-- object: contacts_fk | type: CONSTRAINT --
-- ALTER TABLE public.contact_details DROP CONSTRAINT IF EXISTS contacts_fk CASCADE;
ALTER TABLE public.contact_details ADD CONSTRAINT contacts_fk FOREIGN KEY (contact_id)
REFERENCES public.contacts (contact_id) MATCH FULL
ON DELETE SET NULL ON UPDATE CASCADE;
-- ddl-end --

-- object: public.categories | type: TABLE --
-- DROP TABLE IF EXISTS public.categories CASCADE;
CREATE TABLE public.categories(
	category_id serial NOT NULL,
	name varchar,
	CONSTRAINT category_pk PRIMARY KEY (category_id)

);
-- ddl-end --
ALTER TABLE public.categories OWNER TO postgres;
-- ddl-end --

-- object: public.many_contacts_has_many_addresses | type: TABLE --
-- DROP TABLE IF EXISTS public.many_contacts_has_many_addresses CASCADE;
CREATE TABLE public.many_contacts_has_many_addresses(
	contact_id_contacts integer,
	address_id_addresses integer,
	CONSTRAINT many_contacts_has_many_addresses_pk PRIMARY KEY (contact_id_contacts,address_id_addresses)

);
-- ddl-end --

-- object: contacts_fk | type: CONSTRAINT --
-- ALTER TABLE public.many_contacts_has_many_addresses DROP CONSTRAINT IF EXISTS contacts_fk CASCADE;
ALTER TABLE public.many_contacts_has_many_addresses ADD CONSTRAINT contacts_fk FOREIGN KEY (contact_id_contacts)
REFERENCES public.contacts (contact_id) MATCH FULL
ON DELETE RESTRICT ON UPDATE CASCADE;
-- ddl-end --

-- object: addresses_fk | type: CONSTRAINT --
-- ALTER TABLE public.many_contacts_has_many_addresses DROP CONSTRAINT IF EXISTS addresses_fk CASCADE;
ALTER TABLE public.many_contacts_has_many_addresses ADD CONSTRAINT addresses_fk FOREIGN KEY (address_id_addresses)
REFERENCES public.addresses (address_id) MATCH FULL
ON DELETE RESTRICT ON UPDATE CASCADE;
-- ddl-end --

-- object: contacts_fk | type: CONSTRAINT --
-- ALTER TABLE public.deals DROP CONSTRAINT IF EXISTS contacts_fk CASCADE;
ALTER TABLE public.deals ADD CONSTRAINT contacts_fk FOREIGN KEY (contact_id)
REFERENCES public.contacts (contact_id) MATCH FULL
ON DELETE SET NULL ON UPDATE CASCADE;
-- ddl-end --

-- object: public.many_products_has_many_categories | type: TABLE --
-- DROP TABLE IF EXISTS public.many_products_has_many_categories CASCADE;
CREATE TABLE public.many_products_has_many_categories(
	product_id_products integer,
	category_id_categories integer,
	CONSTRAINT many_products_has_many_categories_pk PRIMARY KEY (product_id_products,category_id_categories)

);
-- ddl-end --

-- object: products_fk | type: CONSTRAINT --
-- ALTER TABLE public.many_products_has_many_categories DROP CONSTRAINT IF EXISTS products_fk CASCADE;
ALTER TABLE public.many_products_has_many_categories ADD CONSTRAINT products_fk FOREIGN KEY (product_id_products)
REFERENCES public.products (product_id) MATCH FULL
ON DELETE RESTRICT ON UPDATE CASCADE;
-- ddl-end --

-- object: categories_fk | type: CONSTRAINT --
-- ALTER TABLE public.many_products_has_many_categories DROP CONSTRAINT IF EXISTS categories_fk CASCADE;
ALTER TABLE public.many_products_has_many_categories ADD CONSTRAINT categories_fk FOREIGN KEY (category_id_categories)
REFERENCES public.categories (category_id) MATCH FULL
ON DELETE RESTRICT ON UPDATE CASCADE;
-- ddl-end --

-- object: products_fk | type: CONSTRAINT --
-- ALTER TABLE public.availabilities DROP CONSTRAINT IF EXISTS products_fk CASCADE;
ALTER TABLE public.availabilities ADD CONSTRAINT products_fk FOREIGN KEY (product_id)
REFERENCES public.products (product_id) MATCH FULL
ON DELETE SET NULL ON UPDATE CASCADE;
-- ddl-end --

-- object: offers_fk | type: CONSTRAINT --
-- ALTER TABLE public.deals DROP CONSTRAINT IF EXISTS offers_fk CASCADE;
ALTER TABLE public.deals ADD CONSTRAINT offers_fk FOREIGN KEY (offer_id)
REFERENCES public.offers (offer_id) MATCH FULL
ON DELETE SET NULL ON UPDATE CASCADE;
-- ddl-end --

-- object: addresses_fk | type: CONSTRAINT --
-- ALTER TABLE public.deals DROP CONSTRAINT IF EXISTS addresses_fk CASCADE;
ALTER TABLE public.deals ADD CONSTRAINT addresses_fk FOREIGN KEY (address_id)
REFERENCES public.addresses (address_id) MATCH FULL
ON DELETE SET NULL ON UPDATE CASCADE;
-- ddl-end --



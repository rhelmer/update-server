--
-- PostgreSQL database dump
--

-- Dumped from database version 9.6.1
-- Dumped by pg_dump version 9.6.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'SQL_ASCII';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


SET search_path = public, pg_catalog;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: locales; Type: TABLE; Schema: public; Owner: rhelmer
--

CREATE TABLE locales (
    name character varying,
    added date
);


ALTER TABLE locales OWNER TO rhelmer;

--
-- Name: platforms; Type: TABLE; Schema: public; Owner: rhelmer
--

CREATE TABLE platforms (
    name character varying,
    version integer NOT NULL,
    added date
);


ALTER TABLE platforms OWNER TO rhelmer;

--
-- Name: products; Type: TABLE; Schema: public; Owner: rhelmer
--

CREATE TABLE products (
    name character varying,
    added date
);


ALTER TABLE products OWNER TO rhelmer;

--
-- Name: releases; Type: TABLE; Schema: public; Owner: rhelmer
--

CREATE TABLE releases (
    product_id integer NOT NULL,
    version_id integer NOT NULL,
    platform_id integer NOT NULL,
    locale_id integer NOT NULL
);


ALTER TABLE releases OWNER TO rhelmer;

--
-- Name: versions; Type: TABLE; Schema: public; Owner: rhelmer
--

CREATE TABLE versions (
    product_id integer NOT NULL,
    version integer NOT NULL,
    added date
);


ALTER TABLE versions OWNER TO rhelmer;

--
-- PostgreSQL database dump complete
--


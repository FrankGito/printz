#!/bin/bash
set -e

PGPASSWORD=postgres psql -v ON_ERROR_STOP=1 --username="frank" --dbname="testo" <<-EOSQL
	CREATE TABLE ipfs_data (
    tokenId  varchar(200) CONSTRAINT pk_tokenId PRIMARY KEY,
    name     varchar(200) NOT NULL,
    uri      varchar(200) NOT NULL
  );
EOSQL

CREATE TABLE IF NOT EXISTS mandants (
    id uuid NOT NULL, 
    association_name varchar(256) NOT NULL,
    website varchar(256) NOT NULL,
    email varchar(128) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS users (
    id uuid NOT NULL,
    mandants_id uuid NOT NULL,
    username VARCHAR(64) NOT NULL,
    lastname VARCHAR(256) NOT NULL,
    email VARCHAR(128) NOT NULL,
    password_hash CHAR(64) NOT NULL,
    CONSTRAINT fk_mandants_id
      FOREIGN KEY(mandants_id) 
	  REFERENCES mandants(id)
);








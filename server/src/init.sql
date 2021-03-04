CREATE TABLE users
(
  id uuid NOT NULL,
  username varchar(100) NOT NULL,
  password varchar(150) NOT NULL,
  created_at timestamp WITHOUT TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
  updated_at timestamp WITHOUT TIME ZONE NULL,
  role varchar(20) NOT NULL,
  PRIMARY KEY (id),
  UNIQUE (username)
);
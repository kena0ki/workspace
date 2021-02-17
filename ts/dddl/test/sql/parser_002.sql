CREATE TABLE manhattan.cities (
  name VARCHAR(100) NOT NULL,
  lat DOUBLE NULL,
  lng DOUBLE,
  constrained INT NULL CONSTRAINT pkey PRIMARY KEY NOT NULL UNIQUE CHECK (constrained > 0),
  ref INT REFERENCES othertable (a, b),
  ref2 INT references othertable2 on delete cascade on update no action
)


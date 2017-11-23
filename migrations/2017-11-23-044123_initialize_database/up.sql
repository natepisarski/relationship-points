create table TestTable (
  TestId integer autoincrement,
  Value text
);

create table TestRating (
  TestId integer,
  Rating integer,
  foreign key(TestId) references TestTable(TestId)
);
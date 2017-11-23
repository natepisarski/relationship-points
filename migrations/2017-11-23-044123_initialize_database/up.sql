create table TestTable (
  TestId integer primary key autoincrement,
  Value text not null
);

create table TestRating (
  TestId integer PRIMARY KEY,
  Rating integer,
  foreign key(TestId) references TestTable(TestId)
);
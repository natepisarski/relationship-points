-- Your SQL goes here
create table Person(
  PersonId int primary key,
  FirstName text,
  LastName text,
  EmailAddress text not null,
  Age int,
  Gender text
)
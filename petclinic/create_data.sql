insert into vets values (1, 'Doggy Days', 'Vet', 'Dogs');
insert into vets values (2, 'Meow', 'Mixers', 'Cats');
insert into vets values (3, 'Pear', 'It', 'Birds');
insert into pettypes values (1, 'Dog');
insert into pettypes values (2, 'Cat');
insert into pettypes values (3, 'Bird');
-- Owners
insert into owners (id, address, city, email, firstname, lastname, telephone) values (4, '1234 Street Lane', 'Metropolis', 'brooks@email.com', 'Brooks', 'Townsend', '123-123-1234');
insert into owners (id, address, city, email, firstname, lastname, telephone) values (1, '1234 Street Lane', 'Metropolis', 'kevin@email.com', 'Kevin', 'Hoffman', '123-123-1234');
insert into owners (id, address, city, email, firstname, lastname, telephone) values (3, '1234 Street Lane', 'Metropolis', 'taylor@email.com', 'Taylor', 'Thomas', '123-123-1234');
insert into owners (id, address, city, email, firstname, lastname, telephone) values (2, '1234 Street Lane', 'Metropolis', 'connor@email.com', 'Connor', 'Smith', '123-123-1234');
insert into owners (id, address, city, email, firstname, lastname, telephone) values (5, '1234 Street Lane', 'Metropolis', 'caroline@email.com', 'Caroline', 'Tarbett', '123-123-1234');
insert into owners (id, address, city, email, firstname, lastname, telephone) values (6, '1234 Street Lane', 'Metropolis', 'bailey@email.com', 'Bailey', 'Hayes', '123-123-1234');
insert into owners (id, address, city, email, firstname, lastname, telephone) values (7, '1234 Street Lane', 'Metropolis', 'lachlan@email.com', 'Lachlan', 'Heywood', '123-123-1234');
-- Pets
insert into pets (id, pettype, name, bday, bmonth, byear, ownerid) values (1, 1, 'Archie', 5, 3, 2019, 4);
insert into pets (id, pettype, name, bday, bmonth, byear, ownerid) values (2, 1, 'Sage', 4, 3, 2020, 1);
insert into pets (id, pettype, name, bday, bmonth, byear, ownerid) values (3, 1, 'Chex', 7, 12, 2016, 3);
insert into pets (id, pettype, name, bday, bmonth, byear, ownerid) values (4, 1, 'Desmond', 11, 12, 2018, 2);
insert into pets (id, pettype, name, bday, bmonth, byear, ownerid) values (5, 1, 'Abby', 11, 10, 2011, 1);
insert into pets (id, pettype, name, bday, bmonth, byear, ownerid) values (6, 2, 'Samson', 12, 12, 2012, 5);
insert into pets (id, pettype, name, bday, bmonth, byear, ownerid) values (7, 1, 'Harley Quinn', 12, 12, 2012, 6);
insert into pets (id, pettype, name, bday, bmonth, byear, ownerid) values (8, 1, 'Loki Odin', 12, 12, 2012, 6);
insert into pets (id, pettype, name, bday, bmonth, byear, ownerid) values (9, 1, 'Nelson', 12, 12, 2012, 7);
insert into visits (day, month, year, description, petid, vetid, ownerid, hour, minute) values (2, 2, 2022, 'Annual Checkup, diagnosis: very good boy', 1, 1, 4, 12, 12);
insert into visits (day, month, year, description, petid, vetid, ownerid, hour, minute) values (9, 6, 2022, 'Her growl sounds like Donald Duck', 5, 1, 1, 16, 10);
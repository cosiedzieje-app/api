CREATE DATABASE somsiad;
USE somsiad;
CREATE TABLE `users` (
 `id` int(11) NOT NULL AUTO_INCREMENT,
 `email` varchar(255) NOT NULL UNIQUE,
 `name` varchar(255) NOT NULL UNIQUE,
 `password` varchar(255) NOT NULL,
 `salt` char(16) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE = InnoDB CHARSET=utf8mb4 COLLATE utf8mb4_polish_ci;

create table `full_users_info`(
`id` int NOT NULL AUTO_INCREMENT,
`name` varchar(30) NOT NULL,
`surname` varchar(30) NOT NULL,
`sex` ENUM('M','F','O') NOT NULL,
`address` JSON NOT NULL,
`reputation` mediumint NOT NULL,
 PRIMARY KEY (`id`)
) ENGINE = InnoDB CHARSET=utf8mb4 COLLATE utf8mb4_polish_ci;
-- Will be added later, for now it errors
-- alter table users add foreign key (id) references full_users_info (id);

CREATE TABLE `Markers` (
`id` INT UNSIGNED NOT NULL AUTO_INCREMENT ,
`coordinates` POINT NOT NULL,
`title` VARCHAR(25) NOT NULL,
`description` TEXT NOT NULL,
`type` ENUM("A","B","C") NOT NULL,
`add_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
`end_time` TIMESTAMP NULL DEFAULT NULL,
`address` JSON NOT NULL,
`contact_info` JSON NOT NULL,
`user_id` INT NOT NULL,
PRIMARY KEY (`ID`)
) ENGINE = InnoDB CHARSET=utf8mb4 COLLATE utf8mb4_polish_ci;
alter table `Markers` add foreign key (`user_id`) references users (`id`)

create user 'cursor'@'%' identified by 's0ms1a$';
grant all privileges on somsiad.* to 'cursor'@'%';
/* Example address JSON:
{
  "adress": {
    "postalCode": "41-200",
    "street": "Jagiellonska",
    "number": 13,
    "country": "Poland"
  }
} */

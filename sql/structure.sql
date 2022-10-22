CREATE DATABASE somsiad;
USE somsiad;
CREATE TABLE `users` (
 `id` int(11) NOT NULL AUTO_INCREMENT,
 `email` varchar(255) NOT NULL UNIQUE,
 `name` varchar(255) NOT NULL UNIQUE,
 `password` varchar(60) NOT NULL,
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
alter table full_users_info add foreign key (id) references users (id) on delete cascade on update cascade;

CREATE TABLE `markers` (
`id` INT UNSIGNED NOT NULL AUTO_INCREMENT ,
`latitude` double NOT NULL,
`longitude` double NOT NULL,
`title` VARCHAR(25) NOT NULL,
`description` TEXT NOT NULL,
`type` ENUM("A","B","C","D") NOT NULL,
`add_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
`start_time` TIMESTAMP NULL DEFAULT NULL,
`end_time` TIMESTAMP NULL DEFAULT NULL,
`address` JSON NOT NULL,
`contact_info` JSON NOT NULL,
`user_id` INT NOT NULL,
PRIMARY KEY (`ID`)
) ENGINE = InnoDB CHARSET=utf8mb4 COLLATE utf8mb4_polish_ci;
alter table `markers` add foreign key (`user_id`) references users (`id`)

/* Example address JSON:
{
  "address": {
    "postalCode": "41-200",
    "street": "Jagiellonska",
    "number": 13,
    "country": "Poland"
  }
} */

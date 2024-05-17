DROP DATABASE IF EXISTS `mysqlleak`;
CREATE DATABASE IF NOT EXISTS `mysqlleak`
    DEFAULT CHARACTER SET utf8mb4;

USE `mysqlleak`;

DROP TABLE IF EXISTS `rands`;
CREATE TABLE IF NOT EXISTS `rands` (
  `id`      INT(11)       NOT NULL AUTO_INCREMENT,
  `randnum1` INT(11)      NOT NULL,
  `randstr1` VARCHAR(255) NOT NULL,
  `randnum2` INT(11)      NOT NULL,
  `randstr2` VARCHAR(255) NOT NULL,
  `randnum3` INT(11)      NOT NULL,
  `randstr3` VARCHAR(255) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `k_randnum1` (`randnum1`),
  KEY `k_id_randstr1` (`id`, `randstr1`),
  KEY `k_id_randstr2` (`id`, `randstr2`),
  KEY `k_id_randstr3` (`id`, `randstr3`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

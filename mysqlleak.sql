USE `mysqlleak_demo`;

DROP TABLE IF EXISTS `times`;
DROP TABLE IF EXISTS `values`;

CREATE TABLE IF NOT EXISTS `values` (
  `id`     INT(11)      NOT NULL AUTO_INCREMENT,
  `rand_a` INT(11)      NOT NULL,
  `rand_b` INT(11)      NOT NULL,
  `rand_s` VARCHAR(255) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `k_rand_a`    (`rand_a`),
  KEY `k_id_rand_s` (`id`, `rand_s`),
  KEY `k_rand_ab`   (`rand_a`,`rand_b`),
  KEY `k_rand_as`   (`rand_a`,`rand_s`),
  KEY `fk_VALUES_RANDA_idx` (`rand_a`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `times` (
  `rand_a` int(11) NOT NULL,
  `millis` int(11) NOT NULL,
  PRIMARY KEY (`rand_a`),
  KEY `fk_TIMES_RANDA_idx` (`rand_a`),
  CONSTRAINT `fk_TIMES_RANDA_idx` FOREIGN KEY (`rand_a`) REFERENCES `values` (`rand_a`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;

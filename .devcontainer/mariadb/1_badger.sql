DROP TABLE IF EXISTS `layers`;
CREATE TABLE `layers` (
  id INT NOT NULL,
  title TEXT NOT NULL,
  asset_path TEXT NOT NULL,

  -- offset_x INT NOT NULL,
  -- offset_Y INT NOT NULL,

  event ENUM('any','user','fursuit','sponsor','staff','security','medic'),
  event_not bool NOT NULL
);
INSERT INTO `layers` VALUES
(1,'base','layers/base.png','any',0),
(6,'badge','badge','any',0),
(2,'unknown','e5b9582029a170cfb1a5caf499870dfd.png','user',0),
(5,'unknown','80ad556a7e84028bf05bdd32c73c1e12.png','staff',0),
(3,'no sponsor','6c38106661ed0b63dc77e75c4c674f54.png','sponsor',1),
(4,'sponsor','f7513a1bf8ab841035bea228a6986876.png','sponsor',0);

DROP TABLE IF EXISTS `layers_config`;
CREATE TABLE `layers_config` (
  page_width_mm FLOAT NOT NULL,
  page_height_mm FLOAT NOT NULL,

  dpi FLOAT NOT NULL,

  avatar_size_pt FLOAT NOT NULL,
  avatar_y_pt FLOAT NOT NULL,

  font_path TEXT NOT NULL,

  regnum_size FLOAT NOT NULL,
  regnum_x_pt FLOAT NOT NULL,
  regnum_y_pt FLOAT NOT NULL,

  nick_size FLOAT NOT NULL,
  nick_y_pt FLOAT NOT NULL
);
INSERT INTO `layers_config` VALUES
(54,86,300,100,100,'3f45f0decb9e9f48b25d13d40e115cc8.ttf',12,10,240,16,60);

DROP TABLE IF EXISTS `staff_asignments`;
CREATE TABLE `staff_assignments` (
  regnumber INT(11) NOT NULL,
  medic bool NOT NULL,
  security bool NOT NULL
);
INSERT INTO `staff_assignments` VALUES
(1, 0, 0);
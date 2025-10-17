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
(1,'base','93c7992ac8ab742fa705bcd87aa66f20.png','any',0),
(2,'badge','badge','any',0),
(3,'attendee','a28db4c902bb7cdeaec8395ccd81a94a.png','user',0),
(5,'staff','e5785e2e7ddc0b7c00f01b04ad9134b2.png','staff',0),
(4,'sponsor','6fd34630b74aa251f23dffa40cc74b11.png','sponsor',0),
(6,'security','736e1704501ca7086c6f2f7db4306f81.png','security',0),
(7,'medic','33a61852b70a04f9d2fd32dd288fb788.png','medic',0),
(8,'fursuit','d47e279130a97f57a6cdcf7d1d44ec9a.png','fursuit',0);


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
  regnum_color TEXT NOT NULL,

  nick_size FLOAT NOT NULL,
  nick_y_pt FLOAT NOT NULL,
  nick_color TEXT NOT NULL
);
INSERT INTO `layers_config` VALUES
(54,86,300,153,59,'ab79ea0152b4c8a85bceb66bc4c251d3.ttf',14,95,60,'#000000',16,40,'#ffffff');

DROP TABLE IF EXISTS `staff_asignments`;
CREATE TABLE `staff_assignments` (
  regnumber INT(11) NOT NULL,
  medic bool NOT NULL,
  security bool NOT NULL
);
INSERT INTO `staff_assignments` VALUES
(1, 0, 0);
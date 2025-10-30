DROP TABLE IF EXISTS `layers`;
CREATE TABLE `layers` (
  id INT NOT NULL,
  title TEXT NOT NULL,
  asset_path TEXT NOT NULL,

  -- offset_x INT NOT NULL,
  -- offset_Y INT NOT NULL,

  event ENUM('any','user','fursuit','sponsor','staff','media','ticket','ticket_convention','ticket_wed','ticket_thu','ticket_fri','ticket_sat','ticket_sun'),
  event_not bool NOT NULL,
  badge_type ENUM('any', 'fursuit', 'con_ticket', 'day_ticket')
);
INSERT INTO `layers` VALUES
(1,'badge','badge','any',0,'any'),
(2,'staff','1c909ca6483a242149a13fb10ad74ba6.png','staff',0,'any'),
(3,'media','8a4d1b51b606747798188e735436e40e.png','media',0,'any'),
(7,'day sponsor','908b3f5b86811fb856223e0a2a5ed40d.png','sponsor',0,'day_ticket'),
(8,'day ticket','7a7cda3de1103e7c89b64fd82cc163d4.png','sponsor',1,'day_ticket'),
(4,'residential','48c5d1f041291f016ae2b871ef4425c3.png','sponsor',1,'con_ticket'),
(5,'residential sponsor','24bb44784447c35aa1888d660cb95d07.png','sponsor',0,'con_ticket'),
(6,'species','e956da7bdb70bbba9a58323cd0117db3.png','any',0,'fursuit'),
(9,'wed','c3a66f078d1bff97df02453117d3ddbc.png','ticket_wed',0,'day_ticket'),
(10,'thu','97c684c7eeb46039c66f088a015eade7.png','ticket_thu',0,'day_ticket'),
(11,'fri','e34bd6257fccbd9bf440205f56b49fce.png','ticket_fri',0,'day_ticket'),
(12,'sat','101d9d1f7ed055d17a5af3ce3b705fdf.png','ticket_sat',0,'day_ticket'),
(13,'sun','8db057bdbcbcc7a8d0deef21d419b38e.png','ticket_sun',0,'day_ticket');


DROP TABLE IF EXISTS `layers_config`;
CREATE TABLE `layers_config` (
  page_width_mm FLOAT NOT NULL,
  page_height_mm FLOAT NOT NULL,

  dpi FLOAT NOT NULL,

  avatar_size_pt FLOAT NOT NULL,
  avatar_y_pt FLOAT NOT NULL,

  name_font_path TEXT NOT NULL,
  nr_font_path TEXT NOT NULL,

  regnum_size FLOAT NOT NULL,
  regnum_x_pt FLOAT NOT NULL,
  regnum_y_pt FLOAT NOT NULL,
  regnum_color TEXT NOT NULL,

  nick_size FLOAT NOT NULL,
  nick_y_pt FLOAT NOT NULL,
  nick_color TEXT NOT NULL
);
INSERT INTO `layers_config` VALUES
(54,85,300,140,36,'17d24beb28415462d4cb742152bf4b9a.ttf','17d24beb28415462d4cb742152bf4b9a.ttf',14,30,196.5,'#45271e',16,30,'#45271e');

DROP TABLE IF EXISTS `staff_asignments`;
CREATE TABLE `staff_assignments` (
  regnumber INT(11) NOT NULL,
  media bool NOT NULL
);
INSERT INTO `staff_assignments` VALUES
(1, 0);
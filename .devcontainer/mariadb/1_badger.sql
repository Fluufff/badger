DROP TABLE IF EXISTS `layers`;
CREATE TABLE `layers` (
  id INT NOT NULL,
  title TEXT NOT NULL,
  asset_path TEXT NOT NULL,

  -- offset_x INT NOT NULL,
  -- offset_Y INT NOT NULL,

  on_sponsor BOOLEAN,
  on_staff BOOLEAN,
  on_staff_security BOOLEAN,
  on_staff_medic BOOLEAN
);
INSERT INTO `layers` VALUES
(1, 'base', 'layers/base.png', 0, 0, 0, 0),
(2, 'badge', 'badge', 0, 0, 0, 0);

DROP TABLE IF EXISTS `staff_roles`;
CREATE TABLE `staff_roles` (
  regnumber INT(11) NOT NULL,
  staff_role TEXT
);

INSERT INTO `staff_roles` VALUES
(2, 'medic'),
(3, 'security');

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
(54.0, 86.0, 300.0, 100.0, 120.0, 'LavishlyYours-Regular.ttf', 12.0, 10.0, 240.0, 16.0, 60.0);

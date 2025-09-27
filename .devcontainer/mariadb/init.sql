DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `users` (
  `regnumber` int(11) NOT NULL AUTO_INCREMENT,
  `email` varchar(50) NOT NULL,
  `hash` char(255) NOT NULL,
  `reset_hash` char(255) DEFAULT NULL,
  `reset_time` int(10) unsigned NOT NULL,
  `creation_date` timestamp NOT NULL DEFAULT current_timestamp(),
  `firstname` varchar(100) DEFAULT NULL,
  `lastname` varchar(100) DEFAULT NULL,
  `nick` varchar(100) DEFAULT NULL,
  `privacy` set('Y','N') NOT NULL DEFAULT 'N',
  `terms` set('Y','N') NOT NULL DEFAULT 'N',
  `registered` set('Y','N') NOT NULL DEFAULT 'N',
  `balance` int(12) NOT NULL DEFAULT 0,
  `gid` int(3) NOT NULL DEFAULT 1,
  `double_auth` tinyint(1) DEFAULT 0,
  `double_auth_secret` char(40) DEFAULT NULL,
  `locked` set('Y','N') DEFAULT 'N',
  PRIMARY KEY (`regnumber`)
) ENGINE=InnoDB AUTO_INCREMENT=494 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

DROP TABLE IF EXISTS `layers`;
CREATE TABLE `layers` (
  id INT AUTO_INCREMENT PRIMARY KEY,
  title TEXT NOT NULL,
  asset_path TEXT NOT NULL,

  on_sponsor BOOLEAN,
  on_staff BOOLEAN,
  on_staff_security BOOLEAN,
  on_staff_medic BOOLEAN
);
-- Admin 个人委托简历. PHP 安装包没有这张表，列名按 resume_trust.vue 反推.

CREATE TABLE IF NOT EXISTS `phpyun_user_entrust` (
  `id`         int(11)       NOT NULL AUTO_INCREMENT,
  `uid`        int(11)       NOT NULL DEFAULT 0 COMMENT '求职者会员id',
  `eid`        int(11)       NOT NULL DEFAULT 0 COMMENT 'resume_expect.id',
  `price`      decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '委托价格',
  `status`     tinyint(1)    NOT NULL DEFAULT 0 COMMENT '0未审核 1已接受 2不接受',
  `add_time`   int(11)       NOT NULL DEFAULT 0 COMMENT '申请时间',
  `audit_time` int(11)       NOT NULL DEFAULT 0,
  `auid`       int(11)       NOT NULL DEFAULT 0 COMMENT '审核管理员',
  PRIMARY KEY (`id`),
  KEY `uid` (`uid`),
  KEY `eid` (`eid`),
  KEY `status` (`status`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

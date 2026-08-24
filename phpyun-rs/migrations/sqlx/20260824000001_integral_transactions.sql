-- Integral balance updates rely on transactions and SELECT ... FOR UPDATE.
-- The legacy PHPYun schema declares these tables as MyISAM, where both are
-- ineffective. A balance also has exactly one logical row per uid; making that
-- invariant explicit lets INSERT IGNORE create missing rows without duplicates.
--
-- Adding the unique keys first deliberately stops the migration when legacy
-- duplicate uid rows exist. Those rows require an operator-reviewed cleanup;
-- silently choosing or merging balances would risk losing user points.

ALTER TABLE `phpyun_member_statis`
    ADD UNIQUE KEY `uk_phpyun_member_statis_uid` (`uid`);

ALTER TABLE `phpyun_company_statis`
    ADD UNIQUE KEY `uk_phpyun_company_statis_uid` (`uid`);

ALTER TABLE `phpyun_member_statis` ENGINE=InnoDB;
ALTER TABLE `phpyun_company_statis` ENGINE=InnoDB;
ALTER TABLE `phpyun_company_pay` ENGINE=InnoDB;

-- phpyun_rs_chat is owned by this port, not by PHP. Rename the status column
-- to the short name used on the wire (`cs`) and add `ctype` so content kind
-- is stored, not only reserved in the JSON protocol.
ALTER TABLE `phpyun_rs_chat`
  CHANGE COLUMN `is_read` `cs` TINYINT UNSIGNED NOT NULL DEFAULT 0,
  ADD COLUMN `ctype` TINYINT UNSIGNED NOT NULL DEFAULT 0 AFTER `cs`,
  DROP INDEX `ix_receiver_unread`,
  ADD INDEX `ix_receiver_unread` (`receiver_uid`, `cs`);

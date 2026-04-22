<?php

class db_tool {
	var $querynum = 0;
	var $link;
	var $histories;
	var $time;
	var $tablepre;
	function connect($dbhost, $dbuser, $dbpw, $dbname = '', $dbcharset, $pconnect = 0, $tablepre='', $time = 0) {
		$this->time = $time;
		$this->tablepre = $tablepre;
		
		if(!$this->link = mysqli_connect($dbhost, $dbuser, $dbpw, $dbname)) {
			$this->halt('无法连接数据库!');
		}
		
		if($this->version() > '4.1') {
			if($dbcharset) {
				mysqli_query($conn,"SET character_set_connection=".$dbcharset.", character_set_results=".$dbcharset.", character_set_client=binary", $this->link);
			}

			if($this->version() > '5.0.1') {
				mysqli_query($conn,"SET sql_mode=''", $this->link);
			}
		}
	}
	function cache_gc() {
		$this->query("DELETE FROM {$this->tablepre}sqlcaches WHERE expiry<$this->time");
	}
	function query($sql, $type = '', $cachetime = FALSE) {
		$func = $type == 'UNBUFFERED' && @function_exists('mysqli_unbuffered_query') ? 'mysqli_unbuffered_query' : 'mysqli_query';
		if(!($query = $func($sql, $this->link)) && $type != 'SILENT') {
			$this->halt('SQL:', $sql);
		}
		$this->querynum++;
		$this->histories[] = $sql;
		return $query;
	}
	function error() {
		return (($this->link) ? mysqli_error($this->link) : mysqli_error($this->link));
	}
	function errno() {
		return intval(($this->link) ? mysqli_errno($this->link) : mysqli_errno($this->link));
	}
	function result($query, $row) {
		$query = @mysqli_result($query, $row);
		return $query;
	}
	function version() {
		return mysqli_get_server_info($this->link);
	}
	function close() {
		return mysqli_close($this->link);
	}
	function halt($message = '', $sql = '') {
		show_msg('run_sql_error', $message.$sql.'<br /> Error:'.$this->error().'<br />Errno:'.$this->errno(), 0);
	}
}
?>
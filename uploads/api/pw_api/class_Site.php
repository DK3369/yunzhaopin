<?php

!defined('P_W') && exit('Forbidden');
//api mode 9

class Site {

	var $base;
	var $db;

	function __construct($base) {
		$this->Site($base);
	}

	function Site($base) {
		$this->base = $base;
		$this->db = $base->db;
	}

	function connect() {
		return new ApiResponse(1);
	}
}
?>

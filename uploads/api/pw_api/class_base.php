<?php

!defined('P_W') && exit('Forbidden');

define('API_CLOSED', 1);
define('API_SIGN_ERROR', 2);
define('API_MODE_NOT_EXISTS', 3);
define('API_METHOD_NOT_EXISTS', 4);

class ApiResponse {

	var $result;
	var $mode;

	function __construct($res, $mode = null) {
		$this->ApiResponse($res, $mode);
	}

	function ApiResponse($res, $mode = null) {
		$this->result = $res;
		$this->mode = $mode;
	}

	function getResult() {
		return $this->result;
	}

	function getMode() {
		return $this->mode;
	}
}

class ErrorMsg {

	var $errCode = 0;
	var $errMessage = '';

	function __construct($errCode, $errMessage) {
		$this->ErrorMsg($errCode, $errMessage);
	}

	function ErrorMsg($errCode, $errMessage) {
		$this->errCode = $errCode;
		$this->errMessage = $errMessage;
	}

	function getErrCode() {
		return $this->errCode;
	}

	function getErrMessage() {
		return $this->errMessage;
	}

	function getResult() {
		return null;
	}
}

class api_client {

	var $type;
	var $apikey;
	var $charset;
	var $db;
	var $classdb;
    var $siteappkey;

	function __construct() {
		$this->api_client();
	}

	function api_client() {
		global $mysqli;
		$this->apikey	= '';
		$this->type		= '';
        $this->siteappkey ='';
		$this->db		=& $mysqli;
		$this->classdb	= array();
		$this->charset	= UC_CHARSET;
	}

	function run($request) {
		global $mysqli,$config;
		if (!is_array($request) || count($request) > 20) {
			return new ErrorMsg(API_SIGN_ERROR, 'Invalid Request');
		}
		foreach ($request as $value) {
			if (!is_scalar($value) && $value !== null) {
				return new ErrorMsg(API_SIGN_ERROR, 'Invalid Request');
			}
		}
		$paramsRaw = isset($request['params']) && is_scalar($request['params']) ? (string) $request['params'] : '';
		if (strlen($paramsRaw) > 1048576) {
			return new ErrorMsg(API_SIGN_ERROR, 'Invalid Request');
		}
		if (isset($request['type']) && $request['type'] == 'uc') {
			$this->type		= 'uc';
			$this->apikey	= UC_KEY;
		} else {
			$this->type		= 'app';
			$this->apikey	= UC_APPID;
            $this->siteappkey = UC_KEY;
		}
		/***
		if ($this->type == 'app' && !$GLOBALS['o_appifopen']) {
			return new ErrorMsg(API_CLOSED, 'App Closed');
		}
		***/
		ksort($request);
		reset($request);
		$arg = '';
		foreach ($request as $key => $value) {
			if ($value && $key!='sig') {
				$arg.="$key=$value&";
			}
		}
		$signature = isset($request['sig']) && is_scalar($request['sig']) ? strtolower((string) $request['sig']) : '';
		$expected = md5($arg.$this->apikey);
		if(empty($this->apikey) || !preg_match('/^[a-f0-9]{32}$/D', $signature)
			|| !hash_equals($expected, $signature)) {
			return new ErrorMsg(API_SIGN_ERROR, 'Error Sign');
		}
		$mode	= isset($request['mode']) && is_scalar($request['mode']) ? (string) $request['mode'] : '';
		$method	= isset($request['method']) && is_scalar($request['method']) ? (string) $request['method'] : '';
		if (!preg_match('/^[A-Za-z][A-Za-z0-9_]{0,31}$/D', $mode)
			|| !preg_match('/^[A-Za-z][A-Za-z0-9_]{0,63}$/D', $method)
			|| !$this->routeAllowed($mode, $method)) {
			return new ErrorMsg(API_METHOD_NOT_EXISTS, 'Invalid Route');
		}
		//echo $request['params'];
		$params = $paramsRaw !== '' ? @unserialize($paramsRaw, array('allowed_classes' => false)) : array();
		if (!is_array($params) || $this->containsObject($params)) {
			return new ErrorMsg(API_SIGN_ERROR, 'Invalid Params');
		}
        if (isset($params['appthreads'])) {
			require_once(R_P.'class_json.php');
			$json = new Services_JSON(true);
			$params['appthreads'] = $json->decode(@gzuncompress($params['appthreads']));
        }

		if ($params && isset($request['charset'])) {
			$params = pwConvert($params,$this->charset,$request['charset']);
		}
		//print_r($this->callback($mode, $method, $params));
		return $this->callback($mode, $method, $params);
	}

	function routeAllowed($mode, $method) {
		$routes = array(
			'Credit'  => array('get', 'syncredit', 'getvalue'),
			'Invite'  => array('get'),
			'Msg'     => array('send', 'SendAppmsg'),
			'Site'    => array('connect'),
			'User'    => array('getInfo', 'alterName', 'deluser', 'synlogin', 'synlogout', 'getusergroup', 'getphpyun'),
			'UserApp' => array('isInstall', 'add', 'appsUpdateCache')
		);
		return isset($routes[$mode]) && in_array($method, $routes[$mode], true);
	}

	function containsObject($value, $depth = 0) {
		if ($depth > 32 || is_object($value) || is_resource($value)) {
			return true;
		}
		if (is_array($value)) {
			foreach ($value as $item) {
				if ($this->containsObject($item, $depth + 1)) {
					return true;
				}
			}
		}
		return false;
	}

	function callback($mode, $method, $params) {

		if (!isset($this->classdb[$mode])) {
			if (!file_exists(R_P.'class_' . $mode . '.php')) {
				return new ErrorMsg(API_MODE_NOT_EXISTS, "Class($mode) Not Exists");
			}
			require_once Pcv(R_P.'class_' . $mode . '.php');
			$this->classdb[$mode] = new $mode($this);
		}

		if (!method_exists($this->classdb[$mode], $method)) {
			return new ErrorMsg(API_METHOD_NOT_EXISTS, "Method($method of $mode) Not Exists");
		}
		!is_array($params) && $params = array();
		//print_r($params);
		return @call_user_func_array(array(&$this->classdb[$mode],$method), $params);
	}

	function dataFormat($data) {
		$res = array(
			'charset' => $this->charset
		);
		if (strtolower(get_class($data)) == 'apiresponse') {
			$res['result'] = $data->getResult();
		} else {
			$res['errCode'] = $data->getErrCode();
			$res['errMessage'] = $data->getErrMessage();
		}
		return serialize($res);
	}
	function strips($param) {
		if (is_array($param)) {
			foreach ($param as $key => $value) {
				$param[$key] = $this->strips($value);
			}
		} else {
			$param = stripslashes($param);
		}
		return $param;
	}
	function strcode($string, $encode = true) {
		!$encode && $string = base64_decode($string);
		$code = '';
		$key  = substr(md5((string) ($_SERVER['HTTP_USER_AGENT'] ?? '') . $this->apikey),8,18);
		 $keylen = strlen($key);
		 $strlen = strlen($string);
		for ($i = 0; $i < $strlen; $i++) {
			$k		= $i % $keylen;
			$code  .= $string[$i] ^ $key[$k];
		}
		return $encode ? base64_encode($code) : $code;
	}



}
?>

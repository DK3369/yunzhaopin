<?php
/*
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */ 

include(dirname(__FILE__).'/global.php');

if($init_type=='member'){

	$common_model = 'index';

	require (APP_PATH . 'app/public/common.php');

	if ($_GET['c'] && !preg_match("/^[0-9a-zA-Z\_]*$/", $_GET['c'])) {
	    $_GET['c']  =   'index';
	}

	$model  =   $_GET['c'];
	$action =   $_GET['act'];

	if ($model == "") $model = "index";
	if ($action == "") $action = "index";

	$usertype   =   $_COOKIE['usertype'];

	if ($usertype == 1) {

	    $type   =   "user";
	} else if ($usertype == 2) {

	    $type   =   "com";
	} else {

	    if ($_COOKIE['uid']) {

	        header('Location: ' . Url('register', array('c' => 'ident')));
	        die;
	    } else {

	        header('Location: ' . Url('login', array('c' => 'ident')));
	        die;
	    }
	}
	
	if ($_GET['m'] == 'ajax') {

	    require(APP_PATH.'member/ajax.class.php');
	    $model  =   $_GET['m'];
	    $action =   $_GET['c'];
	} else {

	    require(APP_PATH.'member/'.$type . "/" . $type . '.class.php');
	    require(APP_PATH.'member/'.$type . "/model/" . $model . '.class.php');
	}

	$common_model = 'member';

	$conclass = $model . '_controller';
	$actfunc = $action . '_action';
	$views = new $conclass($phpyun, $db, $db_config['def'],$common_model);
	if (! method_exists($views, $actfunc)) {
	    $views->DoException();
	}

	$views->$actfunc();
	
}else{

	if(isset($_GET['m']) && $_GET['m'] && !preg_match('/^[0-9a-zA-Z\_]*$/',$_GET['c'])){
		$_GET['m'] = 'index';
	}

	$ModuleName = isset($_GET['m']) ? $_GET['m'] : '';
	if($ModuleName=='')	$ModuleName='index';
	//默认情况下，调用app/controller下与当前目录名相同的模块
	//如当前目录名为ask，则默认调用的是app/controller/ask/下的控制器
	//若需要调用与当前目录名不同的模块，则请修改此处的$ModuleName
	//$ModuleName='job'，则此时将调用app/controller/job/下的控制器

	include(LIB_PATH.'init.php');
}


?>
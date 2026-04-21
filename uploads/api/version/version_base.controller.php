<?php
/*
* $Author ：PHPYUN开发团队
* 官网: http://www.phpyun.com
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
*/
class version_base_controller extends common{
	function __construct($tpl,$db,$def='',$model='index',$m='') {
		$this->common($tpl,$db,$def,$model,$m);
		if ($_GET['m'] == 'version'){
		    return true;
		}else{
		    return false;
		}
	}
	public function render_json($error='', $msg='', $data = array()) {
	    $result = array(
	        'error'  =>  $error,
	        'msg'    =>  $msg,
	        'data'   =>  $data
	    );
	    header('content-type:application/json; charset=utf-8');
	    echo json_encode($result);
	    exit;
	}
}
?>

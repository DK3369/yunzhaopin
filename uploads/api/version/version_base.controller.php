<?php

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

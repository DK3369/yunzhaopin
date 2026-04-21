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
class shop_set_controller extends adminCommon{

	function index_action(){
        $sy_imgsc_mr = $this->config['sy_ossurl'].'/'.$this->config['sy_imgsc_mr'];
        $this->render_json(0, 'ok', compact('sy_imgsc_mr'));
	}
	function saveset_action(){
        if($_FILES['file']['tmp_name']){
            $data  =  array(
                'name'      =>  $_POST['name'],
                'path'      =>  $_POST['path'],
                'file'      =>  $_FILES['file']
            );
            $UploadM=$this->MODEL('upload');
            $return = $UploadM->layUpload($data);
            if (!empty($_POST['name']) && $return['code'] == 0){
                // 后台上传logo后，重新生成缓存
                $this->web_config();
            }
            $this->render_json(0, '商品配置设置成功');
        }else{
            $this->render_json(1, '请上传文件');
        }

	}
	//商品类别
	function get_redeem_option_action(){
	    
	    include(PLUS_PATH."redeem.cache.php");
	    $html = '<option value="">请选择</option>';
	    if(!isset($_POST['tnid']) || !isset($redeem_type[$_POST['tnid']])
	        || count($redeem_type[$_POST['tnid']]) < 1){
	            echo $html;
	            exit;
	    }
	    foreach($redeem_type[$_POST['tnid']] as $tnid){
	        $tname = isset($redeem_name[$tnid]) && $redeem_name[$tnid] ? $redeem_name[$tnid] : '';
	        if($tname != ''){
	            $html .= "<option value='{$tnid}'>{$tname}</option>";
	        }
	    }
	    echo $html;
	    exit;
	    
	}
}
?>
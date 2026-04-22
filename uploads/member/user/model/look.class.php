<?php

class look_controller extends user{
    //对我感兴趣列表
	function index_action(){
    
		$LookResumeM       =     $this->MODEL('lookresume');

		include PLUS_PATH."/com.cache.php";

		$uid                    =     $this->uid;

		$where['uid']           =     $uid;

		$where['status']        =     0;

		$where['orderby']       =     array('datetime,desc');

		$urlarr['c']      =     "look";

		$urlarr['page']	  =	    "{{page}}";

		$pageurl					=     Url('member',$urlarr);

		$pageM						=	    $this  -> MODEL('page');

		$pages						=	    $pageM -> pageList('look_resume', $where, $pageurl, $_GET['page']);

		$where['limit']   =     $pages['limit'];

		$looknew          =     $LookResumeM -> getList($where, array('uid'=>$this->uid, 'usertype'=>$this->usertype));

		$look             =     $looknew['list'];
		$this->yunset("js_def",2);
    
		$this->yunset("look",$look);
    
		$this->public_action();
    
		$this->user_tpl('look');
    
	}
	//删除对我感兴趣
	function del_action(){
      
      $lookresumeM    =  $this->MODEL('lookresume');
    
	    if($_GET['id'] || $_GET['del']){
	        if ($_GET['del']){
	            $id   =  $_GET['del'];
	        }elseif ($_GET['id']){
	            $id   =  $_GET['id'];
	        }
	        
	        $return   =  $lookresumeM -> delInfo(array('id'=>$id,'uid'=>$this->uid,'usertype'=>1));
	        
	        $this -> layer_msg($return['msg'],$return['errcode'],$return['layertype'],$_SERVER['HTTP_REFERER']);
	    }
	}
}
?>
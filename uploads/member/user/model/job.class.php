<?php

class job_controller extends user{
	//申请职位列表
	function index_action(){
		$this -> public_action();
		$JobM    					=   $this -> MODEL('job');
		$statisM    				=   $this -> MODEL('statis');
		
		$where['uid']				=  $this -> uid;
		$where['isdel']				=  9;
		if($_GET['browse']){
			
			$where['is_browse']		=	$_GET['browse'];
			$urlarr['browse']		=	$_GET['browse'];
		}
		if($_GET['datetime']){
			if($_GET['datetime']=='1'){
				$where['datetime']	=	array('>',strtotime(date("Y-m-d 00:00:00")));
			}else{
				$where['datetime']	=	array('>',strtotime('-'.intval($_GET['datetime']).' day'));
			}
			$urlarr['datetime']		=	$_GET['datetime'];
		}
		$urlarr['c']	=	$_GET['c'];
		$urlarr['page']	=	'{{page}}';
	    $pageurl		=	Url('member',$urlarr);

	    $pageM			=	$this   ->  MODEL('page');
	    $pages			=	$pageM  ->  pageList('userid_job',$where,$pageurl,$_GET['page']);

        if ($pages['total'] > 0) {
            $where['orderby']   =   'id';
            $where['limit']     =   $pages['limit'];

            $list               =   $JobM->getSqJobList($where, array('uid' => $this->uid, 'usertype' => $this->usertype));
        }

		
        //未查看 已查看 等待通知 条件不符 无法联系
        $StateList		=	array('1'=>'wap_user_00260','3'=>'wap_user_00266','4'=>'wap_user_00354','7'=>'wap_user_00356');
        $this->yunset("StateList",$StateList);
        
        $search_list	=	array('1'=>'common_01940','3'=>'admin_user_00179','7'=>'admin_user_00178','15'=>'admin_user_00180','30'=>'admin_user_00175');
        $this->yunset("search_list",$search_list);
        
        $num=$JobM -> getSqJobNum(array('uid'=>$this->uid,'isdel'=>9));
		
        $statisM -> upInfo(array('sq_jobnum'=>$num),array('uid'=>$this->uid,'usertype'=>$this->usertype));
       
        $this->yunset("total",$pages['total']);
        $this->yunset("rows",$list);
        $this->yunset("js_def",3);
        $this->user_tpl('job');
	}
    //删除申请职位
	function del_action()
    {

        $JobM   =   $this->MODEL('job');
        if ($_GET['del'] || $_GET['id']) {
            if (is_array($_GET['del'])) {
                $id =   $_GET['del'];
            } else {
                $id =   intval($_GET['id']);
            }
            $arr    =   $JobM->delSqJob($id, array('utype' => 'user', 'uid' => $this->uid, 'usertype' => $this->usertype));
            $this->layer_msg($arr['msg'], $arr['errcode'], $arr['layertype'], $_SERVER['HTTP_REFERER']);
        }
	}
	//取消申请职位
	function qs_action()
    {

        $JobM   =   $this->MODEL('job');
        if ($_POST['id']) {

            $id     =   (int)$_POST['id'];
            $return =   $JobM->qxSqJob(array('id' => $id, 'body' => $_POST['body'], 'uid' => $this->uid, 'usertype' => $this->usertype));

            $this->ACT_layer_msg($return['msg'], $return['errcode'], $_SERVER['HTTP_REFERER']);
        }
	}
}
?>
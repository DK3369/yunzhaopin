<?php

class privacy_controller extends user_controller{
    // 
	function privacy_action(){
		
		$ResumeM	=	$this -> MODEL('resume');
		$resume		=	$ResumeM -> getResumeInfo(array('uid'=>$this->member['uid']),array('field'=>'`status`'));
		$status		=	$resume['status'];

		$this->render_json(0,'',$status);
	}

	// 
	function blacklist_action(){ 
		$blackM						=	$this->MODEL('black');
		
		$where['c_uid']				=	$this->member['uid'];
		$where['usertype']			=	'1';
		$total = $blackM->getBlackNum($where);
		$page		=	$_POST['page'];
		$limit		=	$_POST['limit'];
		$limit		=	!$limit?20:$limit;
	
			
		$where['orderby']	=	'id,desc';
        if($page){
            $pagenav		=	($page-1)*$limit;
            $where['limit']	=	array($pagenav,$limit);
        }else{
            $where['limit']	=	array('',$limit);
        }
		$rows = $blackM->getBlackList($where);
			
		if($rows && is_array($rows)){
			$list	=	count($rows)?$rows:array();
			
			$this->render_json(1,'ok',$list,$total);
		}else{
			$this->render_json(2,'','');
		}
		
	}
    // 
    function up_action()
    {

        $resumeM    =   $this->MODEL('resume');
        $return     =   $resumeM->upResumeInfo(array('uid' => $this->member['uid']), array('rData' => array('status' => intval($_POST['status']))));

        $resumeM->upInfo(array('uid' => $this->member['uid']), array('eData' => array('status' => intval($_POST['status']))));

        $logM       =   $this->MODEL('log');

        $logContent =   yun_at('wap_01806');

        $status     =   $resumeM->getResumeInfo(array('uid' => $this->member['uid']), array('field' => '`status`'));

        if (intval($_POST['status']) == 2) {
            $stext  =   yun_at('admin_user_00259');
        } else if (intval($_POST['status']) == 1) {
            $stext  =   yun_at('wap_js_00005');
        } else if (intval($_GET['status']) == 3) {
            $stext  =   yun_at('member_user_00256');
        }

        $logDetail  =   yun_at('wap_01807').$stext;

        $logM->addMemberLog($this->member['uid'], $this->member['usertype'], $logContent, 2, 2, $logDetail);

        $data['error']  =   $return['errcode'] == 9 ? 1 : 2;
        $data['msg']    =   $return['msg'];

        $this->render_json($data['error'], $data['msg'], $status);
    }

    // 
	function del_action(){
        $blackM		=	$this->MODEL('black');
        $id			=	(int)$_POST['id'];

        $return		=	$blackM->delBlackList($id,array('where'=>array('c_uid'=>$this->member['uid'])));

        if($return['errcode']==9){
            $error	=	1;
        }else{
            $error	=	2;
        }
        $this-> render_json($error,$return['msg'],$return);
    }
    // 
	function delall_action(){
		$blackM		=	$this->MODEL('black');
		
		$return		=	$blackM->delBlackList('',array('uid'=>$this->member['uid'],'usertype'=>$this->member['usertype'],'where'=>array('c_uid'=>$this->member['uid']),'type'=>'all'));
		
		if($return['errcode']==9){
				$error	=	1;
			}else{
				$error	=	2;	
 			}
		$this-> render_json($error,$return['msg'],$return);
	}
    // 
	function searchcom_action(){
		$blackM			=	$this->MODEL('black');
		$companyM		=	$this->MODEL('company');
		$keyword  		=	trim($_POST['keyword']);
		if($keyword!=''){
			$blacklist		=	$blackM->getBlackList(array('c_uid'=>$this->member['uid']),array('field'=>'`p_uid`'));
			if($blacklist && is_array($blacklist)){
				$uids			=	array();
				foreach($blacklist as $v){
					
					if($v['p_uid'] && !in_array($v['p_uid'],$uids)){
						
						$uids[]	=	$v['p_uid'];
					}
				}
				$where['uid']	=	array('notin',pylode(',',$uids));
			}
			$where['name']		=	array('like',$keyword);
			$where['limit']     =   30;
			$company			=	$companyM->getList($where,array('field'=>'`uid`,`name`'));
			$company			=	$company['list'];
		}
		
		
		 if($company && is_array($company)){
			
		  	foreach($company as $val){
		  		 $return[] = $val;
		  	}
		  	$this->render_json(1,'ok',$return);
		  }else{

		  	$this->render_json(2,'','');
				
		  }
	}
    // 
	function save_action(){
		
		$blackM		=  $this->MODEL('black');
		$data		=	array(
			'cuid'		=>	$_POST['p_uid'],
			'uid'		=>	$this->member['uid'],
			'usertype'	=>	1
		);
		$return		=  $blackM -> addBlacklist($data);
		
		if($return['errcode']==9){
			$error =1;
		}else{
			$error=2;
		}
		$this ->render_json($error,$return['msg']);
	}
}
?>

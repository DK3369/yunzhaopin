<?php

class atn_controller extends user_controller{
    //关注的企业列表
	function atncom_action()
	{
	    $atnM		=	$this->MODEL('atn');
		$page		=	$_POST['page'];
		$limit		=	$_POST['limit'];
		$limit		=	!$limit?20:$limit;
        
		$where['uid']				=	$this->member['uid'];
		if($_POST['utype']=='company') {
            $where['sc_usertype'] = '2';
        }

		$total = $atnM->getatnNum($where);
		$where['orderby']			=	'id,desc';
        if($page){
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	array('',$limit);
		}
	    $List				=	$atnM->getatnList($where,array('utype'=>$_POST['utype'],'wap'=>$_POST['wap']));
		if(!empty($List)){
			$list			=	count($List)?$List:array();
			$error			=	1;
		}else{
			$error			=	2;
		}
		$this->render_json($error,'',$list,$total);
	}
	//保存关注企业
	function atncompany_action()
	{
	    $id				=	(int)$_POST['id'];
	    $userinfoM		=	$this->MODEL('userinfo');

	    $data			=	array(
			'id'			=>	$id,
	        'uid'			=>	$this->member['uid'],
			'usertype'		=>	'1',
			'sc_usertype'	=>	'2',
			'utype'			=>	'teacher',
	        'username'		=>	$this->member['username']
		);
		$atnM			=	$this->MODEL('atn');
		$return			=	$atnM->addAtnLt($data);
		$data['error']	=	$return['errcode'];
		$data['msg']	=	$return['msg'];
		$this->render_json($data['error'],$data['msg']);
	    
	}
	//取消关注企业
	function delatncom_action()
	{
	    $utype          =   isset($_POST['utype']) ? $_POST['utype'] : '';
	    $cuid			=	intval($_POST['cuid']);
	    $id 			=	intval($_POST['id']);
	    $atnM			=	$this->MODEL('atn');
        
        $sc_usertype=   2;
        
        $return         =	$atnM->delAtnAll($id,array('sc_uid'=>$cuid,'sc_usertype'=>$sc_usertype,'uid'=>$this->member['uid'],'usertype'=>'1'));
	    $data['error']	=	$return['errcode'];
	    $data['msg']	=	$return['msg'];
		$this->render_json($data['error'],$data['msg'],'');
	}
	
}
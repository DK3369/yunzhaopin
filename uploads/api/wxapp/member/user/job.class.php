<?php

class job_controller extends user_controller{
    
/* wxapp: applications */
	function sqjoblist_action()
	{
		$JobM				=	$this -> MODEL('job');
		$statisM			=	$this -> MODEL('statis');
		
		$page				=	$_POST['page'];
		$limit				=	$_POST['limit'];
		$limit				=	!$limit?20:$limit;
		
		$where['uid']		=	$this->member['uid'];
		$where['isdel']		=	9;
		if (!empty($_POST['browse'])){

            $where['is_browse']		=	$_POST['browse'];
        }
        $total				=	$JobM -> getSqJobNum($where);
		$where['orderby']	=	'id,desc';
		
		if($page){
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	array('',$limit);
		}
	
		$rows   			=	$JobM -> getSqJobList($where,array('uid'=>$this->member['uid'],'usertype'=>'1'));
		
		$num				=	$JobM -> getSqJobNum(array('uid'=>$this->member['uid'],'isdel'=>9));
	
		$statisM 			->	upInfo(array('sq_jobnum'=>$num),array('uid'=>$this->member['uid'],'usertype'=>'1'));
		
		if(!empty($rows)){
			$list			=	count($rows) ? $rows : array();
			$error			=	1;
		}else{
			$error			=	2;
		}
		$this->render_json($error,'',$list,$total);
	}
	/* wxapp: delete application */
	function delsqjob_action()
	{
		$JobM	=	$this -> MODEL('job');
		$id		=	intval($_POST['ids']);
		$arr	=	$JobM -> delSqJob($id,array('utype'=>'user','uid'=>$this->member['uid'],'usertype'=>'1'));
		$error	=	$arr['errcode']==9 ? 1 : 2;
		$this->render_json($error,$arr['msg']);
	}

    /* wxapp: cancel application */
    function qxSqJob_action()
    {
        $JobM	=	$this -> MODEL('job');
        $id		=	intval($_POST['ids']);
        $arr	=	 $JobM -> qxSqJob(array('id'=>$id,'body'=>$_POST['body'],'uid'=>$this->member['uid'],'usertype'=>'1'));
        $error	=	$arr['errcode']==9 ? 1 : 2;
        $this->render_json($error,$arr['msg']);
    }
	
	/* wxapp: favorites */
	function favlist_action()
	{	
		$JobM				=	$this -> MODEL('job');
		$statisM			=	$this -> MODEL('statis');
		
		$page				=	$_POST['page'];
		$limit				=	$_POST['limit'];
		$limit				=	!$limit?20:$limit;
		
		$where['uid']		=	$this->member['uid'];
		
		$where['orderby']	=	array('id,desc');
		$total = $JobM -> getFavJobNum($where);
		if($page){
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	array('',$limit);
		}
		
		$rows				=	$JobM -> getFavJobList($where,array('datatype'=>'moreinfo'));

		$num				=	$JobM -> getFavJobNum(array('uid'=>$this->member['uid'],'type'=>1));

		$statisM	->	upInfo(array('fav_jobnum'=>$num),array('uid'=>$this->member['uid'],'usertype'=>'1'));
		
		
		$list			=	count($rows) ? $rows : array();
		$this->render_json(0,'',$list,$total);
	}
	
	/* wxapp: delete favorite */
	function delfavjob_action()
	{
		$id		 		=	$_POST['ids'];
            
        $jobM    		=  $this->MODEL('job');
        
        $return	 		=	$jobM -> delFavJob($id,array('utype'=>'user','uid'=>$this->member['uid'] ,'usertype'=>'1'));
        $data['error']	=	$return['errcode']==9 ? 1 : 2;
        $data['msg']	=	$return['msg'];
		$this->render_json($data['error'],$return['msg'],'');
	}
	/* wxapp: interview notices */
	function invitelist_action()
	{
        $JobM				=	$this -> MODEL('job');
	    $where['uid']		=	$this->member['uid'];
		$where['type']		=	array('<>',1);
		$where['isdel']		=	9;

		if(!empty($_POST['status'])){
            $where['is_browse'] = $_POST['status'];
        }
		$total = $JobM->getInviteNum($where);
		$page				=	$_POST['page'];
		$limit				=	$_POST['limit'];
		$limit				=	!$limit?20:$limit;
		
		$where['orderby']	=	'id,desc';
		
		if($page){
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	array('',$limit);
		}
		

		$list				=	$JobM -> getYqmsList($where);
		$data['list']		=	count($list) ? $list:array();
        $data['total'] = $total;
		$this->render_json(1,'',$data);
	}
	
	/* wxapp: interview invite detail */
	function inviteshow_action()
	{
		$id	   =  (int)$_POST['id'];
		$JobM  =  $this -> MODEL('job');
		
		$info  =  $JobM -> getYqmsInfo(array('id'=>$id,'uid'=>$this->member['uid']),array('yqh'=>1,'uid'=>$this->member['uid'],'usertype'=>'1'));
		$job   =  $JobM->getInfo(array('id'=>$info['jobid']),array('field'=>'name,minsalary,maxsalary'));
		
		$info['jobname'] = $job['name'];
		$info['salary']  = $job['job_salary'];
		$info['over']  = time() > strtotime($info['intertime']) ? 1 : 0;
		

        $data['list']		=	$info;
		$this->render_json(1,'',$data['list']);
	}
	/* wxapp: delete interview notice */
	function invitedel_action()
	{
		$id				=	(int)$_POST['id'];
		if ($_POST['type'] == 'black'){
		    $blackM 		=   $this -> MODEL('black');
		    $bdata   		=   array('id' => $id, 'uid' => $this->member['uid'], 'usertype' =>1, 'type' => 'yqms');
		    $return 		=   $blackM->addBlacklist($bdata);
		}elseif ($_POST['type'] == 'del'){
		    $JobM		=  $this->MODEL('job');
		    $return		=  $JobM -> delYqms($id,array('uid'=>$this->member['uid'],'usertype'=>1));
		}
		$error	=	$return['errcode']==9 ? 1 : 2;
		$this->render_json($error,$return['msg']);
	}
	
    function inviteset_action()
    {
        $id         =   (int)$_POST['id'];
        $browse     =   (int)$_POST['browse'];
        $username   =   $this->member['username'];

        $JobM       =   $this->MODEL('job');

        $remark     =   '';
        if($browse==4 && $_POST['remark']){
            $remark =   trim($_POST['remark']);
        }

        $return     =   $JobM->setYqms(array('id' => $id, 'browse' => $browse, 'remark'=>$remark, 'uid' => $this->member['uid'], 'username' => $username, 'port' => $this->port));

        $data['error']  =   $return['errcode'] == 9 ? 1 : 2;
        $this->render_json($data['error'], $return['msg'], '');
    }

	/* wxapp: browse history */
	function look_job_action()
	{
		$JobM				=   $this -> MODEL('job');
		$page				=	$_POST['page'];
		$limit				=	$_POST['limit'];
		$limit				=	!$limit?20:$limit;

		$where['uid']		=   $this->member['uid'];
		$where['status']	=   0;
		$total = $JobM->getLookJobNum($where);
        $where['orderby']	=	array('id,desc');
		
		if($page){
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	array('',$limit);
		}
        
        $rows    			=   $JobM -> getLookJobList($where);
		if(is_array($rows)&&!empty($rows)){
			$data			=	count($rows)?$rows:array();
			$error			=	1;
		}else{
			$error			=	2;
		}
		$this -> render_json($error,'',$data,$total);
	}
	/* wxapp: delete browse history */
	function look_job_del_action()
	{
		$JobM   		=   $this -> MODEL('job');
		$id 			=	(int)$_POST['ids'];
		$return 		=   $JobM -> delLookJob($id,array('uid'=>$this->member['uid'],'usertype'=>'1'));
		$data['error']	=	$return['errcode']==9 ? 1 : 2;
		$data['msg']	=	$return['msg'];
		$this -> render_json($data['error'],$return['msg'],'');
	}
	/* wxapp: similar jobs */
	function like_job_action()
	{
		$data			=	array(
			'id'		=>	(int)$_POST['id'],
		    'uid'		=>	$this->member['uid']
		);
		$resumeM		=	$this->MODEL('resume');
		$list			=	$resumeM->likeJob($data);

		$data			=	count($list) ? $list : array();
		$this -> render_json(1,'',$data);
	}
	
	function jobcolumn_action()
	{
	    
	    $JobM			=	$this -> MODEL('job');
		$AtnM			=	$this -> MODEL('atn');
		$LookresumeM	=	$this -> MODEL('lookresume');
		$PartM	=	$this -> MODEL('part');
		// 
		$invitenum		=	$JobM -> getYqmsNum(array('uid'=>$this->member['uid'],'isdel'=>9));
		$show['invitenum']	=	$invitenum?$invitenum:0;
		// 
		$sqnum			=	$JobM -> getSqJobNum(array('uid'=>$this->member['uid'],'isdel'=>9));
		$show['sqnum']	=	$sqnum?$sqnum:0; 
		// 
		$collectnum		=	$JobM -> getFavJobNum(array('uid'=>$this->member['uid'],'type'=>1));
		$show['collectnum']			=	$collectnum?$collectnum:0;
		$where['uid']				=	$this->member['uid'];
		$where['sc_usertype']		=	'2';
		
		$atncomnum		=	$AtnM -> getAtnNum($where);
		$atnnum			=	$atncomnum;
		$show['atnnum']	=	$atnnum?$atnnum:0;
		
		// 
		$lookjobnum		=	$JobM -> getLookJobNum(array('uid'=>$this->member['uid'],'status'=>'0'));
		$show['lookjobnum']	=	$lookjobnum?$lookjobnum:0;
		
		// 
		$looknum		=	$LookresumeM -> getLookNum(array('uid'=>$this->member['uid'],'usertype'=>'2','status'=>'0'));
		$show['looknum']=	$looknum?$looknum:0;
		
		$wkyqnum		=	$JobM -> getYqmsNum(array('uid'=>$this->member['uid'],'isdel'=>9,'is_browse'=>'1'));
		
		$show['wkyqnum']=	$wkyqnum?$wkyqnum:0;
		$wlooknum		=	$JobM -> getLookJobNum(array('uid'=>$this->member['uid'],'status'=>'0','datetime'=>array('<',time())));
		$show['wlooknum']	=	$wlooknum?$wlooknum:0;
		// 
		$partapplynum		=	$PartM->getPartSqNum(array('uid'=>$this->member['uid']));
		$partcollectnum		=	$PartM->getPartcollectNum(array('uid'=>$this->member['uid']));
		$allpartnum			=	$partapplynum + $partcollectnum;
		$show['allpartnum']	=	$allpartnum?$allpartnum:0;
		$show['iosfk']		=	$this->config['sy_iospay'] ;
		$this->render_json(0,'',$show);
	}
}
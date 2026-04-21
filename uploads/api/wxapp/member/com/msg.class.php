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
class msg_controller extends com_controller{
    
	/* 求职咨询 */
	function msglist_action(){
        $MsgM			   =  $this -> MODEL('msg');
		$where['job_uid']  =  $this->member['uid'];
		$where['del_status']	=	0;
		$where['status']		=	1;
        $total = $MsgM->getMsgNum($where);
		$page              =  $_POST['page'];
		
    	if ($_POST['limit']){

			 $limit				 =  $_POST['limit'];
			 
			if($page){//分页
				
				$pagenav		 =  ($page-1)*$limit;
				
				$where['limit']  =  array($pagenav,$limit);
				
      		}else{

				$where['limit']  =  $limit;
				
			}    
			 
    	}

		$where['orderby']  =  array('reply_time,asc','id,desc');


		
		$rows			   =  $MsgM -> getList($where);

		$rows 			   =  $rows['list'];
		
		$data['list']	   =  count($rows)?$rows:array();

  		$this->render_json(0,'',$data,$total);
	}
  	/**
	 * 企业咨询消息删除
	 */
	function delsmsglist_action(){

        if(!$_POST['id']){

            $data['error']  =   3;
            $data['errmsg'] =   '参数不正确';
        }else{

            $MsgM   =   $this->MODEL('msg');

            $return =   $MsgM->delMsg($_POST['id'], array('job_uid' => $this->member['uid']));
            if ($return['errcode'] == 9) {

                $LogM       =   $this->MODEL('log');
                $logContent =   '消息处理：删除求职咨询（ID：'.$_POST['id'].'）';
                $LogM->addMemberLog($this->member['uid'], $this->member['usertype'], $logContent, 18, 3);

                $data['error']  =   0;
                $data['errmsg'] =   '删除成功！';
            } else {

                $data['error']  =   1;
                $data['errmsg'] =   '删除失败！';
            }
        }
        $this->render_json($data['error'],$data['errmsg']);

	}
  	function savereply_action(){

        $MsgM   =   $this->MODEL('msg');
        if (!$_POST['id']) {

            $data['error']  =   3;
            $data['errmsg'] =   '参数不正确';
        } else {


            $data['reply'] = $_POST['reply'];
            $data['reply_time'] = time();
            $data['user_remind_status'] = '0';

            $where['id'] = (int)$_POST['id'];
            $where['job_uid'] = $this->member['uid'];
            $nid    =   $MsgM->upReplyInfo($where, $data);

            if ($nid) {

                $LogM       =   $this->MODEL('log');
                $logContent =   '消息处理：回复求职咨询';
                $logDetail  =   '回复求职咨询：'.$_POST['reply'];
                $LogM->addMemberLog($this->member['uid'], $this->member['usertype'], $logContent, 18, 2, $logDetail);

                $data['error']  =    1;
                $data['errmsg'] =   '回复成功';
            } else {
                $data['error']  =   2;
                $data['errmsg'] =   '回复失败';
            }
        }
        $this->render_json($data['error'], $data['errmsg']);
  	}
	//消息
	function sysnews_action(){
    
	    // 聊天

		$JobM		  =  $this -> MODEL('job');
		// 对我感兴趣（查看职位）
		$looknum        =   $JobM->getLookJobNum(array('com_id'=>$this->member['uid'],'com_status'=>0));
		$newlook        =   $JobM->getLookJobInfo(array('com_id'=>$this->member['uid'],'com_status'=>0,'orderby'=>'datetime'), array('utype'=>'user'));
		$list['looknum']=	$looknum;
		$list['newlook']=	!empty($newlook) ? $newlook : array();
   		//申请职位
    	$userid_jobnum	=	$JobM -> getSqJobNum(array('com_id'=>$this->member['uid'],'isdel'=>9,'is_browse'=>'1','type'=>array('<>',3)));
 		//系统消息
    	$SysmsgM		=	$this -> MODEL('sysmsg');
 		$sxnum			=	$SysmsgM -> getSysmsgNum(array('fa_uid'=>$this->member['uid'],'usertype'=>$this->member['usertype'],'remind_status'=>'0'));
    	$list['sxnum']	=	$sxnum;
		$list['userid_jobnum']	=	$userid_jobnum;
   	 	//求职者咨询
		$jobnum = 0;
		if ($this->config['com_message'] == 1){
		    
		    $qzwhere['job_uid'] = $this->member['uid'];
		    $qzwhere['del_status']	=	0;
			$qzwhere['status']		=	1;
		    $qzwhere['PHPYUNBTWSTART'] = '';
		    $qzwhere['reply'][]  =  array('isnull');
		    $qzwhere['reply'][]  =  array('=','','OR');
		    $qzwhere['PHPYUNBTWEND'] = '';
		    
		    $MsgM	= $this -> MODEL('msg');
		    $jobnum = $MsgM->getMsgNum($qzwhere);
		}
		$list['jobnum']      =  $jobnum;
		$list['com_message'] = !empty($this->config['com_message']) ? $this->config['com_message'] : 0;
		$list['sysnum']  	 =  $sxnum + $userid_jobnum + $jobnum;
        $list['gzhurl'] = Url('wap', array('c'=>'ajax','a'=>'gzhqrcode','token'=>$this->member['gzhtoken']));
        if ($this->member['subscribe'] != 1 && !empty($this->member['wxid'])){
            $wxM    =   $this->MODEL('weixin');
            $wxUser =   $wxM->getWxUser($this->member['wxid']);
            $this->obj->update_once('member', array('subscribe' => $wxUser['subscribe']), array('uid' => $this->member['uid']));
            $this->member['subscribe']  =   $wxUser['subscribe'];
        }
        $list['subscribe']  =   $this->member['subscribe'];

   		$this->render_json(0,'',$list); 
	}
	
	/**
	 *系统消息查询
	*/
	function sysmsgnews_action(){
		$SysmsgM	=	$this -> MODEL('sysmsg');
    	//存在执行这段代码
		$msgwhere['fa_uid']     =  $this->member['uid'];
		$msgwhere['usertype']   =  $this->member['usertype'];
		$msgwhere['remind_status']= array('<>',1);
		$msginfo              	=     $SysmsgM->getSysmsgInfo($msgwhere,array('field'=>'`id`'));
    	if($msginfo){
      		$data                =   array(
          		'remind_status'   =>  1
      		);
     		$SysmsgM->upSysmsg($msgwhere,$data);
    	}    
 
		$where['fa_uid']		 =  $this->member['uid'];
		$where['usertype']		 =  $this->member['usertype'];
		$total = $SysmsgM->getSysmsgNum($where);
    	$page					 =  $_POST['page'];
      	if ($_POST['limit']){
        	$limit				 =  $_POST['limit'];
        	if($page){//分页
           		$pagenav		 =  ($page-1)*$limit;
            	$where['limit']  =  array($pagenav,$limit);
        	}else{
          		$where['limit']  =  $limit;
        	}
     	}
		$where['orderby']  =  'id';
		
		$rows  =  $SysmsgM -> getList($where, array('type'=>$_POST['type']));
		
		$data['list']  =  count($rows)?$rows:array();
		
    	$this->render_json(0,'',$data,$total);
  	}
	/**
	 * 系统消息删除
	 */
	function delsysmsgnews_action(){
        $SysmsgM	=	$this -> MODEL('sysmsg');
        if(!$_POST['id']){
            $data['error']	=	3;
            $data['errmsg']	=	'参数不正确';
        }else{
            $return  =	 $SysmsgM -> delSysmsg($_POST['id'],array('fa_uid'=>$this->member['uid']));
            if($return['errcode']==9){

                $LogM       =   $this -> MODEL('log');
                $logContent =   '消息处理：删除系统消息（ID：'.$_POST['id'].'）';
                $LogM->addMemberLog($this->member['uid'], $this->member['usertype'], $logContent, 18, 3);

                $data['error']	=	0;
                $data['errmsg']	=	'删除成功！';
            }else{
                $data['error']	=	1;
                $data['errmsg']	=	'删除失败！';
            }
        }
        $this->render_json($data['error'],$data['errmsg']);
	}
  
}
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
class msg_controller extends user_controller{
    /**
     * 消息
     */
	function sysnews_action()
	{

	    //面试通知
	    $JobM				=	$this		-> MODEL('job');
		$wkyqnum			=	$JobM		-> getYqmsNum(array('uid'=>$this->member['uid'],'isdel'=>9,'is_browse'=>'1'));
		$list['wkyqnum']	=	$wkyqnum;
		//私信
		$SysmsgM			=	$this 		-> MODEL('sysmsg');
		$sxnum			    =	$SysmsgM	-> getSysmsgNum(array('fa_uid'=>$this->member['uid'],'usertype'=>'1','remind_status'=>'0'));
		$list['sxnum']	    =	$sxnum;
		//职位咨询回复
		$MsgM		=	$this -> MODEL('msg');
		$commsgnum	=	$MsgM -> getMsgNum(array('uid'=>$this->member['uid'],'reply'=>array('<>',''),'user_remind_status'=>'0'));
		$list['commsgnum']  =	$commsgnum;
		$list['com_message'] = !empty($this->config['com_message']) ? $this->config['com_message'] : 0;
		$list['sysnum']  =  $wkyqnum + $sxnum + $commsgnum;
        $list['gzhurl'] = Url('wap', array('c'=>'ajax','a'=>'gzhqrcode','token'=>$this->member['gzhtoken']));

        if ($this->member['subscribe'] != 1 && !empty($this->member['wxid'])){
            $wxM    =   $this->MODEL('weixin');
            $wxUser =   $wxM->getWxUser($this->member['wxid']);
            $this->obj->update_once('member', array('subscribe' => $wxUser['subscribe']), array('uid' => $this->member['uid']));
            $this->member['subscribe']  =   $wxUser['subscribe'];
        }
        $list['subscribe']  =   $this->member['subscribe'];
		$this->render_json(0,'ok',$list);
	}
	//系统消息
	function sxnews_action()
	{
	    $SysmsgM			=	$this -> MODEL('sysmsg');
		$SysmsgM -> upInfo(array('fa_uid'=>$this->member['uid'],'usertype'=>'1','remind_status'=>'0'),array('remind_status'=>'1'));

        $where['fa_uid']	=	$this->member['uid'];
        $total = $SysmsgM->getSysmsgNum($where);
		$page				=	$_POST['page'];
		$limit				=	$_POST['limit'];
		$limit				=	!$limit?20:$limit;
		

		$where['usertype']	=	'1';
		
        $where['orderby']	=	'id';
        if($page){
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	array('',$limit);
		}
		$rows				=	$SysmsgM -> getList($where, array('type'=>$_POST['type']));
		if(!empty($rows)){
			$list			=	count($rows) ? $rows : array();
			$error			=	1;
		}else{
			$error			=	2;
		}
		$this->render_json($error,'',$list,$total);
	}
	/**
	 * 删除系统消息
	 */
	function delsxnews_action()
	{

        $SysmsgM    =   $this->MODEL('sysmsg');
        $return     =   $SysmsgM->delSysmsg((int)$_POST['id'], array('fa_uid' => $this->member['uid']));

        $LogM       =   $this->MODEL('log');
        $logContent =   '消息处理：删除系统消息（ID：'.$_POST['id'].'）';
        $LogM->addMemberLog($this->member['uid'], $this->member['usertype'], $logContent, 18, 3);

        $data['error']  =   $return['errcode'] == 9 ? 1 : 2;
        $this->render_json($data['error'], $return['msg'], '');
	}
	//职位咨询消息
	function zxmsg_action(){
		$msgM	=	$this -> MODEL('msg');
		$msgM -> upInfo(array('uid'=>$this->member['uid']),array('user_remind_status'=>1,'usertype'=>$this->member['usertype']));


		$where['uid']	=	$this->member['uid'];
        $total = $msgM->getMsgNum($where);
		$page			=	$_POST['page'];
		$limit			=	$_POST['limit']?$_POST['limit']:10;
		$where['orderby']		=	'id';
		if($page){
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	array('',$limit);
		}
		
		$rows	=	$msgM  ->  getList($where);
		if(!empty($rows)){
			$list			=	count($rows['list']) ? $rows['list'] : array();
			$error			=	1;
		}else{
			$error			=	2;
		}
		$this->render_json($error,'',$list,$total);
	}
    //删除职位咨询消息
	function delzxmsg_action(){
	    $msgM		=  $this->MODEL('msg');
	    $return		=  $msgM -> delInfo($_POST['id'], ['uid' => $this->member['uid']]);
		$data['error']	=	$return['errcode']==9 ? 1 : 2;
	    $this->render_json($data['error'],$return['msg'],'');
	}
}
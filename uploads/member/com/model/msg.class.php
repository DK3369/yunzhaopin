<?php

class msg_controller extends company{	
	
	function index_action(){
		
		$MsgM	=	$this -> MODEL('msg');
		
		$where['job_uid']		=	$this -> uid;
		
		$where['status']		=	1;

		$where['del_status']	=	0;
		
		$urlarr		=	array("c" => "msg","page" => "{{page}}");
		
		$pageurl	=	Url('member',$urlarr);
		
		$pageM		=	$this -> MODEL('page');
		
		$pages		=	$pageM -> pageList('msg',$where,$pageurl,$_GET['page'],$this->config['sy_listnum']);
		
		if($pages['total'] > 0){
			
			
			$where['orderby']	=	'datetime';
			
			
			$where['limit']		=	$pages['limit'];
			
			
			$rows	=	$MsgM -> getList($where);
			
		}
		
		$this -> yunset("rows",$rows['list']);
		
		$this -> public_action();
		
		$this -> company_satic();
		
		$this -> com_tpl('msg');
	
	}
	
	function del_action()
    {
        $MsgM   =   $this->MODEL('msg');

        if ($_GET['id']) {

            $where['id']        =   (int)$_GET['id'];
            $where['job_uid']   =   $this->uid;

            $nid    =   $MsgM->upInfo($where);

            if ($nid) {
                $logM       =   $this->MODEL('log');
                $logContent =   'api_wxapp_00001'.$_GET['id'].'）';
                $logM->addMemberLog($this->uid, 2, $logContent, 18, 3);

                $this->layer_msg('admin_user_00187', 9, 0, "index.php?c=msg");
            } else {

                $this->layer_msg('admin_user_00186', 8, 0, "index.php?c=msg");
            }
        }
	}
	
	function save_action()
    {
        $MsgM	=	$this -> MODEL('msg');

        if($_POST['submit']){

            $data['reply']              =   $_POST['reply'];
            $data['reply_time']         =   time();
            $data['user_remind_status'] =   0;

            $id =   $MsgM->upReplyInfo(array('id' => $_POST['id'], 'job_uid' => $this->uid), $data);

            if($id){

                $logM       =   $this->MODEL('log');
                $logContent =   'api_wxapp_00003';
                $logDetail  =   'member_com_00698'.$_POST['reply'];

                $logM->addMemberLog($this->uid, 2, $logContent, 18, 2, $logDetail);

                $this->ACT_layer_msg('member_com_00699',9,"index.php?c=msg");
            }else{

                $this->ACT_layer_msg('admin_system_00137',8,"index.php?c=msg");
            }
        }
	}
	
	function getContent_action(){
	    
	    $MsgM  =  $this -> MODEL('msg');
	    
	    $msg   =  $MsgM->getInfo(array('id'=>$_POST['id'],'job_uid'=>$this -> uid),array('field'=>'`content`'));
	    
	    echo yun_json_encode($msg);
	}
}
?>
<?php

class special_controller extends company{
	//招聘会
	function index_action(){
		$this->company_satic();
		$this->public_action();
		$specialM	=	$this->MODEL('special');
		$urlarr["c"]	=	"special";
		$urlarr["page"]	=	"{{page}}";
		$pageurl		=	Url('member',$urlarr);
		$where['uid']	=	$this->uid;
		
		$pageM		=	$this  -> MODEL('page');
		$pages		=	$pageM -> pageList('special_com',$where,$pageurl,$_GET['page'],$this->config['sy_listnum']);
		
		if($pages['total'] > 0){
			if($_GET['order'])
			{
				$where['orderby']		=	$_GET['t'].','.$_GET['order'];
				$urlarr['order']		=	$_GET['order'];
				$urlarr['t']			=	$_GET['t'];
			}else{
				$where['orderby']		=	'id';
			}
			$where['limit']	=	$pages['limit'];
			
			$List	=	$specialM -> getSpecialComList($where, array('utype'=>'user'));
			
			$this->yunset("rows" , $List['list']);
		}
		$this->com_tpl("special");
	}
    function del_action()
    {

        $specialM   =   $this->MODEL('special');
        $delRes     =   $specialM->delSpecialCom(array('id' => (int)$_GET['id'], 'uid' => $this->uid), " ");

        if ($delRes) {

            $logM = $this->MODEL('log');
            $logContent =   'common_00694';
            $logM->addMemberLog($this->uid, $this->usertype, $logContent, 14, 3);

            $this->layer_msg('admin_user_00187', 9, 0, $_SERVER['HTTP_REFERER']);
        } else {

            $this->layer_msg('admin_user_00186', 8, 0, $_SERVER['HTTP_REFERER']);
        }
    }
}
?>
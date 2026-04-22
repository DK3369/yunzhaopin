<?php

class resume_controller extends common	
{
	
	public $userInfo	=	array();

    function __construct($tpl, $db, $def = '', $model = 'index', $m = '')
    {
        $this->common($tpl, $db, $def, $model, $m);
        
        if ($this->usertype == '2') {
	
			$userInfoM	=	$this->MODEL('userinfo');

			$this->userInfo	=	$userInfoM -> getUserInfo(array('uid' => $this->uid), array('usertype' => $this->usertype));

			$this->yunset('userInfo', $this->userInfo);
        }
    }

}
?>
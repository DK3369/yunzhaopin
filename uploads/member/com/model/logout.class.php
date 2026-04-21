<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
class logout_controller extends company
{
    function index_action()
    {

        $logoutM    =  $this->MODEL('logout');

        $row        =  $logoutM->getInfo(array('uid'=>$this->uid));
        $step       =   1;
        if (!empty($row)){
            $step   =   3;
        }

        $userM = $this -> MODEL('userinfo');
        $tel  = $userM -> getInfo(array('uid' => $this->uid));

        $this->yunset('tel', $tel);

        $this->yunset('step', $step);

        $this->public_action();

        $this->com_tpl('logout');
    }

    function logoutApply_action()
    {

        $_POST      =  $this->post_trim($_POST);
        $p          =  array(
            'password'  =>  $_POST['password']
        );

        $logoutM    =  $this->MODEL('logout');
        $return     =  $logoutM->apply(array('uid'=>$this->uid), $p);

        echo json_encode($return);
        die();

    }
    function logoutmsg_action()
    {

        $_POST      =  $this->post_trim($_POST);
        $p          =  array(
            'mobile'  =>  $_POST['mobile'],
            'code'    =>  $_POST['code'],
        );

        $logoutM    =  $this->MODEL('logout');
        $return     =  $logoutM->applymsg(array('uid'=>$this->uid), $p);

        echo json_encode($return);
        die();

    }

}

?>
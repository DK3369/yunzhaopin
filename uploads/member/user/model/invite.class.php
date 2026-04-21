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
class invite_controller extends user{

    // 面试通知列表
    function index_action(){
       $JobM       =   $this -> MODEL('job');
        $resume = $this -> public_action();
        
        if($resume['sex']==1){
            
            $name  =   mb_substr($resume['name'],0,1,'utf-8').'先生';
        }elseif($resume['sex']==2){
            
            $name  =   mb_substr($resume['name'],0,1,'utf-8').'女士';
        }else{
            
            $name  =   $resume['name'];
        }
        $this->yunset("realmsgname",$name);
        
        $where  =   $urlarr =   array();
        
        $where['uid']   =   $this->uid;
        $where['type']  =   array('<>', 1);
        $where['isdel'] =   9;
        $urlarr['c']    =   $_GET['c'];
        $urlarr['page'] =   '{{page}}';
        $pageurl        =   Url('member', $urlarr);

        $pageM          =   $this -> MODEL('page');
        $pages          =   $pageM -> pageList('userid_msg', $where, $pageurl, $_GET['page']);

        if ($pages['total'] > 0) {
            $where['orderby']   =   'id';
            $where['limit']     =   $pages['limit'];

           
            $list       =   $JobM -> getYqmsList($where);
        }
        $time           =   time();
        $this -> yunset('time', $time);
        $this -> yunset('rows', $list);
        $this -> yunset('js_def', 4);
        $this -> user_tpl('invite');
    }

    /**
     * @desc 面试通知 - 屏蔽企业操作
     */
    function shield_action()
    {
        $blackM =   $this -> MODEL('black');
        $id     =   intval($_GET['id']);
        $data   =   array('id' => $id, 'uid' => $this->uid, 'usertype' => $this->usertype, 'type' => 'yqms');
        $return =   $blackM->addBlacklist($data);

        $this -> layer_msg($return['msg'], $return['errcode'], 0, $_SERVER['HTTP_REFERER']);
    }
    //删除面试通知
    function del_action()
    {
        $JobM   =   $this -> MODEL('job');
        if ($_GET['id']) {
            $id     =   intval($_GET['id']);
            $return =   $JobM -> delYqms($id, array('uid' => $this->uid, 'usertype' => $this->usertype));
            $this -> layer_msg($return['msg'], $return['errcode'], $return['layertype'], $_SERVER['HTTP_REFERER']);
        }
    }
    //邀请函数据
    function ajax_action()
    {
        $JobM   =   $this -> MODEL('job');
        if ($_POST['id']) {

          
            $row    =   $JobM -> getYqmsInfo(array('id' => intval($_POST['id']),'isdel'=>9,'uid'=>$this->uid), array('yqh' => 1, 'uid' => $this->uid, 'usertype' => $this->usertype));
            
            echo json_encode($row);
            die();
        }
    }
    //面试邀请同意或者拒绝
    function inviteset_action()
    {
        $JobM       =   $this->MODEL('job');
        if($_GET['id']){
            $id         =   (int)$_GET['id'];
            $browse     =   (int)$_GET['browse'];
        }else if($_POST['id']){
            $id         =   (int)$_POST['id'];
            $browse     =   (int)$_POST['browse'];
        }
        $remark     =   '';
        if($browse=='4' && $_POST['remark']){
            $remark =   trim($_POST['remark']);
        }
        
        $return     =   $JobM->setYqms(array('id' => $id, 'browse' => $browse, 'remark'=>$remark, 'uid' => $this->uid, 'username' => $this->username, 'port' => '1'));

        if($_GET['id']){
            $this->layer_msg($return['msg'], $return['errcode'], 0, $_SERVER['HTTP_REFERER']);
        }else if($_POST['id']){
            $this ->  ACT_layer_msg($return['msg'], $return['errcode'],$_SERVER['HTTP_REFERER']);
        }
        

        
    }
}
?>
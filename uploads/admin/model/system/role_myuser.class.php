<?php

class role_myuser_controller extends adminCommon{
    /**
     * 管理员-我的账号
     */
	function index_action(){
        $adminM = $this->MODEL('admin');
        $user = $adminM->getAdminUser(array('uid' => $_SESSION['auid']));
        $group = $adminM->getAdminGroup(array('id' => $user['m_id'], 'did' => $this->config['did']));
        include(PLUS_PATH."/admindir.php");
        $rt = array(
            'username' => $user['username'],
            'mobile' => $user['moblie'],
            'real_name' => $user['name'] ? $user['name'] : '',
            'wxid' => $user['wxid'] ? $user['wxid'] : '',
            'qy_wxid' => $user['qy_userid'] ? $user['qy_userid'] : '',
            'last_login' => $user['lasttime'] ? date('Y-m-d H:i:s', $user['lasttime']) : '',
            'group_name' => $group['group_name'] ? $group['group_name'] : '',
            'qy_app_id' => $this->config['wx_qy_corpid'],
            'agent_id' => isset($this->config['wx_photo_agentId']) ? $this->config['wx_photo_agentId'] : $this->config['wx_qy_agentid'],
            'redirect_uri' => UrlEncode($this->config['sy_weburl'] . '/'.$admindir.'/#/myaccount'),
            'state' => $this->qywx_state
        );
        $this->render_json(0, 'ok', $rt);
	}
    /**
     * 管理员-我的账号-修改密码保存
     */
    function savePass_action(){
        if($_POST['useradd'] && $_SESSION['auid']){
            $_POST = $this->post_trim($_POST);
            $adminM = $this->MODEL('admin');
            $return = $adminM->upAdminUser(array('password' => $_POST['new_pwd']), array('uid' => $_SESSION['auid']), array('oldpass' => $_POST['old_pwd'],'okpassword' => $_POST['re_pwd']));
            if ($return['id']){
                unset($_SESSION['authcode']);
                unset($_SESSION['auid']);
                unset($_SESSION['ausername']);
                unset($_SESSION['ashell']);
            }
            if ($return['errcode'] == 9) {
                $this->admin_json( 0, $return['msg']);
            } else {
                $this->render_json( 1, $return['msg']);
            }
        }
    }
    /**
     * 解绑微信
     */
    function delAdminWxBind_action()
    {
        if ($_SESSION['auid']) {
            $adminM = $this->MODEL('admin');
            $uadmin = $adminM->getAdminUser(array('uid' => $_SESSION['auid']));
            if ($uadmin['wxid'] == '') {
                $this->render_json(1, '微信已解绑，请勿重复操作');
            } else {
                $rt = $adminM->upInfo(array('wxid' => ''), array('uid' => $_SESSION['auid']));
                if ($rt['id']) {
                    $this->admin_json( 0, '微信解绑成功');
                } else {
                    $this->render_json( 1, '微信解绑失败');
                }
            }
        }
    }

    /**
     * 解绑企微
     */
    function delAdminQyUserId_action()
    {
        if ($_SESSION['auid']) {
            $adminM = $this->MODEL('admin');
            $uadmin = $adminM->getAdminUser(array('uid' => $_SESSION['auid']));
            if ($uadmin['qy_userid'] == '') {
                $this->render_json(1, '企业微信已解绑，请勿重复操作');
            } else {
                $rt = $adminM->upInfo(array('qy_userid' => '', 'is_follower' => 2), array('uid' => $_SESSION['auid']));
                if ($rt['id']) {
                    $this->admin_json( 0, '企业微信解绑成功');
                } else {
                    $this->render_json( 1, '企业微信解绑失败');
                }
            }
        }
    }
}
?>
<?php

class role_user_controller extends adminCommon{
    protected	$qywx_state = 'weblogin@phpyun';
    /**
     * 管理员-管理员管理
     */
    function index_action(){
        $where = array();
        $_POST = $this->post_trim($_POST);
        if(!empty($_POST['keyword'])){
            $where['PHPYUNBTWSTART_A'] = '';
            $where['username'] = array('like',$_POST['keyword']);
            $where['name'] = array('like',$_POST['keyword'], 'OR');
            $where['PHPYUNBTWEND_A'] = '';
        }
        if(!empty($_POST['m_id'])){
            $where['m_id'] = $_POST['m_id'];
        }

        $adminM = $this->MODEL('admin');
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('admin_user', array(), $page, array('limit' => $pageSize));
        $userList = array();
        if ($pages['total']) {
            $where['orderby']  =  'uid';
            $where['limit'] = $pages['limit'];
            $List = $adminM->getList($where);
            foreach ($List as $v) {
                $logintime = $v['control_login'] ? explode(' - ', $v['control_login']) : array();
                $userList[] = array(
                    'uid' => $v['uid'],
                    'username' => $v['username'],
                    'group_name' => $v['group_name'],
                    'name' => $v['name'],
                    'mobile' => trim($v['moblie']),
                    'weixin' => trim($v['weixin']),
                    'qq' => trim($v['qq']),
                    'num' => $v['num'] ? $v['num'] : '',
                    'call_num' => $v['call_num'] ? $v['call_num'] : '',
                    'tuoxin_num' => $v['tuoxin_num'] ? $v['tuoxin_num'] : '',
                    'follow_num' => $v['follow_num'] ? $v['follow_num'] : '',
                    'deal_num' => $v['deal_num'] ? $v['deal_num'] : '',
                    'month_deal_num' => $v['month_deal_num'] ? $v['month_deal_num'] : '',
                    'jobtai_ranking' => $v['jobtai_ranking'],
                    'is_crm' => $v['is_crm'],
                    'isdid' => $v['isdid'],
                    'm_id' => $v['m_id'],
                    'crm_city' => $v['crm_city'] ? explode(',', $v['crm_city']) : array(),
                    'crm_duty' => $v['crm_duty'] ? explode(',', $v['crm_duty']) : array(),
                    'control_login' => $v['control_login'] ? explode(" - ", $v['control_login']) : '',
                    'login_start' => $logintime[0] ? $logintime[0] : '',
                    'login_end' => $logintime[1] ? $logintime[1] : '',
                    'index_lookstatistc' => $v['index_lookstatistc'],
                    'photo' => checkpic($v['photo']),
                    'ewm' => checkpic($v['ewm'])
                );
            }
            unset($List);
        }
        $data['list'] = $userList;
        $group = $adminM->getAdminGroupList(array('did' => $this->config['did'], 'orderby' => 'id'));
        $data['group'] = $group;
        $week = array(
            '1' => '礼拜一',
            '2' => '礼拜二',
            '3' => '礼拜三',
            '4' => '礼拜四',
            '5' => '礼拜五',
            '6' => '礼拜六',
            '7' => '礼拜日'
        );
        $data['week'] = $week;
        $data['total'] = intval($pages['total']);
        $data['perPage'] = $pageSize;
        $data['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'ok', $data);
    }
    /**
     * 管理员-添加、修改保存
     */
    function save_action(){
        $adminM = $this->MODEL('admin');
        if(isset($_POST['useradd'])){
            $_POST = $this->post_trim($_POST);
            if(!empty($_FILES)){
                if($_FILES['photo_file']['tmp_name']){
                    $upArrphoto = array(
                        'file' => $_FILES['photo_file'],
                        'dir' => 'adminuser'
                    );
                }
                if($_FILES['ewm_file']['tmp_name']){
                    $upArrewm = array(
                        'file' => $_FILES['ewm_file'],
                        'dir' => 'adminuser'
                    );
                }
                $uploadM  =  $this->MODEL('upload');
                if(!empty($upArrphoto)){
                    $picphoto = $uploadM->newUpload($upArrphoto);
                }
                if(!empty($upArrewm )){
                    $picewm = $uploadM->newUpload($upArrewm );
                }
                if (!empty($picphoto['msg'])) {
                    $this->render_json(1, $picphoto['msg']);
                }
                if (!empty($picphoto['picurl'])){
                    $photo = $picphoto['picurl'];
                }
                if (!empty($picewm['msg'])) {
                    $this->render_json(1, $picewm['msg']);
                }
                if (!empty($picewm['picurl'])){
                    $ewm = $picewm['picurl'];
                }
            }
            if($_POST['is_crm'] == '1'){
                if (!empty($_POST['crm_duty'])) {
                    $crm_duty =	pylode(',', $_POST['crm_duty']);
                }
                if (!empty($_POST['crm_city'])) {
                    $crm_city = $_POST['crm_city'] ? pylode(',', $_POST['crm_city']) : '';
                }
            }
            if ($_POST['login_start'] && $_POST['login_end']) {
                $logintime = array($_POST['login_start'], $_POST['login_end']);
            } else {
                $logintime = array();
            }
            $post = array(
                'username' => $_POST['username'],
                'name' => $_POST['name'],
                'm_id' => $_POST['m_id'],
                'moblie' => $_POST['mobile'],
                'weixin' => $_POST['weixin'],
                'qq' => $_POST['qq'],
                'is_crm' => $_POST['is_crm'],
                'num' => $_POST['num'],
                'call_num' => $_POST['call_num'],
                'tuoxin_num' => $_POST['tuoxin_num'],
                'follow_num' => $_POST['follow_num'],
                'deal_num' => $_POST['deal_num'],
                'month_deal_num' => $_POST['month_deal_num'],
                'jobtai_ranking' => intval($_POST['jobtai_ranking']),
                'crm_duty' => $crm_duty,
                'crm_city' => $crm_city,
                'r_status' => $_POST['r_status'],
                'photo' => $photo,
                'ewm' => $ewm,
                'depart' =>	$_POST['depart'],
                'control_login'=> $logintime ? implode(' - ', $logintime) : '',
                'index_lookstatistc'=>$_POST['index_lookstatistc']
            );

            $adminuser = $adminM->getAdminUser(array('uid'=>intval($_POST['uid'])));
            if($adminuser['photo'] && $photo==""){
                $post['photo'] = $adminuser['photo'];
            }
            if($adminuser['ewm'] && $ewm==""){
                $post['ewm'] = $adminuser['ewm'];
            }
            if($_POST['password']){
                $post['password'] = $_POST['password'];
            }
            if($_POST['isdid']){
                $post['isdid'] = intval($_POST['isdid']);
            }
            if (empty($_POST['uid'])){
                $return = $adminM->addAdminUser($post);
            }else{
                $return = $adminM->upAdminUser($post,array('uid'=>$_POST['uid']));
                if ($return['id'] && $_POST['uid']==$_SESSION['auid']){
                    if($post['username'] != $adminuser['username'] || !empty($post['password'])
                        || $post['name'] != $adminuser['name'] || $post['m_id'] != $adminuser['m_id']
                        || $post['isdid'] != $adminuser['isdid'] || $post['is_crm'] != $adminuser['is_crm']
                        || $post['control_login'] != $adminuser['control_login']
                        || $post['index_lookstatistc'] != $adminuser['index_lookstatistc']){
                        unset($_SESSION['authcode']);
                        unset($_SESSION['auid']);
                        unset($_SESSION['ausername']);
                        unset($_SESSION['ashell']);
                        $this->admin_json(0, '管理员(ID:'.$_POST['uid'].')修改成功,请重新登录！');
                    }
                }
            }
            if ($return['errcode'] == 9) {
                $this->admin_json( 0,$return['msg']);
            } else {
                $this->render_json( 1,$return['msg']);
            }
        }
    }

    /**
     * 管理员-管理员列表-删除
     */
    function del_action(){
        if ($_POST['del']){
            $adminM = $this->MODEL('admin');
            $return = $adminM->delAdminUser(array('uid' => intval($_POST['del'])));
            if ($return['errcode'] == 9) {
                $this->admin_json(0, $return['msg']);
            } else {
                $this->render_json( 1, $return['msg']);
            }
        }
    }
}
?>
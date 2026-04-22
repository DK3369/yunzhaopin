<?php

/**
 * Class domain_controller
 *
 * @author shiy
 * @version 7.0
 */
class domain_group_controller extends adminCommon
{
    //  管理员详细信息
    function adminInfo_action()
    {

        $adminM =   $this->MODEL('admin');
        $info   =   $adminM->getAdminUser(array('uid' => $_POST['uid']), array('field' => '`uid`,`username`,`name`,`m_id`,`did`'));
        $this->render_json(0,'', $info);
    }

    //  分站管理员
    function adminList_action()
    {

        $adminM =   $this->MODEL('admin');

        $where  =   array('did'   =>  array('>', 0));

        if (isset($_POST['keyword'])) {

            $keyStr =   trim($_POST['keyword']);
            $where['PHPYUNBTWSTART_A'] = '';
            $where['username'] = array('like', $keyStr);
            $where['name'] = array('like', $keyStr, 'OR');
            $where['PHPYUNBTWEND_A'] = '';
        }

        $page       =   !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize   =   !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);

        $pageM      =   $this->MODEL('page');
        $pages      =   $pageM->adminPageList('admin_user', $where, $page, array('limit' => $pageSize));

        $list   =   array();
        $total  =   intval($pages['total']);
        if ($total > 0) {
            if (isset($_POST['order'])) {

                $where['orderby']   =   $_POST['t'].' ,'.$_POST['order'];
            } else {

                $where['orderby']   =   'uid';
            }

            $where['limit'] = $pages['limit'];
            $list   =   $adminM->getList($where);
        }
        $result =   array(

            'list'  =>  $list,
            'total' =>  intval($total),
            'pageSize'  =>  $pageSize,
            'pageSizes' =>  $pages['page_sizes']
        );
        $this->render_json(0, '', $result);
    }

    //  删除分站管理员信息
    function delAdmin_action()
    {

        $adminM = $this->MODEL('admin');
        $delUid = $_POST['uid'];
        if (!empty($delUid)) {
            if (is_array($delUid)) {

                $where = array('uid' => implode(',', $delUid));
            } else {

                $where = array('uid' => addslashes($delUid));
            }
            $return = $adminM->delAdminUser($where);
            if ($return['errcode'] == 9) {

                $this->admin_json(0, $return['msg']);
            } else {

                $this->admin_json(1, $return['msg']);
            }
        } else {
            $this->render_json(1, '参数错误，请重试！');
        }
    }

    //  添加/修改分站管理员，所用缓存数据（分站、用户组）
    function getAdminCache_action()
    {

        $adminM = $this->MODEL('admin');
        $group = $adminM->getAdminGroupList(array('group_type' => '2'), array('field' => '`id`,`did`,`group_name`'));
        $groupArr = array();
        foreach ($group as $k1 => $v1) {
            $groupArr[] = array('id' => $v1['id'], 'name' => $v1['group_name']);
        }

        $siteM = $this->MODEL('site');
        $domain = $siteM->getList(array('orderby' => 'id,desc'), array('field' => "`id`,`title`"));
        $domainArr = array();
        foreach ($domain as $k2 => $v2) {
            $domainArr[] = array('id' => $v2['id'], 'name' => $v2['title']);
        }

        $this->render_json(0, '', array('groupArr' => $groupArr, 'domainArr' => $domainArr));
    }

    function saveAdmin_action()
    {

        $_POST  =   $this->post_trim($_POST);
        $adminM =   $this->MODEL('admin');

        $post   =    array(
            'username' => $_POST['username'],
            'name' => $_POST['name'],
            'm_id' => $_POST['m_id'],
        );

        if ($_POST['password']) {
            $post['password'] = $_POST['password'];
        }
        if ($_POST['did']) {
            $post['did'] = intval($_POST['did']);
        }

        if (!isset($_POST['uid'])) {

            $return =   $adminM->addAdminUser($post);
        } else {

            $return =   $adminM->upAdminUser($post, array('uid' => $_POST['uid']));
            if ($return['errcode'] == '9' && $_POST['uid'] == $_SESSION['auid']) {

                unset($_SESSION['authcode']);
                unset($_SESSION['auid']);
                unset($_SESSION['ausername']);
                unset($_SESSION['ashell']);

                $this->admin_json(0, '管理员(ID:' . $_POST['uid'] . ')修改成功,请重新登录！');
            }
        }
        if ($return['errcode'] == '9'){

            $this->admin_json(0, $return['msg']);
        }else{

            $this->admin_json(1, $return['msg']);
        }
    }

    //  分站管理员权限组
    function groupList_action()
    {

        $adminM =   $this->MODEL('admin');

        $where  =   array('group_type' => 2);

        if (isset($_POST['keyword'])){

            $where['group_name']    =   array('like', trim($_POST['keyword']));
        }

        $page       =   !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize   =   !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);

        $pageM      =   $this->MODEL('page');
        $pages      =   $pageM->adminPageList('admin_user_group', $where, $page, array('limit' => $pageSize));

        $list   =   array();
        $total  =   intval($pages['total']);
        if ($total > 0) {
            if (isset($_POST['order'])) {

                $where['orderby']   =   $_POST['t'].' ,'.$_POST['order'];
            } else {

                $where['orderby']   =   'id';
            }

            $where['limit'] = $pages['limit'];
            $list = $adminM->getAdminGroupList($where, array('field' => '`id`,`group_name`,`did`', 'utype' => 'admin', 'uwhere'=>array('groupby'=>'m_id')));
        }

        $result = array(

            'list'  =>  $list,
            'total' =>  intval($total),
            'pageSize'   =>  $pageSize,
            'pageSizes' =>  $pages['page_sizes']
        );
        $this->render_json(0, '', $result);
    }

    //  删除分站管理员权限组信息
    function delGroup_action()
    {

        $adminM = $this->MODEL('admin');
        $delId  = $_POST['id'];
        if (!empty($delId)) {
            if (is_array($delId)) {

                $where = array('id' => implode(',', $delId));
            } else {

                $where = array('id' => $delId);
            }

            $return = $adminM->delAdminGroup($where);
            if ($return['errcode'] == 9) {

                $this->admin_json(0, $return['msg']);
            } else {

                $this->admin_json(1, $return['msg']);
            }
        } else {

            $this->render_json(1, '参数错误，请重试！');
        }
    }

    //  添加/修改分站管理员权限
    function groupInfo_action()
    {
        $power  =   $groupInfo = array();

        //修改管理员类型
        if (isset($_POST['id']) && intval($_POST['id']) > 0) {

            $adminM =   $this->MODEL('admin');
            $groupInfo  =   $adminM->getAdminGroup(array('id' => intval($_POST['id']), 'did' => $this->config['did']));
            $power  =   unserialize($groupInfo['group_power']);
        }

        $navM   =   $this->MODEL('navigation');
        $navArr =   $navM->getAdminNavList(array('display' => array('<>', '1'), 'orderby' => 'sort,asc'), array('utype' => 'power'));
        $dataArr=   array(

            'one_menu'  =>  $navArr['one_menu'],
            'two_menu'  =>  $navArr['two_menu'],
            'three_menu'=>  $navArr['three_menu'],
            'navigation'=>  $navArr['navigation']
        );

        // 一级、二级菜单id对应关系数组
        $oneChildIds = $twoChildIds = array();
        foreach ($navArr['one_menu'] as $oneId => $twoArr) {
            foreach ($twoArr as $twoVal) {
                $oneChildIds[$oneId][] = $twoVal['id'];
            }
        }
        // 二级、三级菜单id对应关系数组
        foreach ($navArr['two_menu'] as $twoId => $threeArr) {
            foreach ($threeArr as $threeVal) {
                $twoChildIds[$twoId][] = $threeVal['id'];
            }
        }
        $dataArr['one_children_ids'] = $oneChildIds;
        $dataArr['two_children_ids'] = $twoChildIds;
        $checkedThreeIds = $checkedFourIds = array();
        // 处理已选中的三级菜单
        foreach ($dataArr['two_menu'] as $v) {
            foreach ($v as $vv) {
                if (in_array($vv['id'], $power)) {
                    $checkedThreeIds[] = $vv['id'];
                }
            }
        }
        // 处理已选中的四级菜单
        foreach ($dataArr['three_menu'] as $v) {
            foreach ($v as $vv) {
                if (in_array($vv['id'], $power)) {
                    $checkedFourIds[] = $vv['id'];
                }
            }
        }
        $dataArr['checked_three_ids'] = $checkedThreeIds;
        $dataArr['checked_four_ids'] = $checkedFourIds;
        $dataArr['groupInfo'] = $groupInfo;
        $dataArr['power'] = $power;


        $siteM  =   $this->MODEL('site');
        $domain =   $siteM->getList(array('orderby' => 'id,desc'), array('field' => "`id`,`title`"));
        $dataArr['domain']  =   $domain;

        $this->render_json(0, '', $dataArr);
    }

    function saveGroup_action()
    {

        if ($_POST['group_name'] == '') {
            $this->render_json(1, '请填写权限组名称');
        }

        if (empty($_POST['three_ids'])) {

            $this->render_json(1, '请至少选择一项权限');
        } else {

            $powerA = array_merge($_POST['one_ids'], $_POST['two_ids'], $_POST['three_ids']);

            if (!empty($_POST['four_ids'])) {

                $powerA = array_merge($powerA, $_POST['four_ids']);
            }
        }

        $power  =   array_filter($powerA);

        $post   =   array(

            'group_name'    =>  $_POST['group_name'],
            'group_power'   =>  serialize(array_filter($power)),
            'group_type'    =>  2,
            'did'           =>  $_POST['did'] ? $_POST['did'] : $this->config['did'],
        );

        $adminM =   $this->MODEL('admin');
        if (isset($_POST['groupid']) && intval($_POST['groupid']) > 0) {

            $return = $adminM->upAdminGroup($post, array('id' => intval($_POST['groupid'])));
        } else {

            $return = $adminM->addAdminGroup($post);
        }
        if (intval($return['errcode']) == 9) {

            $navigationM = $this->MODEL('navigation');
            $navigationM->upAdminNav(array('dids' => 0), array('display' => array('<>', 1)));
            $navigationM->upAdminNav(array('dids' => 1), array('id' => array('in', pylode(',', $power)), 'display' => array('<>', 1)));

            $error = 0;
        }else{

            $error = 1;
        }

        $this->admin_json($error, $return['msg']);
    }

}
?>
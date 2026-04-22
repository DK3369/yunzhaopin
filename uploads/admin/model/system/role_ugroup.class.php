<?php

class role_ugroup_controller extends adminCommon{
    /**
     * 管理员-管理员类型
     */
    function index_action(){
        $adminM = $this->MODEL('admin');
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('admin_user_group', array(), $page, array('limit' => $pageSize));
        $List = array();
        if ($pages['total']) {
            $where['limit'] = $pages['limit'];
            $List = $adminM->getAdminGroupList(array('did' => $this->config['did'], 'orderby' => 'id'), array('utype' => 'admin'));
            foreach ($List as &$v) {
                $v['group_type_n'] = $v['group_type'] ==  1 ? '普通分组' : '分站管理组';
            }
        }
        $data['list'] = $List;
        $data['total'] = intval($pages['total']);
        $data['perPage'] = $pageSize;
        $data['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'ok', $data);
    }
    /**
     * 管理员-添加管理员类型
     */
    function info_action(){
        $power = $group = array();
        //修改管理员类型
        if($_POST['id']){
            $adminM = $this->MODEL('admin');
            $group = $adminM->getAdminGroup(array('id' => intval($_POST['id']), 'did' => $this->config['did']));
            $power = unserialize($group['group_power']);
        }
        $navigationM = $this->MODEL('navigation');
        $return = $navigationM->getAdminNavList(array('display' => array('<>', '1'), 'orderby' => 'sort,asc'), array('utype' => 'power'));
        $setarr = array(
            'one_menu' => $return['one_menu'],
            'two_menu' => $return['two_menu'],
            'three_menu' => $return['three_menu'],
            'navigation' => $return['navigation']
        );
        $oneChildIds = $twoChildIds = array();
        // 一级、二级菜单id对应关系数组
        foreach ($return['one_menu'] as $oneId => $twoArr) {
            foreach ($twoArr as $twoVal) {
                $oneChildIds[$oneId][] = $twoVal['id'];
            }
        }
        // 二级、三级菜单id对应关系数组
        foreach ($return['two_menu'] as $twoId => $threeArr) {
            foreach ($threeArr as $threeVal) {
                $twoChildIds[$twoId][] = $threeVal['id'];
            }
        }
        $setarr['one_children_ids'] = $oneChildIds;
        $setarr['two_children_ids'] = $twoChildIds;
        $checkedThreeIds = $checkedFourIds = array();
        // 处理已选中的三级菜单
        foreach ($setarr['two_menu'] as $v) {
            foreach ($v as $vv) {
                if (in_array($vv['id'], $power)) {
                    $checkedThreeIds[] = $vv['id'];
                }
            }
        }
        // 处理已选中的四级菜单
        foreach ($setarr['three_menu'] as $v) {
            foreach ($v as $vv) {
                if (in_array($vv['id'], $power)) {
                    $checkedFourIds[] = $vv['id'];
                }
            }
        }
        $setarr['checked_three_ids'] = $checkedThreeIds;
        $setarr['checked_four_ids'] = $checkedFourIds;
        $setarr['admin_group'] = $group;
        $setarr['power'] = $power;
        $this->render_json(0, 'ok', $setarr);
    }
    /**
     * 管理员-管理员类型-添加、修改保存
     */
    function save_action()
    {
        if (!$_POST['group_name']) {
            $this->render_json(1, '请填写权限组名称');
        }
        $powerA = array_merge($_POST['one_ids'], $_POST['two_ids'], $_POST['three_ids'], $_POST['four_ids']);
        $power = array_filter($powerA);
        if (empty($power)) {
            $this->render_json(1, '请至少选择一项权限');
        }
        $post = array(
            'group_name'   =>  $_POST['group_name'],
            'group_power'  =>  serialize(array_filter($power))
        );
        $adminM = $this->MODEL('admin');
        //检查权限组名称是否重复
        $info = $adminM->getAdminGroup(array('group_name'=>$_POST['group_name']));
        if (!empty($info) && !$_POST['groupid']){
            $this->render_json(1, '权限组名称已存在');
        }
        if (empty($_POST['groupid'])) {
            $post['group_type'] = 1;
            $post['did'] = $this->config['did'];

            $return = $adminM->addAdminGroup($post);
        } else {
            $return = $adminM->upAdminGroup($post, array('id' => intval($_POST['groupid'])));
        }
        if ($return['errcode'] == 9) {
            if ($_POST['group_name'] == '分站管理员') {
                $navigationM = $this->MODEL('navigation');
                $navigationM->upAdminNav(array('dids' => 0), array('display' => array('<>', 1)));
                $navigationM->upAdminNav(array('dids' => 1), array('id' => array('in', pylode(',', $power)), 'display' => array('<>', 1)));
            }
        }
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json( 1, $return['msg']);
        }
    }
    /*
     * 删除管理员类型
     */
    function del_action()
    {
        $del = intval($_POST['id']);
        if($del){
            $return	= $this->MODEL('admin')->delAdminGroup(array('id' => $del));
            $this->admin_json($return['errcode'] == 9 ? 0 : 1, $return['msg']);
        }else{
            $this->render_json(1, '参数错误,请重试');
        }
    }
}
?>
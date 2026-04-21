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
class admin_nav_controller extends adminCommon{
    /**
     * 后台导航管理
     */
	function index_action(){
        $navigationM = $this->MODEL('navigation');

        $data = array();
        $where = array('orderby' => 'sort');

        if (isset($_POST['keyid'])) { // 前端懒加载需要
            $where['keyid'] = $_POST['keyid'];
            $data['hasChildren'] = true;
        }
        $return = $navigationM->getAdminNavList($where, $data);

        $list = !empty($return['list']) ? isset($_POST['keyid']) ? $return['list'] : $navigationM->childrenList($return['list']) : array(); // 存在keyid不查子级

        $this->render_json(0, '', compact('list'));
	}

	/**
	 * 后台导航管理：添加
	 */
	function add_action(){
	    $data  =  array(
	        'keyid'    	=>  $_POST['keyid'],
	        'name'    	=>  $_POST['name'],
	        'url'     	=>  $_POST['url'],
            'path'     	=>  $_POST['path'],
	        'classname'	=>  $_POST['classname'],
	        'display'  	=>  $_POST['display'],
			'dids'    	=>  $_POST['dids'],
	        'sort'    	=>  $_POST['sort'] == '' ? '0' : $_POST['sort']
	    );
	    $navM  =  $this -> MODEL('navigation');

        if (!empty($_POST['id'])) {
            $id = $navM->upAdminNav($data, array('id' => intval($_POST['id'])));
            if ($id) {
                $this->admin_json(0, '导航修改成功');
            } else {
                $this->admin_json(1, '导航修改失败');
            }
        } else {
            $id = $navM->addAdminNav($data);
            if ($id) {
                $this->admin_json(0, '导航添加成功');
            } else {
                $this->admin_json(1, '导航添加失败');
            }
        }
	}

	// 升级记录
	function version_action(){
	    
	    $versionM = $this->MODEL('version');
        $list     = $versionM->getVersionList();

        $this->render_json(0, '', compact('list'));
	}
	
	function path_action(){
	    
	    echo APP_PATH;   
	}

    /**
     * 导航信息获取
     *
     */
    function info_action()
    {
        $navM = $this->MODEL('navigation');

        $info = $navM->getAdminNav(array('id' => intval($_POST['id'])));

        $this->render_json(0, '', compact('info'));
    }

    /**
     * 后台导航删除
     */
    function del_action()
    {
        $navM = $this->MODEL('navigation');

        $return = $navM->delAdminNav(array('id' => intval($_POST['id'])));

        $this->admin_json($return['error'] == 9 ? 0 : 1, $return['msg']);
    }

    // 导航是否启用
    function changeDisplay_action()
    {
        $navigationM = $this->MODEL('navigation');

        $res = $navigationM->upAdminNav(array(trim($_POST['field']) => intval($_POST['status'])), array('id' => intval($_POST['id'])));

        if ($res) {
            $this->admin_json(0, "导航状态(ID:{$_POST['id']})修改成功");
        } else {
            $this->admin_json(1, "导航状态(ID:{$_POST['id']})修改失败");
        }
    }
}

?>
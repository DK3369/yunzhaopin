<?php

class admin_nav_controller extends adminCommon{
    /**
     * Admin navigation management.
     */
	function index_action(){
        $navigationM = $this->MODEL('navigation');

        $data = array();
        $where = array('orderby' => 'sort');

        if (isset($_POST['keyid'])) { // Required for frontend lazy loading.
            $where['keyid'] = $_POST['keyid'];
            $data['hasChildren'] = true;
        }
        $return = $navigationM->getAdminNavList($where, $data);

        $list = !empty($return['list']) ? isset($_POST['keyid']) ? $return['list'] : $navigationM->childrenList($return['list']) : array(); // Do not query child nodes when keyid exists.

        $this->render_json(0, '', compact('list'));
	}

	/**
	 * Admin navigation management: add.
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
                $this->admin_json(0, 'admin_01363');
            } else {
                $this->admin_json(1, 'admin_01364');
            }
        } else {
            $id = $navM->addAdminNav($data);
            if ($id) {
                $this->admin_json(0, 'admin_01365');
            } else {
                $this->admin_json(1, 'admin_01366');
            }
        }
	}

	// Upgrade records.
	function version_action(){
	    
	    $versionM = $this->MODEL('version');
        $list     = $versionM->getVersionList();

        $this->render_json(0, '', compact('list'));
	}
	
	function path_action(){
	    
	    echo APP_PATH;   
	}

    /**
     * Get navigation info.
     *
     */
    function info_action()
    {
        $navM = $this->MODEL('navigation');

        $info = $navM->getAdminNav(array('id' => intval($_POST['id'])));

        $this->render_json(0, '', compact('info'));
    }

    /**
     * Delete admin navigation.
     */
    function del_action()
    {
        $navM = $this->MODEL('navigation');

        $return = $navM->delAdminNav(array('id' => intval($_POST['id'])));

        $this->admin_json($return['error'] == 9 ? 0 : 1, $return['msg']);
    }

    // Toggle navigation enabled state.
    function changeDisplay_action()
    {
        $navigationM = $this->MODEL('navigation');

        $res = $navigationM->upAdminNav(array(trim($_POST['field']) => intval($_POST['status'])), array('id' => intval($_POST['id'])));

        if ($res) {
            $this->admin_json(0, yun_t('admin_model_00083', array('id' => $_POST['id'])));
        } else {
            $this->admin_json(1, yun_t('admin_model_00084', array('id' => $_POST['id'])));
        }
    }
}

?>

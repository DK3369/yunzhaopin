<?php

class set_navmap_controller extends adminCommon{
    /**
     * 网站地图
     */
    function index_action(){
        $NavmapM = $this->MODEL('navmap');
        if($_POST['eject']){
            if($_POST['eject'] == '2'){
                $where['eject'] = '0';
            }else{
                $where['eject'] = $_POST['eject'];
            }
        }
        if($_POST['display']){
            if($_POST['display'] == '2'){
                $where['display'] =	'0';
            }else{
                $where['display'] =	$_POST['display'];
            }
        }
        if($_POST['ctype']){
            
            $where['type'] = $_POST['ctype'];
            
        }
        if (trim($_POST['keyword'])){
            if($_POST['type']=='2'){
                $where['url'] =	array('like', trim($_POST['keyword']));
            }else{
                $where['name'] = array('like', trim($_POST['keyword']));
            }
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('navmap', $where, $page, array('limit' => $pageSize));
        $nav = array();
        if($pages['total']>0){
            $where['orderby'] =	'sort';
            $where['limit'] = $pages['limit'];
            $nav = $NavmapM->getNavMapList($where);
            if(is_array($nav)){
                foreach($nav as $key => $value){
                    $nav[$key]['display_n'] = $value['display'] == '1';
                    foreach($nav as $k => $v){
                        if($value['id'] == $v['nid']){
                            $nav[$k]['typename'] = $value['name'];
                        }
                    }
                }
            }
        }
        $data['list'] = $nav;
        $data['total'] = intval($pages['total']);
        $data['perPage'] = $pageSize;
        $data['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'ok', $data);
    }
    /**
     * 网站地图信息
     */
	function getTypes_action(){
        $NavmapM = $this->MODEL('navmap');
        $types =	$NavmapM->getNavMapList(1, array('field' => '`id`,`name`'));
        $tpArr = array();
        foreach ($types as $tp) {
            $tpArr[] = array('label' => $tp['name'], 'value' => $tp['id']);
        }
        $rt = array(
            'type' => $tpArr
        );
        $this->render_json(0, 'ok', $rt);
	}

    /**
     * 网站地图-添加、修改保存
     */
    function save_action(){
        $NavmapM = $this->MODEL('navmap');
        if($_POST['submit']){
            $postData=array(
                'nid' => $_POST['nid'],
                'eject' => $_POST['eject'],
                'display' => $_POST['display'],
                'name' => $_POST['name'],
                'url' => str_replace("amp;","", $_POST['url']),
                'furl' => $_POST['furl'],
                'sort' => $_POST['sort'],
                'type' => $_POST['type'],
            );
            if($_POST['id']){
                $nbid =	$NavmapM->upNavMap(array('id'=>$_POST['id']),$postData);
                $msg = "更新网站地图(ID:".$_POST['id'].")";
            }else{
                $nbid =	$NavmapM->addNavMap($postData);
                $msg = "添加网站地图";
            }
            $this->cache_action();
            if (isset($nbid) && $nbid) {
                $this->admin_json(0, $msg."成功！");
            } else {
                $this->admin_json(1, $msg."失败！");
            }
        }
    }

    /*
     * 删除网站地图
     */
    function del_action(){
        //批量删除
        $NavmapM = $this->MODEL('navmap');
        $return = $NavmapM->delNavMap($_POST['del']);
        $this->cache_action();
        $this->admin_json($return['errcode'] == 9 ? 0 : 1, $return['msg']);
    }

    function cache_action(){
        include(LIB_PATH."cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $makecache=$cacheclass->navmap_cache("navmap.cache.php");
    }
    /*
     * 网站地图显示、隐藏
     */
    function nav_xianshi_action(){
        $NavmapM = $this->MODEL('navmap');
        $nid = $NavmapM->upNavMap(array('id' => intval($_POST['id'])), array(''.$_POST['type'].'' => intval($_POST['rec'])));
        if($_POST['type']=='display'){
            $this->MODEL('log')->addAdminLog("网站地图是否显示(ID:".$_POST['id'].")设置成功！");
        }else{
            $this->MODEL('log')->addAdminLog("网站地图是否新窗口打开(ID:".$_POST['id'].")设置成功！");
        }
        $this->cache_action();
        $this->render_json($nid ? 0 : 1);
    }
}
?>
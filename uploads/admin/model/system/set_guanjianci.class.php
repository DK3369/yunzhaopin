<?php

class set_guanjianci_controller extends adminCommon
{
    /**
     * @return void
     */
    function keyWord_action(){
        $keywordarr = array(
            "1" => 'wap_js_00130', "2" => 'wap_user_00220', "3" => 'wap_user_00154',
            "4" => 'default_00262', "5" => 'wap_com_00428',
            "8" => 'admin_01381', "9" => 'admin_01382',
            "10" => 'wap_user_00084', "11" => 'admin_user_00018', '12' => 'wap_user_00223',
            '13' => 'wap_js_00066'
        );
        $this->render_json(0, '', $keywordarr);
    }

    function index_action(){
        $hotM = $this->MODEL('hotkey');
        $search_list[] = array("param" => "rec", "name" => 'wap_01465', "value" => array("1" => 'admin_01339', "2" => 'admin_system_00448'));
        $search_list[] = array("param" => "check", "name" => 'wap_com_00406', "value" => array("1" => 'wap_user_00165', "2" => 'wap_user_00166'));
        $type = (int)$_POST['type'];
        if ($type) {
            $where['type'] = $type;
        }
        if ($_POST['rec'] == '1') {
            $where['tuijian'] = 1;
        } elseif ($_POST['rec'] == '2') {
            $where['tuijian'] = 0;
        }
        if ($_POST['bold'] == '1') {
            $where['bold'] = 1;
        } elseif ($_POST['bold'] == '2') {
            $where['bold'] = 0;
        }
        if (trim($_POST['keyword'])) {
            $where['key_name'] = array('like', trim($_POST['keyword']));
            if ($_POST['cate'] != '') {
                $where['type'] = $_POST['cate'];
            }
        }
        if ($_POST["check"] == 1) {
            $where['check'] = $_POST['check'];
        } elseif ($_POST['check'] == "2") {
            $where['check'] = array('<>', 1);
        }
        $pageM = $this->MODEL('page');
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;

        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pages = $pageM->adminPageList('hot_key', $where, $page, array('limit' => $pageSize));
        if (!$pages['total']) {
            $this->render_json(0, yun_auto_t('数据为空!'),['list' => [], 'total' => 0,'pageSize'=>$pageSize]);
        }
        if ($_POST['order']) {
            $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
        } else {
            $where['orderby'] = 'id,desc';
        }
        $where['limit'] = $pages['limit'];
        $List = $hotM->getList($where);
        foreach ($List as &$v) {
            $v['check'] = $v['check'] ? true : false;
            $v['bold'] = $v['bold'] ? true : false;
            $v['tuijian'] = $v['tuijian'] ? true : false;
            $v['check'] = $v['check'] ? true : false;
        }

        $data= array(
            'list' => $List,
            'total' => (int)$pages['total'],
            'pageSize'=>$pageSize,
            'pageSizes'=>$pages['page_sizes']
        );

        $this->render_json(0, '',$data);
    }

    function save_action(){
        if (!trim($_POST['key_name'])) {
            $this->render_json(1, yun_at('admin_01383'));
        }
        $value['key_name'] = trim($_POST['key_name']);
        $value['num'] = trim($_POST['num']);
        $value['type'] = trim($_POST['type']);
        $value['size'] = trim($_POST['size']);
        $value['bold'] = trim($_POST['bold']) == "true"?1:0;
        $value['color'] = trim($_POST['color']);
        $value['tuijian'] = trim($_POST['tuijian'])== "true"?1:0;
        $value['check'] = '1';
        $hotM = $this->MODEL('hotkey');
        if ($_POST['id']) {
            $id = $_POST['id'];
            $oid = $hotM->upHotkey(array('id' => $_POST['id']), $value);
        } else {
            $oid = $hotM->addInfo($value);
            $id = $oid;
        }
        $this->get_cache();
        if ($oid){
            $this->admin_json(0,"关键字(ID:" . $id . ")保存成功！");
        }else{
            $this->render_json(1,yun_at('admin_system_00042'));
        }
    }

    function del_action(){

        if ($_POST['del']) {
            if (is_array($_POST['del'])) {
                $where['id'] = array('in', pylode(',', $_POST['del']));
            } else {
                $this->render_json(1, yun_at('admin_system_00043'));
            }
        }
        if ($_POST['id']) {
            $where['id'] = $_POST['id'];
        }
        if (!isset($where)) {
            $this->render_json(1, yun_at('admin_system_00043'));
        }
        $hotM = $this->MODEL('hotkey');
        $return = $hotM->delHotkey($where);
        $this->get_cache();
        if ($return) {
            $this->render_json($return['code'], $return['msg']);
        }
        $this->render_json($return['code'], $return['msg']);
    }

    /**
     * 对关键字 进行 加粗 推荐 审核设置
     * @return void
     */
    function recup_action()
    {
        $hotM = $this->MODEL('hotkey');
        $data['type'] = $_POST['type'];
        $data['id'] = $_POST['id'];
        $data['rec'] = $_POST['rec']== 'true'?1:0;
        $return = $hotM->recupHotkey($data);
        $this->get_cache();
        if ($return) {
            $this->render_json(0, yun_auto_t('操作成功!'));
        }
        $this->render_json(0, yun_auto_t('操作失败!'));
    }

    function get_cache()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $makecache = $cacheclass->keyword_cache("keyword.cache.php");
    }

    function status_action()
    {

        $hotM = $this->MODEL('hotkey');
        if (!$_POST['pid']) {
            $this->render_json(1, yun_auto_t('非法操作!'));

        }
        if (is_array($_POST['pid'])){
            $aid = implode(',',$_POST['pid']);
        }else{
            $aid =  $_POST['pid'];
        }
        $data['size'] = $_POST['size'];
        $data['check'] = $_POST['check']== 'true'?1:0;;
        $data['tuijian'] = $_POST['tuijian']== 'true'?1:0;;
        $data['bold'] = $_POST['bold']== 'true'?1:0;;
        if ($_POST['type']) {
            $data['type'] = $_POST['type'];
        }
        $data['color'] = $_POST['color'];
        $id = $hotM->upHotkey(array('id' => array('in', $aid)), $data);
        $this->get_cache();
        if ($id){
            $this->admin_json(0, "关键字(ID:" . $aid . 'admin_system_00044');
        }
        $this->admin_json(1,"关键字(ID:" . $aid . ")批量审核失败！");

    }

    /**
     * 批量审核
     * @return void
     */
    function state_action()
    {
        if (!$_POST['sid']) {
            $this->render_json(1, yun_auto_t('请选择关键词!'));
        }
        if (is_array($_POST['sid'])){
            $aid = implode(',',$_POST['sid']);
        }else{
            $aid =  $_POST['sid'];
        }
        $data['check'] = $_POST['status'];
        $hotM = $this->MODEL('hotkey');
        $id = $hotM->upHotkey(array('id' => array('in', $aid)), $data);
        $this->get_cache();
        if ($id){
            $this->admin_json(0, "关键字(ID:" . $aid . 'admin_system_00044');
        }
        $this->admin_json(1,"关键字(ID:" . $aid . ")批量审核失败！");
    }
}
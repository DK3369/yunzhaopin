<?php

class category_px_subject_type_controller extends adminCommon
{
    /**
     * 培训分类 开课类型
     */
    function index_action()
    {
        $categoryM = $this->MODEL('category');
        $whereData['orderby'] = 'sort,desc';
        $list = $categoryM->getSubjectTypeClassList($whereData);
        $this->render_json(0, '', $list);
    }

    //添加
    function add_action()
    {
        $_POST = $this->post_trim($_POST);
        if ($_POST) {
            $data['name'] = @explode('-', $_POST['name']);
            $categoryM = $this->MODEL('category');
            $return = $categoryM->addSubjectTypeClass($data);
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    //删除
    function del_action()
    {
        $whereData = array();
        $data = array();
        $categoryM = $this->MODEL('category');
        if ($_POST['delid']) {//单个删除
            $whereData['id'] = $_POST['delid'];
            $data['type'] = 'one';
        }
        if ($_POST['del']) {//批量删除
            $whereData['id'] = array('in', pylode(',', $_POST['del']));
            $data['type'] = 'all';
        }
        $return = $categoryM->delSubjectTypeClass($whereData, $data);
        $this->admin_json($return['error'], $return['msg']);
    }

    function ajax_action()
    {
        $categoryM = $this->MODEL('category');
        $whereData['id'] = $_POST['id'];
        $addData['sort'] = $_POST['sort'];
        $addData['name'] = $_POST['name'];
        if ($_POST) {
            $result = $categoryM->upSubjectTypeClass($addData, $whereData);
            $this->admin_json($result['error'], $result['msg']);
        }
    }
}
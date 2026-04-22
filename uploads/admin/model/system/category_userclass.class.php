<?php

class category_userclass_controller extends adminCommon
{
    /**
     * 系统-类别-会员分类：个人会员分类
     */
    function index_action()
    {
        $categoryM = $this->MODEL('category');
        $whereData['keyid'] = '0';
        $whereData['orderby'] = array('id,asc');
        $position = $categoryM->getUserClassList($whereData);
        $this->render_json(0, '', $position);
    }

    /**
     * 添加
     */
    function save_action()
    {
        $_POST = $this->post_trim($_POST);

        $addData['ctype'] = $_POST['ctype'];
        $addData['name'] = explode('-', $_POST['name']);
        $addData['variable'] = explode('-', $_POST['str']);
        $addData['keyid'] = $_POST['nid'];

        $categoryM = $this->MODEL('category');
        $return = $categoryM->addUserClass($addData);
        $this->admin_json($return['error'], $return['msg']);
    }

    /**
     * 分类管理
     */
    function up_action()
    {
        $return = array();
        $categoryM = $this->MODEL('category');
        //查询子类别
        if ($_POST['id']) {
            $id = $_POST['id'];
            $whereOne['id'] = $id;
            $whereTwo['keyid'] = $id;
            $whereTwo['orderby'] = array('sort,asc', 'id,asc');
            $class1 = $categoryM->getUserClass($whereOne);//单个第一级分类
            $class2 = $categoryM->getUserClassList($whereTwo);//第一级分类的子类

            $return['class1'] = $class1;
            $return['class2'] = $class2;
        }
        $position = $categoryM->getUserClassList(array('keyid' => '0', 'orderby' => array('id,asc')));//所有第一级分类
        $return['position'] = $position;
        $this->render_json(0, '', $return);
    }

    /**
     * 删除
     */
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
        if ($whereData) {
            $return = $categoryM->delUserClass($whereData, $data);
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    /**
     * 更新名称 排序
     */
    function ajax_action()
    {
        if ($_POST) {
            $categoryM = $this->MODEL('category');
            $whereData['id'] = $_POST['id'];
            $addData['sort'] = $_POST['sort'];
            $addData['name'] = $_POST['name'];

            $return = $categoryM->addUserClass($addData, $whereData);
            $this->admin_json($return['error'], $return['msg']);
        }
    }
}
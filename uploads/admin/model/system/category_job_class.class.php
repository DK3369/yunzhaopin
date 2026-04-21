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
class category_job_class_controller extends adminCommon
{
    /**
     * 职位类别
     */
    function index_action()
    {
        $categoryM = $this->MODEL('category');
        $whereData['keyid'] = '0';
        $whereData['orderby'] = array('sort,asc', 'id,asc');
        $position = $categoryM->getJobClassList($whereData);
        $this->render_json(0, '', $position);
    }

    function getJobClass_action()
    {
        $categoryM = $this->MODEL('category');
        $position = $categoryM->getJobClassList(array('keyid' => '0', 'orderby' => array('sort,asc', 'id,asc')));
        $this->render_json(0, '', $position);
    }

    function classadd_action()
    {
        $return = array();
        $categoryM = $this->MODEL('category');
        $position = $categoryM->getJobClassList(array('keyid' => '0', 'orderby' => array('sort,asc', 'id,asc')));

        if ($_POST['id']) {
            $info = $categoryM->getJobClass(array('id' => $_POST['id']));
            $job = $categoryM->getJobClass(array('id' => $info['keyid']));
            $class2 = $categoryM->getJobClassList(array('keyid' => $job['keyid'], 'orderby' => array('sort,asc', 'id,asc')));

            $return['type'] = 'three';
            $return['info'] = $info;
            $return['class2'] = $class2;
            $return['job'] = $job;
        } elseif ($_POST['tid']) {
            $info = $categoryM->getJobClass(array('id' => $_POST['tid']));

            $return['type'] = 'two';
            $return['info'] = $info;
        }

        $return['position'] = $position;
        $this->render_json(0, '', $return);
    }

    //添加职位
    function save_action()
    {
        $_POST = $this->post_trim($_POST);
        if ($_POST['submit']) {
            $data = array(
                'id' => intval($_POST['id']),
                'nid' => intval($_POST['nid']),//一级分类
                'name' => $_POST['position'],
                'e_name' => $_POST['e_name'],
                'keyid' => intval($_POST['keyid']),//二级分类
                'sort' => $_POST['sort'],
                'content' => strip_tags($_POST['content'])
            );
            $categoryM = $this->MODEL('category');
            $return = $categoryM->addJobClass($data);
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    //职位管理
    function up_action()
    {
        $return = array();
        $categoryM = $this->MODEL('category');
        //查询子类别
        if ((int)$_POST['id']) {
            $id = (int)$_POST['id'];
            $onejob = $categoryM->getJobClass(array('id' => $_POST['id']));
            $return = $categoryM->getJobClassList(array('keyid' => $_POST['id'], 'orderby' => array('sort,asc', 'id,asc')), '*', array('type' => 'oneall'));
            $twojob = $return['twojob'];
            $threejob = $return['threejob'];

            $return['id'] = $id;
            $return['onejob'] = $onejob;
            $return['twojob'] = $twojob;
            $return['threejob'] = $threejob;
        }
        $position = $categoryM->getJobClassList(array('keyid' => '0', 'orderby' => array('sort,asc', 'id,asc')));
        $return['position'] = $position;
        $this->render_json(0, '', $return);
    }

    //删除
    function del_action()
    {
        $categoryM = $this->MODEL('category');
        if ((int)$_POST['delid']) {
            $whereData['id'] = $_POST['delid'];
            $data['type'] = 'one';
        } else if ($_POST['del']) { //批量删除
            $whereData['id'] = array('in', pylode(',', $_POST['del']));
            $data['type'] = 'all';
        }
        if ($whereData) {
            $return = $categoryM->delJobClass($whereData, $data);
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    function move_action()
    {
        if ($_POST) {
            $keyid = $_POST['keyid'];
            $nid = $_POST['nid'];
            $pid = $_POST['pid'];
            $categoryM = $this->MODEL('category');

            $addData['keyid'] = intval($keyid ? $keyid : $nid);
            $whereData['id'] = $pid;

            $return = $categoryM->upJobClass($addData, $whereData);
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    function ajax_action()
    {
        if ($_POST) {
            $categoryM = $this->MODEL('category');
            $whereData['id'] = $_POST['id'];
            $addData['sort'] = $_POST['sort'];
            $addData['name'] = $_POST['name'];
            if ($_POST['e_name']) {
                $addData['e_name'] = $_POST['e_name'];
            } elseif ($_POST['s_name']) {
                $addData['s_name'] = $_POST['s_name'];
            }

            $return = $categoryM->upJobClass($addData, $whereData);
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    /**
     * 推荐
     */
    function setrec_action()
    {
        if ($_POST['id']) {
            $categoryM = $this->MODEL('category');
            $whereData['id'] = $_POST['id'];
            $addData['rec'] = $_POST['rec'];
            $return = $categoryM->upJobClass($addData, $whereData);
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    /**
     * 移动 获取下一级分类列表
     */
    function get_class_action()
    {
        if ($_POST['nid']) {
            $categoryM = $this->MODEL('category');
            $typeclass = $categoryM->getJobClassList(array('keyid' => intval($_POST['nid']), 'orderby' => array('sort,asc', 'id,asc')), 'id,name,keyid');

            $this->render_json(0, '', $typeclass);
        }
    }

    /**
     * 生成拼音
     */
    function ajaxpinyin_action()
    {
        if ($_POST) {
            $where['e_name'][] = array('isnull');
            $where['e_name'][] = array('=', '', 'OR');
            $where['orderby'] = array('sort,desc', 'id,asc');
            $data['field'] = '`id`,`name`,`e_name`';
            $data['type'] = 'job';
            $data['post'] = $_POST;
            $categoryM = $this->MODEL('category');
            $return = $categoryM->setPinYin($where, $data);
            if ($return['error'] === 0) {
                $this->admin_json($return['error'], '职位类别' . $return['msg']);
            } else {
                $this->render_json($return['error'], '职位类别' . $return['msg'], array('page' => $return['page']));
            }
        }
    }

    /**
     * 一键查重
     */
    function ajaxchachong_action()
    {
        if ($_POST) {
            $where['e_name'] = array('<>', '');
            $where['groupby'] = 'e_name';
            $where['having'] = array('enum' => array('>', '1'));
            $data = array(
                'field' => '*,count(*) as enum',
                'page' => $_POST['page'],
                'type' => 'job'
            );
            $categoryM = $this->MODEL('category');
            $list = $categoryM->setChaChong($where, $data);
            $this->render_json(0, '', $list);
        }
    }

    function clearpinyin_action()
    {
        $categoryM = $this->MODEL('category');
        $categoryM->clearPinYin('job_class');
        $this->admin_json(0, '职位类别清空拼音成功');
    }
}
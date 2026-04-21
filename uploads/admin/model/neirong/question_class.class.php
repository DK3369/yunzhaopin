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

class question_class_controller extends adminCommon
{
    // 列表
    function index_action()
    {
        $askM = $this->MODEL('ask');

        if ($_POST['pid']) {
            $where['pid'] = $_POST['pid'];
        } else {
            $where['pid'] = "0";
        }

        if (trim($_POST['keyword'])) {
            $where['name'] = array('like', trim($_POST['keyword']));
        }

        $pageM = $this->MODEL('page');

        $pages = $pageM->adminPageList('q_class', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = array();
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = 'id';
            }

            $where['limit'] = $pages['limit'];

            $list = $askM->getQclassList($where, array('utype' => 'admin'));
        }

        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }

    // 获取分类信息
    function add_action()
    {
        $askM = $this->Model('ask');

        if ($_POST['id']) {
            $info = $askM->getQclassInfo($_POST['id']);
        }

        $classList = $askM->getQclassList(array('pid' => 0));

        $this->render_json(0, 'ok', compact('info', 'classList'));
    }

    // 保存分类
    function save_action()
    {
        $askM = $this->Model('ask');

        $_POST['intro'] = str_replace("&amp;", "&", $_POST['intro']);

        if ($_FILES['pic']['tmp_name']) {
            $upArr = array(
                'file' => $_FILES['pic'],
                'dir' => 'question_class'
            );

            $uploadM = $this->MODEL('upload');

            $picr = $uploadM->newUpload($upArr);

            if (!empty($picr['msg'])) {
                $this->render_json(1, $picr['msg']);
            } elseif (!empty($picr['picurl'])) {
                $pictures = $picr['picurl'];
            }
        }

        if (isset($pictures)) {
            $_POST['pic'] = $pictures;
        }

        if (!empty($_POST['id'])) {
            $id = intval($_POST['id']);
            $nbid = $askM->upQclassInfo(array('id' => $id), $_POST);
            $msg = "问答类别(ID:{$id})添加";
        } else {
            $nbid = $askM->addQclassInfo($_POST);
            $msg = "问答类别(ID:{$nbid})添加";
        }

        if ($nbid) {
            $this->cache_action();
            $this->admin_json(0, $msg . '成功');
        } else {
            $this->render_json(1, $msg . '失败');
        }
    }

    // 删除
    function del_action()
    {
        $askM = $this->Model('ask');


        if (is_array($_POST['del'])) {

            $delID    =   $_POST['del'];

        } else {
            $delID    =   $_POST['id'];
        }
        if (!$delID){
            $this->render_json(1,'参数错误');
        }
        $return = $askM->delQclass($delID);
        if ($return['errcode'] === 0) {
            if (is_array($delID)){
                $delID = pylode(',', $delID);
            }
            $this->del_question($delID);
            $this->cache_action();

            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    // 删除分类下问答
    function del_question($cid)
    {
        $askM = $this->Model('ask');

        $qid = $askM->getList(array('id' => $cid));
        if (!$qid) {
            return false;
        }

        foreach ($qid as $q_v) {
            $qids[] = $q_v['id'];
        }

        $qids = pylode(',', $qids);

        $askM->delquestiongroups($qids);
    }

    // 重新生成缓存
    function cache_action()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $cacheclass->ask_cache("ask.cache.php");
    }
}

?>
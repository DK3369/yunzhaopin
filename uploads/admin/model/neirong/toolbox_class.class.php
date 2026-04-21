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

class toolbox_class_controller extends adminCommon
{
    // 列表
    function index_action()
    {
        $hrM = $this->MODEL('hr');

        $list = $hrM->getClassList(array());

        $this->render_json(0, 'ok', compact('list'));
    }

    // 保存
    function save_action()
    {
        $post = $this->post_trim($_POST);

        if (empty($post) || empty($post['name']) || empty($post['content'])) {
            $this->render_json(1, '参数错误');
        }

        $hrM = $this->MODEL('hr');

        if ($_FILES['pic']['tmp_name']) {
            $upArr = array(
                'file' => $_FILES['pic'],
                'dir' => 'hrclass'
            );

            $uploadM = $this->MODEL('upload');

            $pic = $uploadM->newUpload($upArr);

            if (!empty($pic['msg'])) {
                $this->render_json(1, $pic['msg']);
            } elseif (!empty($pic['picurl'])) {
                $data['pic'] = $pic['picurl'];
            }
        }

        $data['name'] = $post['name'];
        $data['content'] = $post['content'];

        if (!empty($post['id'])) {
            $id = intval($post['id']);
            $nid = $hrM->upClassInfo(array('id' => $id), $data);
            $msg = "工具箱类别(ID:" . $id . ")修改";
        } else {
            $nid = $hrM->addClassInfo($data);
            $msg = "工具箱类别(ID:" . $nid . ")添加";
        }

        if ($nid) {
            $this->admin_json(0, $msg . '成功');
        } else {
            $this->render_json(1, $msg . '失败');
        }
    }

    // 删除
    function del_action()
    {
        if (empty($_POST['del'])) {
            $this->render_json(1, '参数错误');
        }

        if (is_array($_POST['del'])) {
            $delID = $_POST['del'];
        } else {
            $delID = intval($_POST['del']);
        }

        $hrM = $this->Model('hr');

        $return = $hrM->delHrClass($delID);

        if ($return['errcode'] > 0) {
            $this->render_json(1, $return['msg']);
        } else {
            $this->admin_json(0, $return['msg']);
        }
    }
}

?>
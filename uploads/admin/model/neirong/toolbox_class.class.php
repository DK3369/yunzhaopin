<?php



class toolbox_class_controller extends adminCommon
{
    // 列表
    function index_action()
    {
        $hrM = $this->MODEL('hr');

        $list = $hrM->getClassList(array());

        $this->render_json(0, 'ok', compact('list'));
    }

    // Save.
    function save_action()
    {
        $post = $this->post_trim($_POST);

        if (empty($post) || empty($post['name']) || empty($post['content'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
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
            $successMsg = yun_t('admin_model_00197', array('id' => $id));
            $errorMsg = yun_t('admin_model_00198', array('id' => $id));
        } else {
            $nid = $hrM->addClassInfo($data);
            $successMsg = yun_t('admin_model_00199', array('id' => $nid));
            $errorMsg = yun_t('admin_model_00200', array('id' => $nid));
        }

        if ($nid) {
            $this->admin_json(0, $successMsg);
        } else {
            $this->render_json(1, $errorMsg);
        }
    }

    // Delete.
    function del_action()
    {
        if (empty($_POST['del'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
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

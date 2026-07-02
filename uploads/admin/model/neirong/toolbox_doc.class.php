<?php



class toolbox_doc_controller extends adminCommon
{
    // 设置高级搜索功能
    function set_search()
    {
        $search_list[] = array("param" => "status", "name" => 'admin_00271', "value" => array("1" => 'member_com_00023', "0" => 'admin_user_00340'));

        $ad_time = array('1' => 'common_01940', '3' => 'admin_user_00179', '7' => 'admin_user_00178', '15' => 'admin_user_00180', '30' => 'admin_user_00175');

        $search_list[] = array("param" => "end", "name" => 'admin_00269', "value" => $ad_time);

        return $search_list;
    }

    public function  getGroup_action(){
        $search_list = $this->set_search();
        $this->render_json(0,'',array('search_list'=>$search_list));
    }




    // 列表
    function index_action()
    {

        $hrM = $this->MODEL('hr');

        if ($_POST["type"] != "" && $_POST['keyword'] != "") {
            if ($_POST["type"] == "1") {
                $where['name'] = array('like', trim($_POST['keyword']));
            } elseif ($_POST['type'] == "2") {
                $hrclass = $hrM->getClassList(array('name' => array('like', trim($_POST['keyword']))), 'id');

                if ($hrclass) {
                    foreach ($hrclass as $v) {
                        $cids[] = $v['id'];
                    }

                    $where['cid'] = array('in', pylode(',', $cids));
                }
            }
        }

        if ($_POST['status'] == "0") {
            $where['is_show'] = $_POST['status'];
        } elseif ($_POST['status'] == "1") {
            $where['is_show'] = $_POST['status'];
        }

        if ($_POST['end']) {
            if ($_POST['end'] == 1) {
                $where['add_time'] = array('>=', strtotime(date("Y-m-d 00:00:00")));
            } else {
                $where['add_time'] = array('>=', strtotime('-' . intval($_POST['end']) . ' day'));
            }
        }

        $pageM = $this->MODEL('page');

        $pages = $pageM->adminPageList('toolbox_doc', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
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

            $list = $hrM->getList($where);

            $classList = $hrM->getClassList();

            if (is_array($list)) {
                foreach ($list as $key => $val) {
                    foreach ($classList as $value) {
                        if ($val['cid'] == $value['id']) {
                            $list[$key]['cname'] = $value['name'];
                        }
                    }
                }
            }
        }

        $this->render_json(0, 'ok', compact( 'list', 'total', 'page_sizes', 'limit', 'page'));
    }

    // 获取添加信息
    function add_action()
    {
        $hrM = $this->MODEL('hr');

        $info = '';

        if ($_POST['id']) {
            $id = intval($_POST['id']);
            $info = $hrM->getInfo($id);

            $info['file_name'] = basename($info['url']);
        }

        $classList = $hrM->getClassList();

        $this->render_json(0, 'ok', compact('classList', 'info'));
    }

    // Save.
    function save_action()
    {
        $hrM = $this->MODEL('hr');

        if ($_POST['name'] == '') {
            $this->render_json(1, yun_at('admin_neirong_00026'));
        } else if ($_POST['cid'] == '') {
            $this->render_json(1, yun_at('admin_01348'));
        }

        $id = !empty($_POST['id']) ? intval($_POST['id']) : '';

        if (!$id && $_FILES['file']['name'] == '') {
            $this->render_json(1, yun_at('admin_00268'));
        }

        $post = array(
            'name' => $_POST['name'],
            'cid' => $_POST['cid'],
            'is_show' => $_POST['is_show']
        );

        if (!empty($_FILES['file']) && $_FILES['file']['name']) {
            $upArr = array(
                'file' => $_FILES['file'],
                'dir' => 'hrdoc'
            );

            $uploadM = $this->MODEL('upload');

            $result = $uploadM->uploadDoc($upArr);

            if ($result['msg']) {
                $this->render_json(1, $result['msg']);
            } else {
                $post['url'] = $result['docurl'];
            }
        }

        if ($id) {
            $nid = $hrM->upHrInfo(array('id' => $id), array('post' => $post));
            $successMsg = yun_t('admin_model_00201', array('id' => $id));
            $errorMsg = yun_t('admin_model_00202', array('id' => $id));
        } else {
            $post['add_time'] = time();
            $nid = $hrM->addHrInfo($post);
            $successMsg = yun_t('admin_model_00203', array('id' => $nid));
            $errorMsg = yun_t('admin_model_00204', array('id' => $nid));
        }

        if ($nid) {
            $this->admin_json(0, $successMsg);
        } else {
            $this->render_json(1, $errorMsg);
        }
    }

    // Delete document.
    function del_action()
    {
        $hrM = $this->Model('hr');

        $delID = $_POST['id'] ? intval($_POST['id']) : $_POST['del'];

        $return = $hrM->delHr($delID);

        if ($return['errcode'] > 0) {
            $this->render_json(1, $return['msg']);
        } else {
            $this->admin_json(0, $return['msg']);
        }
    }

    // Frontend display status.
    function show_action()
    {
        $hrM = $this->Model('hr');
        $nid = $hrM->upHrInfo(array('id' => intval($_POST['id'])), array('post' => array('is_show' => intval($_POST['show']))));

        if ($nid) {
            $this->admin_json(0, yun_t('admin_model_00205', array('id' => $_POST['id'])));
        } else {
            $this->render_json(1, yun_at('admin_01349'));
        }
    }
}

?>

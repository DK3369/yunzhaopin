<?php

/* *
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
*/

class toolbox_doc_controller extends adminCommon
{
    // 设置高级搜索功能
    function set_search()
    {
        $search_list[] = array("param" => "status", "name" => '前台显示', "value" => array("1" => "显示", "0" => "不显示"));

        $ad_time = array('1' => '今天', '3' => '最近三天', '7' => '最近七天', '15' => '最近半月', '30' => '最近一个月');

        $search_list[] = array("param" => "end", "name" => '上传日期', "value" => $ad_time);

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

    // 保存
    function save_action()
    {
        $hrM = $this->MODEL('hr');

        if ($_POST['name'] == '') {
            $this->render_json(1, '文档名称不能为空');
        } else if ($_POST['cid'] == '') {
            $this->render_json(1, '请选择文档分类');
        }

        $id = !empty($_POST['id']) ? intval($_POST['id']) : '';

        if (!$id && $_FILES['file']['name'] == '') {
            $this->render_json(1, '请上传文档');
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
            $msg = "文档(ID:{$id})更新";
        } else {
            $post['add_time'] = time();
            $nid = $hrM->addHrInfo($post);
            $msg = "文档(ID:{$nid})添加";
        }

        if ($nid) {
            $this->admin_json(0, $msg . "成功");
        } else {
            $this->render_json(1, $msg . "失败");
        }
    }

    // 删除文档
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

    // 前台显示状态设置
    function show_action()
    {
        $hrM = $this->Model('hr');
        $nid = $hrM->upHrInfo(array('id' => intval($_POST['id'])), array('post' => array('is_show' => intval($_POST['show']))));

        if ($nid) {
            $this->admin_json(0, "文档(ID:{$_POST['id']})显示状态设置成功");
        } else {
            $this->render_json(1, "文档显示状态设置失败");
        }
    }
}

?>
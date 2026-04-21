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

class announcement_controller extends adminCommon
{
    function set_search()
    {
        $ad_time = array('1' => '今天', '3' => '最近三天', '7' => '最近七天', '15' => '最近半月', '30' => '最近一个月');
        $search_list[] = array("param" => "end", "name" => '发布时间', "value" => $ad_time);

        return $search_list;
    }

    function getGroup_action(){
        $search_list = $this->set_search();

        //提取分站内容
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain', $Options = array('needreturn' => true, 'needassign' => true, 'needall' => true));

        $domainList = !empty($domain['Dname']) ? (object)$domain['Dname'] : (object)array();

        $this->render_json(0, 'ok', compact('search_list', 'domainList'));

    }

    // 列表
    function index_action()
    {

        $announcementM = $this->MODEL('announcement');

        if (trim($_POST['keyword'])) {
            $where['title'] = array('like', trim($_POST['keyword']));
        }

        if ($_POST['end']) {
            if ($_POST['end'] == 1) {
                $where['datetime'] = array('>=', strtotime(date("Y-m-d 00:00:00")));
            } else {
                $where['datetime'] = array('>=', strtotime('-' . intval($_POST['end']) . ' day'));
            }
        }

        $pageM = $this->MODEL('page');

        $pages = $pageM->adminPageList('admin_announcement', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
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

            $ListAnnouncement = $announcementM->getList($where, array('field' => 'datetime,did,endtime,id,startime,title,view_num', 'domain' => 1));

            $list = !empty($ListAnnouncement['list']) ? $ListAnnouncement['list'] : [];
        }

        $this->render_json(0, 'ok', compact( 'list', 'total', 'page_sizes', 'limit', 'page'));
    }

    // 获取添加信息
    function save_action()
    {

    }

    // 保存
    function add_action()
    {

        if ($_POST['submit']) {

            $post = $this->post_trim($_POST);

            if (empty($post) || empty($post['title']) || empty($post['keyword']) || empty($post['description'])) {
                $this->render_json(1, '参数错误');
            }

            $announcementM = $this->MODEL('announcement');

            if ($post['startime'] == '') {
                $data['startime'] = date('Y-m-d H:i:s', time());
            }

            $data['title'] = $post['title'];
            $data['did'] = $post['did'];
            $data['keyword'] = $post['keyword'];
            $data['startime'] = $post['startime'] ? $post['startime'] : date('Y-m-d H:i:s', time());
            $data['endtime'] = $post['endtime'];
            $data['description'] = $post['description'];
            $data['content'] = $post['content'];

            if (!empty($post['id'])) {
                $id = intval($post['id']);
                $nid = $announcementM->upInfo(array('id' => $id), $data);
                $msg = '公告(ID:' . $id . ')修改';
            } else {
                $nid = $announcementM->addInfo($data);
                $msg = '公告(ID:' . $nid . ')添加';
            }

            if ($nid) {
                $this->admin_json(0, $msg . '成功');
            } else {
                $this->render_json(1, $msg . '失败');
            }
        }else{
            $announcementM = $this->MODEL('announcement');
            $info = '';
            if ($_POST['id']) {
                $id = intval($_POST['id']);
                $info = $announcementM->getInfo(array('id' => $id));
            }
            //提取分站内容
            $cacheM = $this->MODEL('cache');
            $domain = $cacheM->GetCache('domain', $Options = array('needreturn' => true, 'needassign' => true, 'needall' => true));

            $domainList = !empty($domain['Dname']) ? (object)$domain['Dname'] : (object)array();

            $this->render_json(0, 'ok', compact('info', 'domainList'));
        }
    }

    // 删除
    function del_action()
    {
        $announcementM = $this->Model('announcement');

        $delID = $_POST['id'] ? intval($_POST['id']) : $_POST['del'];

        $return = $announcementM->delAnnouncement($delID);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    // 分配站点
    function checksitedid_action()
    {
        if (empty($_POST['id']) || !isset($_POST['did'])) {
            $this->render_json(1, '参数错误');
        }

        $ids = is_array($_POST['id']) ? pylode(',', $_POST['id']) : intval($_POST['id']);

        $siteDomain = $this->MODEL('site');
        $siteDomain->updDid(array('admin_announcement'), array('id' => array('in', $ids)), array('did' => $_POST['did']));

        $this->admin_json(0, '公告(ID:' . $ids . ')分配站点成功');
    }
}

?>
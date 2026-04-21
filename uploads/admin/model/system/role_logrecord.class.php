<?php

class role_logrecord_controller extends adminCommon
{

    function index_action()
    {
        $logM = $this->MODEL('log');
        if (isset($_POST['end'])) {
            if ($_POST['end'] == '1') {
                $where['ctime'][] = array('>=', strtotime(date("Y-m-d 00:00:00")));
            } else {
                $where['ctime'][] = array('>=', strtotime('-' . (int)$_POST['end'] . 'day'));
            }
        }
        if ($_POST['time']) {
            $time = @explode('~', $_POST['time']);
            $where['PHPYUNBTWSTART_A'] = '';
            $where['ctime'][] = array('>=', strtotime($time[0]));
            $where['ctime'][] = array('<=', strtotime($time[1] . "23:59:59"));
            $where['PHPYUNBTWEND_A'] = '';
        }
        if (trim($_POST['ukeyword'])) {
            $where['username'] = array('like', trim($_POST['ukeyword']));
        }
        if (trim($_POST['keyword'])) {
            $where['content'] = array('like', trim($_POST['keyword']));
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('admin_log', $where, $page, array('limit' => $pageSize));
        $List = array();
        if ($pages['total']) {
            $where['orderby'] = 'id,desc';
            $where['limit'] = $pages['limit'];
            $List = $logM->getAdminLogList($where);
            $cacheM = $this->MODEL('cache');
            $domain = $cacheM->GetCache('domain');
            foreach ($List as &$v) {
                $v['did_name'] = $domain['Dname'][$v['did']];
                $v['ctime_n'] = date('Y-m-d H:i:s', $v['ctime']);
            }
        }
        $data['list'] = $List;
        $data['total'] = intval($pages['total']);
        $data['perPage'] = $pageSize;
        $data['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0, '', $data);
    }

}

?>
<?php

class set_cron_controller extends adminCommon{
    /**
     * 设置-计划任务
     */
	function index_action(){
        $cronM = $this->MODEL('cron');
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('cron', array(), $page, array('limit' => $pageSize));
        $List = array();
        if ($pages['total']) {
            $where['orderby'] = 'id,desc';
            $where['limit'] = $pages['limit'];
            $List = $cronM->getList($where);
            foreach ($List as &$v) {
                $v['nowtime_n'] = $v['nowtime'] ? date('Y-m-d H:i', $v['nowtime']) : '-';
                $v['nexttime_n'] = $v['nexttime'] ? date('Y-m-d H:i', $v['nexttime']) : '-';
                if ($v['type'] == 1) {
                    $v['type_n'] = '每周';
                } elseif ($v['type'] == 2) {
                    $v['type_n'] = '每月';
                } elseif ($v['type'] == 3) {
                    $v['type_n'] = '每天';
                } elseif ($v['type'] == 4) {
                    $v['type_n'] = '每隔N秒';
                } elseif ($v['type'] == 5) {
                    $v['type_n'] = '每隔N分钟';
                }
                if ($v['display'] == 1) {
                    $v['display_n'] = '是';
                } elseif ($v['display'] == 2) {
                    $v['display_n'] = '否';
                }
            }
        }
        $data['list'] = $List;
        $data['total'] = intval($pages['total']);
        $data['perPage'] = $pageSize;
        $data['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'ok', $data);
	}

	// 执行计划任务
    function run_action()
    {
        if (!$_POST['id']) {
            $this->render_json(1, '参数异常');
        }
        $CronM = $this->MODEL('cron');
        $cron = $CronM->getList();
        $CronM->excron($cron, $_POST['id'],'admin');
        $this->admin_json(0, '计划任务(ID:' . $_POST["id"] . ')执行成功！');
    }

    function info_action()
    {
        $arrweek[] = array('label' => '周一', 'value' => '0');
        $arrweek[] = array('label' => '周二', 'value' => '1');
        $arrweek[] = array('label' => '周三', 'value' => '2');
        $arrweek[] = array('label' => '周四', 'value' => '3');
        $arrweek[] = array('label' => '周五', 'value' => '4');
        $arrweek[] = array('label' => '周六', 'value' => '5');
        $arrweek[] = array('label' => '周日', 'value' => '6');
        for ($i = 1; $i <= 31; $i++) {
            $montharr[] = array('label' => $i . "日", 'value' => '' . $i);
        }
        for ($i = 0; $i <= 23; $i++) {
            $hourarr[] = array('label' => $i . "时", 'value' => '' . $i);
        }
        $data['hourarr'] = $hourarr;
        $data['montharr'] = $montharr;
        $data['arrweek'] = $arrweek;
        $row = array();
        if ($_POST["id"]) {
            $row = $this->MODEL('cron')->getInfo(array('id' => $_POST["id"]));
            $row['display'] = $row['display'] == 1;
        }
        $data['row'] = $row;
        $this->render_json(0, 'ok', $data);
    }

    /*
     * 计划任务保存
     */
    function save_action()
    {
        if ($_POST) {
            $CronM = $this->MODEL('cron');
            $_POST['display'] = $_POST['display'] === 'true' ? 1 : 2;
            $return = $CronM->addInfo($_POST);
            if ($return['errcode'] == '9') {
                $this->croncache();
                $this->admin_json(0, $return['msg']);
            } else {
                $this->render_json(1, $return['msg']);
            }
        }
    }

    //生成计划任务缓存文件
    function croncache()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $cacheclass->cron_cache("cron.cache.php");
    }

    /*
     * 删除计划任务
     */
    function del_action()
    {
        if ($_POST["del"]) {
            $CronM = $this->MODEL('cron');
            $return = $CronM->delInfo($_POST["del"]);
            $this->croncache();
            if ($return['errcode'] == 9) {
                $this->admin_json(0, $return['msg']);
            } else {
                $this->render_json(1, $return['msg']);
            }
        }
    }

    /*
     * 任务日志列表
     */
    function cronLog_action(){
        $CronM = $this->MODEL('cron');
        if($_POST['keyword']){
            $keyword = trim($_POST['keyword']);
            $cronwhere = array(
                'name' => array('like', $keyword)
            );
            $crons = $CronM->getList($cronwhere);
            $cids = array();
            foreach ($crons as $key => $value) {
                $cids[] = $value['id'];
            }
            $where['cid'] = array('in', pylode(',',$cids));
        }
        if($_POST['time']){
            $times  =  @explode('~',$_POST['time']);
            $where['PHPYUNBTWSTART'] = '';
            $where['ctime'][] = array('>=', strtotime($times[0]));
            $where['ctime'][] = array('<=', strtotime($times[1] . '23:59:59'));
            $where['PHPYUNBTWEND'] = '';
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('cron_log', $where, $page, array('limit' => $pageSize));
        $List = array();
        //分页数大于0的情况下 执行列表查询
        if($pages['total'] > 0){
            //limit order 只有在列表查询时才需要
            if($_POST['order']){
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            }else{
                $where['orderby'] = array('id,desc');
            }
            $where['limit'] = $pages['limit'];
            $List = $CronM->getCronLogs($where,array('utype' => 'admin'));
        }
        $data['list'] = $List;
        $data['total'] = intval($pages['total']);
        $data['perPage'] = $pageSize;
        $data['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'', $data);
    }
    /*
     * 删除任务日志
     */
    function delLog_action(){
        if (is_array($_POST['del'])){
            $id = pylode(',', $_POST['del']);
            $where = array('id' => array('in', $id));
        }else{
            $where = array('id' => intval($_POST['del']));
        }
        $CronM = $this->MODEL('cron');
        $return = $CronM->delCronlLog($where);
        if ($return['errcode'] == 9) {
            $this->admin_json( 0, $return['msg']);
        } else {
            $this->render_json( 1, $return['msg']);
        }
    }
}
?>
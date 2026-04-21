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
class gongzhao_controller extends adminCommon
{
    function getGroup_action(){
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain', $Options = array('needreturn' => true, 'needassign' => true, 'needall' => true));
        $data['Dname'] = (object)$domain['Dname'];
        $data['today'] = date('Y-m-d');
        $this->render_json(0,'ok', $data);
    }
    function index_action() {
        $gongzhaoM = $this->MODEL('gongzhao');
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
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('gongzhao', $where, $page, array('limit' => $pageSize));
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = 'id';
            }
            $where['limit'] = $pages['limit'];
            $Listgongzhao = $gongzhaoM->getList($where, array('utype' => 'admin'));
        }
        $data['list'] = $Listgongzhao['list'] ? $Listgongzhao['list'] : array();
        $data['total'] = intval($pages['total']);
        $data['perPage'] = $pageSize;
        $data['pageSizes'] = $pages['page_sizes'];
        //提取分站内容

        $this->render_json(0,'ok', $data);
    }

    function add_action()
    {
        if (isset($_POST['add'])){
            // 添加打开弹窗请求，方便权限控制
            $this->render_json(0);
        }
        $gongzhaoM = $this->MODEL('gongzhao');
        $info = $_POST;
        $info = $this->post_trim($info);
        if (!$info['title']) {
            $this->render_json(1, '公招标题不能为空');
        }
        $info['startime'] = $info['startime_n'] ? $info['startime_n'] : date('Y-m-d');
        $info['endtime'] = $info['endtime_n'] ? $info['endtime_n'] : '';
        // 新上传图片文件处理
        foreach ($_FILES['pic'] as $nk => $nv) {
            foreach ($nv as $nkk => $nvv) {
                $pic_file[$nkk][$nk] = $nvv;
            }
        }
        if($_FILES['pic']['tmp_name']){
            $upArrsl = array(
                'file' => $pic_file[0],
                'dir' => 'gongzhao'
            );
            $uploadM = $this->MODEL('upload');
            if(!empty($upArrsl)){
                $picsl = $uploadM->newUpload($upArrsl);
            }
            if ($picsl && !empty($picsl['msg'])){
                $return['msg'] = $picsl['msg'] ? $picsl['msg'] : '上传失败';
                $this->render_json(1, $return['msg']);
            } else {
                if (!empty($picsl['picurl'])){
                    $info['pic'] = $picsl['picurl'];
                }
            }
        }
        if ($info['id']) {
            $nid = $gongzhaoM->upInfo(array('id' => intval($info['id'])), $info);
            if ($nid) {
                $this->admin_json(0, "公招(ID:" . $info['id'] . ")更新成功！");
            } else {
                $this->render_json(1, "公招(ID:" . $info['id'] . ")更新失败！");
            }
        } else {
            $nid = $gongzhaoM->addInfo($info);
            if ($nid) {
                $this->admin_json(0, "公招添加成功！");
            } else {
                $this->render_json(1, "公招添加失败！");
            }
        }
    }

    function del_action()
    {
        $gongzhaoM = $this->Model('gongzhao');
        $delID = $_POST['del'];
        $addArr = $gongzhaoM->delgongzhao($delID);
        if ($addArr['errcode'] == 9) {
            $this->admin_json( 0, $addArr['msg']);
        } else {
            $this->render_json( 1, $addArr['msg']);
        }
    }

    function checksitedid_action()
    {
        if ($_POST['id']) {
            $id = pylode(',', $_POST['id']);
            if ($id) {
                $siteDomain = $this->MODEL('site');
                $siteDomain->updDid(array('gongzhao'), array('id' => array('in', $id)), array('did' => $_POST['did']));
                $this->admin_json(0, '公招(ID:' . $_POST['uid'] . ')分配站点成功！');
            } else {
                $this->render_json(1, '请正确选择需分配数据！');
            }
        } else {
            $this->render_json(1, '参数不全请重试！');
        }
    }

    function whb_action()
    {
        $WhbM = $this->MODEL('whb');
        $gzHb = $WhbM->getWhbList(array('type' => 4, 'isopen' => '1','orderby' => 'sort,desc'));
        $this->render_json(0, '', $gzHb);
    }

    // 公招海报
    function getgongzhaoHb_action()
    {
        $whbM = $this->MODEL('whb');
        if (!$_GET['hb']) {
            $whb = $whbM->getWhb(array('type' => 4, 'isopen' => 1, 'orderby' => 'sort,desc'));
            $_GET['hb'] = $whb['id'];
        }
        $data = array(
            'hb' => $_GET['hb'] ? $_GET['hb'] : 1,
            'id' => $_GET['id']
        );
        echo $whbM->getGongzhaoHb($data);
    }

    // 设置和取消推荐
    function setRec_action(){
        $id = intval($_POST['del']);
        if (isset($_POST['rec']) && intval($_POST['rec'] == 1)){
            $rec = 1;
            $msg = '设置推荐';
        }else{
            $rec = 0;
            $msg = '取消推荐';
        }
        $gongzhaoM = $this->Model('gongzhao');
        $nid = $gongzhaoM->upInfo(array('id' => $id), array('rec' => $rec));
        $nid ? $this->admin_json(0, $msg."成功(ID:" . $id . ")") : $this->render_json(1,$msg."失败(ID:" . $id . ")");
    }
}
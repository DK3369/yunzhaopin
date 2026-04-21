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
class evaluate_controller extends adminCommon
{
    function  getGroup_action(){
        $EvaluateM = $this->MODEL('evaluate');
        $group = $EvaluateM->getList(array('keyid' => '0'));
        $grouparr = array();
        $showgroup = array();
        if($group){
            foreach($group as $val){
                $grouparr[] = array('value' => $val['id'], 'label' => $val['name']);
                $showgroup[$val['id']] = $val['name'];
            }
        }
        $rt['grouparr'] = $grouparr;
        $rt['preview_url'] = Url('evaluate', array('c' => 'exampaper'));
        $rt['show_group'] = $showgroup;
        $this->render_json(0,'ok', $rt);

    }
    //测评试卷列表
    function index_action(){
        $EvaluateM = $this->MODEL('evaluate');
        $showgroup = array();

        $where['keyid'] = array('<>', 0);
        if(trim($_POST['keyword'])){
            $where['name'] = array('like', trim($_POST['keyword']));
        }
        if((int)$_POST['keyid']){
            $where['keyid'] = intval($_POST['keyid']);
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('evaluate_group', $where, $page, array('limit' => $pageSize));
        if($pages['total'] > 0){
            if($_POST['order']){
                $where['orderby'] =	$_POST['t'] . ',' . $_POST['order'];
            }else{
                $where['orderby'] =	'id';
            }
            $where['limit'] = $pages['limit'];
            $rows =	$EvaluateM->getList($where);
            foreach ($rows as $k => $v) {
                $rows[$k]['keyid_n'] = $showgroup[$v['keyid']];
            }
        }

        $rt['list'] = $rows ? $rows : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'ok', $rt);
    }

    //添加修改测评试卷
    function examup_action(){

    }

    //保存添加修改测评试卷
    function add_action(){

        if ($_POST['add']){
            $EvaluateM = $this->MODEL('evaluate');
            $group_all = $EvaluateM->getList(array('keyid' => 0));
            $rt['group_all'] = $group_all;
            $fullscore = 0;
            if($_POST['id']){
                $where['id'] = (int)$_POST['id'];
                $info =	$EvaluateM->getInfo($where, array('pic' => 1));
                $info['fromscore'] = mb_unserialize($info['fromscore']);
                $info['toscore'] = mb_unserialize($info['toscore']);
                $info['comment'] = mb_unserialize($info['comment']);
                $info['describe'] = explode(",", $info['describe']);
                if (count($info['fromscore'])) {
                    foreach ($info['fromscore'] as $k => $v) {
                        $info['pj_arr'][] = array('from' => $v, 'to' => $info['toscore'][$k], 'content' => $info['comment'][$k]);
                    }
                }
                // 问题、选项、试卷总分
                $ask = $EvaluateM->getEvaluateList(array('gid' => (int)$_POST['id'], 'orderby' => 'id,asc'));
                foreach($ask as $key => $val){
                    $tempscore = intval($ask[$key]['score'][0]);
                    foreach($ask[$key]['score'] as $v){
                        if($v > $tempscore){
                            $tempscore = $v;
                        }
                    }
                    $fullscore += $tempscore;
                }
                $rt['info'] = $info;
                $rt['ask'] = $ask;
                $rt['anum'] = count($ask);
            }
            $rt['fullscore'] = $fullscore;
            $this->render_json(0, '', $rt);
        }else{
            $EvaluateM = $this->MODEL('evaluate');
            $info = $this->post_trim($_POST);
            $info['pj_arr'] = json_decode(stripslashes($info['pj_arr']), 1);
            $info['ask_arr'] = json_decode(stripslashes($info['ask_arr']), 1);
            if (!$info['name']) {
                $this->render_json(1, '请填写测评名称！');
            }
            // 新上传图片文件处理
            foreach ($_FILES['pic'] as $nk => $nv) {
                foreach ($nv as $nkk => $nvv) {
                    $pic_file[$nkk][$nk] = $nvv;
                }
            }
            if($_FILES['pic']['tmp_name']){
                $upArr = array(
                    'file' => $pic_file[0],
                    'dir' => 'evaluate'
                );
                $uploadM = $this->MODEL('upload');
                $pic = $uploadM->newUpload($upArr);
                if (!empty($pic['msg'])){
                    $this->render_json(1, $pic['msg']);
                }elseif (!empty($pic['picurl'])){
                    $pictures = $pic['picurl'];
                }
            }
            $from = $to = $content = array();
            foreach($info['pj_arr'] as $v) {
                $from[] = $v['from'];
                $to[] = $v['to'];
                $content[] = $v['content'];
            }
            $data =	array(
                'examtitle' => $info['name'],
                'selectgroup' => $info['keyid'],
                'sort' => $info['sort'],
                'top' => $info['top'],
                'hot' => $info['hot'],
                'recommend' => $info['recommend'],
                'description' => $info['description'],
                'fromscore' => $from,
                'toscore' => $to,
                'comment' => $content
            );
            if(isset($pictures)){
                $data['pic'] = $pictures;
            }
            if($info['id']){
                $scale = $EvaluateM->upInfo(array('id' => intval($info['id'])), $data);
                $nid = $info['id'];
            }else{
                $scale = $EvaluateM->addInfo($data);
                $nid = $scale;
            }
            if ($nid) {
                $ids = array();
                foreach ($info['ask_arr'] as $v) {
                    if ($v['id']) {
                        $scale = $EvaluateM->upEvaQuestion(array('id' => intval($v['id'])), array(
                            'question' => $v['question'],
                            'option' => $v['option'],
                            'score' => $v['score']
                        ));
                        $scale && $this->MODEL('log')->addAdminLog("测评问题(ID:" . $v['id'] . ")修改成功");
                        !in_array($v['id'], $ids) && array_push($ids, $v['id']);
                    } else {
                        $scale = $EvaluateM->addEvaQuestion(array(
                            'question' => $v['question'],
                            'option' => serialize($v['option']),
                            'score' => $v['score'],
                            'examid' => intval($nid)
                        ));
                        if ($scale) {
                            $this->MODEL('log')->addAdminLog("测评问题(ID:" . $scale . ")添加成功");
                        }
                        !in_array($scale, $ids) && array_push($ids, $scale);
                    }
                }
            }
            $EvaluateM->delEvaQuestion($ids, intval($nid), 'notin');// 删除多余的试题
            $scale ? $this->admin_json(0, "操作成功！", array('nid' => $nid)) : $this->render_json(1, "操作失败！");
        }
   }

    //添加,更新问题
    function ajaxsave_action(){
        $EvaluateM = $this->MODEL('evaluate');
        $ask = json_decode(stripslashes($_POST['ask']), 1);
        $ask = $this->post_trim($ask);
        $ids = array();
        foreach ($ask as $v) {
            if ($v['id']) {
                $scale = $EvaluateM->upEvaQuestion(array('id' => intval($v['id'])), array(
                    'question' => $v['question'],
                    'option' => $v['option'],
                    'score' => $v['score']
                ));
                $scale && $this->MODEL('log')->addAdminLog("测评问题(ID:" . $v['id'] . ")修改成功");
                !in_array($v['id'], $ids) && array_push($ids, $v['id']);
            } else {
                $scale = $EvaluateM->addEvaQuestion(array(
                    'question' => $v['question'],
                    'option' => serialize($v['option']),
                    'score' => $v['score'],
                    'examid' => intval($_POST['examid'])
                ));
                if($scale){
                    $this->MODEL('log')->addAdminLog("测评问题(ID:".$scale.")添加成功");
                }
                !in_array($scale, $ids) && array_push($ids, $scale);
            }
        }
        $EvaluateM->delEvaQuestion($ids, intval($_POST['examid']), 'notin');// 删除多余的试题
        $this->render_json(0, '操作成功');
    }

    //删除测评试卷
    function delevaluate_action(){
        $EvaluateM = $this->MODEL('evaluate');
        if($_POST['del']){
            $del = $_POST['del'];
            if (!$del) {
                $this->render_json(1, '请选择您要删除的测评试卷！');
            }
            if(is_array($del)){
                $this->delevagroup($del);
                $this->admin_json(0, '测评试卷(ID:'.@implode(',',$del).')删除成功！');
            } else {
                $result	= $EvaluateM->delevaluate($del);
                isset($result) ? $this->admin_json(0, '测评试卷(ID:' . $del . ')删除成功！') : $this->render_json(1,'删除失败！');
            }
        }
    }

    //删除问题
    function delquestion_action(){
        $EvaluateM = $this->MODEL('evaluate');
        if($_POST['qid']){
            $qid = $_POST['qid'];
            $scale = $EvaluateM->delEvaQuestion($qid);
            isset($scale) ? $this->admin_json(0, '测评问题(ID:'.$qid.')删除成功！') : $this->render_json(1, '删除失败！');
        }
    }

    function delevagroup($ids){
        $EvaluateM = $this->MODEL('evaluate');
        $id = pylode(',', $ids);
        $EvaluateM->delEvaluateGroups($id);
    }

    //测评类别列表
    function group_action(){
        $EvaluateM = $this->MODEL('evaluate');
        $where = array();
        $evaluate_group = $EvaluateM->getList($where, array('orderby' => "sort,desc"));
        if(is_array($evaluate_group)){
            $rootid	= array();
            foreach($evaluate_group as $key => $value){
                if($value['keyid'] != 0){
                    $rootid[$value['keyid']][] = $value['id'];
                }else{
                    $rootid[$value['id']][] = $value['id'];
                }
            }
        }
        if(is_array($rootid) && $rootid){
            foreach($rootid as $k => $v){
                $root_arr = @implode(",", $v);
                $count = $EvaluateM->getEvaluateGroupNum(array('keyid' => $k),array('keyid' => array('in', $root_arr, 'OR')));
                foreach($evaluate_group as $key => $value){
                    if($value['id'] == $k){
                        $evaluate_group[$key]['count'] = $count;
                        $evaluate_group[$key]['roots'] = count($v) - 1;
                    }
                }
            }
        }
        $list = array();
        foreach ($evaluate_group as $k=>$v){
            if ($v['keyid']==0){
                $list[] = $v;
            }
        }
        $this->render_json(0, '', $list);
    }

    //添加测评类别管理
    function addgroup_action(){
        $EvaluateM = $this->MODEL('evaluate');
        if ($_POST['classname'] == "") {
            $this->render_json(1, "请正确填写你的类别！");
        }
        $where['name'] = $_POST['classname'];
        $row = $EvaluateM->getInfo($where);
        if ($row) {
            $this->render_json(1, "已经存在此类别！");
        }
        $nid = $EvaluateM->addEvaluateGroupInfo($_POST);
        if ($nid) {
            $this->admin_json(0, "测评类别添加成功！");
        } else {
            $this->render_json(1, "测评类别添加失败！");
        }
    }

    //点击修改 类别名称、排序
    function ajax_action(){
        $EvaluateM = $this->MODEL('evaluate');
        $whereData['id'] = $_POST['id'];
        $_POST['name'] && $addData['name'] = $_POST['name'];
        $_POST['sort'] && $addData['sort'] = $_POST['sort'];
        $EvaluateM->upEvaluateGroupInfo($addData,$whereData);
        $this->render_json(0, '操作成功');
    }

    //删除测评类别
    function delgroup_action(){
        $EvaluateM = $this->MODEL('evaluate');
        // 删除分组   删除该分组下的所有试卷。
        // 该分组下所有的试卷的id
        $id	= intval($_POST['del']);
        if (!$id) {
            $this->render_json(1, '请选择要删除的数据');
        }
        $titleid = $EvaluateM->getList(array('keyid' => $id));
        $ids = array();
        foreach($titleid as $val){
            $ids[] = $val['id'];
        }
        $this->delevagroup($ids);
        $result	= $EvaluateM->delEvaluateGroup($id);
        isset($result) ? $this->admin_json(0, '测评类别(ID:'.$_GET['id'].')删除成功！') : $this->render_json(1,'删除失败！');
    }

    //测评留言管理列表
    function message_action(){
        $EvaluateM = $this->MODEL('evaluate');
        if(trim($_POST['keyword']) != ""){
            if($_POST['type']=='1'){
                $info =	$EvaluateM->getMemberList(array('username' => array('like', trim($_POST['keyword']))));
                if($info&&is_array($info)){
                    foreach($info as $v){
                        $uids[] = $v['uid'];
                    }
                }
                $where['uid'] =	array('in', pylode(',', $uids));
            }else{
                $where['message'] =	array('like', trim($_POST['keyword']));
            }
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('evaluate_leave_message', $where, $page, array('limit' => $pageSize));
        if($pages['total'] > 0){
            $where['orderby'] =	'id';
            $where['limit'] = $pages['limit'];
            $rows =	$EvaluateM->getMessageList($where);
        }
        $rt['list'] = $rows ? $rows : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'ok', $rt);
    }

    //删除测评留言管理
    function delmsg_action(){
        $EvaluateM = $this->MODEL('evaluate');
        $delID = $_POST['del'];
        $return	= $EvaluateM->delMessage($delID);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    //测评用户记录列表
    function record_action(){
        $EvaluateM = $this->MODEL('evaluate');

        if(trim($_POST['keyword']) != ""){
            if($_POST['type'] == '1' || $_POST['type'] == ''){
                $info =	$EvaluateM->getMemberList(array('username' => array('like', trim($_POST['keyword']))));
                if($info&&is_array($info)){
                    foreach($info as $v){
                        $uids[] = $v['uid'];
                    }
                }
                $where['uid'] =	array('in', pylode(',', $uids));
            }else{
                $exameInfo = $EvaluateM->getList(array('name' => array('like', trim($_POST['keyword']))));
                if($exameInfo&&is_array($exameInfo)){
                    foreach($exameInfo as $v){
                        $examids[] = $v['id'];
                    }
                }
                $where['examid'] = array('in', pylode(',', $examids));
            }
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('evaluate_log', $where, $page, array('limit' => $pageSize));
        if($pages['total'] > 0){
            if($_POST['order']){
                $where['orderby'] =	$_POST['t'] . ',' . $_POST['order'];
            }else{
                $where['orderby'] =	'id';
            }
            $where['limit'] = $pages['limit'];
            $rows =	$EvaluateM->getEvaluateLogList($where);
        }
        $rt['list'] = $rows ? $rows : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'ok', $rt);
    }

    function recordGroup_action(){
        $EvaluateM = $this->MODEL('evaluate');
        $group = $EvaluateM->getList(array('keyid' => array('=', 0)));
        $rt['arr'] = array();
        if($group){
            foreach($group as $val){
                $arr[$val['id']] = $val['name'];
            }
            $rt['arr'] = $arr;
        }
        $this->render_json(0,'ok', $rt);
    }
    //删除测评用户记录
    function delevaluatelog_action(){
        $EvaluateM = $this->MODEL('evaluate');
        $delID = $_POST['del'];
        $return = $EvaluateM->delEvaluateLog($delID);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }
}
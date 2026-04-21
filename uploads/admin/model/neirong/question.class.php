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

class question_controller extends adminCommon
{
    //设置高级搜索功能
    function set_search()
    {
        $search_list[] = array("param" => "is_recom", "name" => '是否推荐', "value" => array("1" => "已推荐", "2" => "未推荐"));
        $search_list[] = array("param" => "status", "name" => '审核状态', "value" => array("0" => "未审核", "1" => "已审核", "2" => "未通过"));

        $ad_time = array('1' => '今天', '3' => '最近三天', '7' => '最近七天', '15' => '最近半月', '30' => '最近一个月');
        $search_list[] = array("param" => "end", "name" => '提问时间', "value" => $ad_time);

        return $search_list;
    }


    function getGroup_action(){
        $search_list = $this->set_search();
        $this->render_json(0, 'ok', compact('search_list'));
    }
    // 列表
    function index_action()
    {
        $askM = $this->MODEL('ask');

        if ($_POST['id']) {//举报页传递参数
            $where['id'] = $_POST['id'];
        }

        if ($_POST['is_recom']) {
            if ($_POST['is_recom'] == 2) {
                $where['is_recom'] = 0;
            } elseif ($_POST['is_recom'] == 1) {
                $where['is_recom'] = 1;
            }
        }

        if (isset($_POST['status']) && $_POST['status'] !== '') {
            $where['state'] = $_POST['status'];
        }

        if ($_POST['end']) {
            if ($_POST['end'] == 1) {
                $where['add_time'] = array('>=', strtotime(date("Y-m-d 00:00:00")));
            } else {
                $where['add_time'] = array('>=', strtotime('-' . (int)$_POST['end'] . 'day'));
            }
        }

        if (trim($_POST['keyword'])) {
            if ($_POST['type'] == '1') {
                $where['title'] = array('like', trim($_POST['keyword']));
            } elseif ($_POST['type'] == "2") {
                $where['nickname'] = array('like', trim($_POST['keyword']));
            }
        }

        $pageM = $this->MODEL('page');

        $pages = $pageM->adminPageList('question', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
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

            $list = $askM->getList($where, array('utype' => 'admin'));
        }

        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }

    // 是否推荐
    function recommend_action()
    {
        if (empty($_POST['id']) || !isset($_POST['rec'])) {
            $this->render_json(1, '参数错误');
        }

        $askM = $this->Model('ask');

        $id = intval($_POST['id']);
        $nid = $askM->upRecommend(array('id' => $id), array('is_recom' => intval($_POST['rec'])));

        if ($nid) {
            $this->admin_json(0, '问答(ID:' . $id . ')推荐设置成功');
        } else {
            $this->render_json(1, '问答推荐设置失败');
        }
    }

    // 获取添加信息
    function add_action()
    {
        $askM = $this->Model('ask');

        $info = '';
        if (!empty($_POST['id'])) {
            $id = intval($_POST['id']);
            $info = $askM->getInfo($id);
        }

        $classList = $askM->getQclassList(array('orderby' => 'sort,desc'), array('field' => 'id,name,pid'));

        if ($classList) {
            $newClassList = array();
            foreach ($classList as $key => $val) {
                if ($val['pid'] == 0) {
                    if (isset($newClassList[$val['id']])) { // 先增加子类的情况下需要把父类合并进去
                        $newClassList[$val['id']] = array_merge($val, $newClassList[$val['id']]);
                    } else {
                        $newClassList[$val['id']] = $val;
                    }
                } else {
                    $newClassList[$val['pid']]['children'][] = $val;
                }
            }
            $classList = array_values($newClassList);
        }

        $this->render_json(0, 'ok', compact('info', 'classList'));
    }

    // 获取子类 - 暂时无用
    function get_class_action()
    {
        $askM = $this->Model('ask');

        if ($_POST['pid']) {
            $q_class = $askM->getQclassList(array('pid' => $_POST['pid'], 'orderby' => 'sort,desc'), array('field' => 'id,name,pid'));

            if ($q_class[0]) {
                $html = '';
                foreach ($q_class as $v) {
                    $html .= '<option value="' . $v['id'] . '">' . $v['name'] . '</option>';
                }

                echo $html;
            } else {
                echo $html = "<div class=\"yun_admin_select_box_list\">该分类下暂无子类！</div>";
            }
        }
    }

    // 保存
    function save_action()
    {
        $post = $this->post_trim($_POST);
        if (empty($post['title']) || empty($post['cid'])) {
            $this->render_json(1, '参数错误');
        }

        $askM = $this->Model('ask');

        $id = intval($post['id']);
        $post['content'] = str_replace("&amp;", "&", $_POST['content']);
        $nbid = $askM->upAskInfo(array('id' => $id), $post);

        if ($nbid) {
            $this->admin_json(0, "问答(ID:" . $id . ")更新成功");
        } else {
            $this->render_json(1, '问答更新失败');
        }
    }

    // 删除问答
    function del_action()
    {
        if (empty($_POST['del']) && empty($_POST['id'])) {
            $this->render_json(1, '参数错误');
        }

        if (!empty($_POST['del'])) { // 批量删除
            $ids = pylode(',', $_POST['del']);
        } else {// 单个删除
            $ids = $_POST['id'];
        }

        $askM = $this->MODEL('ask');

        $return = $askM->delquestion($ids, array('utype' => 'admin'));

        if ($return) {
            $this->admin_json(0, "问答(ID:{$ids})删除成功");
        } else {
            $this->render_json(1, '问答删除失败');
        }
    }

    //修改审核状态
    function status_action()
    {
        if (empty($_POST['id']) || empty($_POST['status'])) {
            $this->render_json(1, '参数错误');
        }

        $askM = $this->MODEL('ask');

        $id = $_POST['id'];

        $data['state'] = $_POST['status'];
        $data['lastupdate'] = time();

        $nid = $askM->upStatusInfo($id, $where = array(), $data);

        $ids = pylode(',', $id);
        $List = $askM->getList(array('id' => array('in', $ids)), array('field' => '`id`,`uid`,`title`'));

        /* 消息前缀 */
        $tagName = '问答';

        if (!empty($List)) {
            foreach ($List as $v) {
                $uids[] = $v['uid'];

                /* 处理审核信息 */
                if ($_POST['status'] == 2) {
                    $statusInfo = $tagName . ':<a href="answertpl,' . $v['id'] . '">' . $v['title'] . '</a>,审核未通过';
                    if ($_POST['statusbody']) {
                        $statusInfo .= ' , 原因：' . $_POST['statusbody'];
                    }

                    $msg[$v['uid']][] = $statusInfo;
                } elseif ($_POST['status'] == 1) {
                    $msg[$v['uid']][] = $tagName . ':<a href="answertpl,' . $v['id'] . '">' . $v['title'] . '</a>,已审核通过';
                }
            }
            //发送系统通知
            if (!empty($_POST['status'])) {
                $sysmsgM = $this->MODEL('sysmsg');
                $sysmsgM->addInfo(array('uid' => $uids, 'content' => $msg));
            }
        }

        if ($nid) {
            $this->admin_json(0, "问答审核(ID:{$ids})设置成功");
        } else {
            $this->render_json(1, '问答审核设置失败');
        }
    }

    //获取该提问的所有回答
    function getanswer_action()
    {
        $askM = $this->MODEL('ask');

        $id = intval($_POST['id']);

        $awhere = array('orderby' => 'add_time,desc');

        $ques = '';
        if ($id) {
            $ques = $askM->getInfo($id);
            $awhere['qid'] = $id;
        }

        if (isset($_POST['status'])) {
            $awhere['status'] = $_POST['status'];
        }

        if ($_POST['aid']) {
            $awhere['id'] = $_POST['aid'];
            $list = $askM->getAnswersList($awhere, array('utype' => 'admin'));
        } else {
            $list = $askM->getAnswersList($awhere, array('utype' => 'admin'));
        }

        $this->yunset("qid", $_POST['id']);

        $this->render_json(0, 'ok', compact('list', 'ques'));
    }

    /**
     * 审核
     */
    function statusAnswer_action()
    {
        if (empty($_POST['id']) || empty($_POST['status'])) {
            $this->render_json(1, '参数错误');
        }

        $askM = $this->MODEL('ask');

        $statusData = array(
            'status' => intval($_POST['status']),
            'statusbody' => trim($_POST['statusbody'])
        );

        $return = $askM->statusAnswer($_POST['id'], $statusData);

        if ($return['errcode'] == 0) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    //更新 回答
    function save_answer_action()
    {
        if (empty($_POST['id']) || empty($_POST['content'])) {
            $this->render_json(1, '参数错误');
        }

        $askM = $this->MODEL('ask');

        $data['support'] = intval($_POST['support']);
        $data['content'] = str_replace("&amp;", "&", $_POST['content']);

        $id = intval($_POST['id']);
        $return = $askM->upAnswerInfo(array('id' => $id), $data);

        if ($return) {
            $this->admin_json(0, "回答(ID:{$id})修改成功");
        } else {
            $this->render_json(1, '回答修改失败');
        }
    }

    // 删除用户回答
    function delanswer_action()
    {
        if ((empty($_POST['del']) && empty($_POST['id'])) || empty($_POST['qid'])) {
            $this->render_json(1, '参数错误');
        }

        if (!empty($_POST['del'])) { // 批量删除
            $id = $_POST['del'];
            $nums = count($id);
            $ids = pylode(',', $id);
        } else {// 单个删除
            $nums = 1;
            $id = $ids = intval($_POST['id']);
        }

        $askM = $this->MODEL('ask');

        $result = $askM->delAnswer('', $id);

        if ($result['errcode'] == 9) {
            $askM->upStatusInfo(intval($_POST['qid']), '', array('answer_num' => array('-', $nums)));
            $this->admin_json(0, '问答回答(ID:' . $ids . ')删除成功');
        } else {
            $this->render_json(1, '问答回答删除失败');
        }
    }

    // 获得针对某一提问回答的评论
    function getcomment_action()
    {
        $askM = $this->MODEL('ask');

        $cwhere = array('orderby' => 'id,desc');

        if ($_POST['aid']) {
            $cwhere['aid'] = intval($_POST['aid']);
        } else if ($_POST['id']) {
            $cwhere['id'] = intval($_POST['id']);
        }

        if (isset($_POST['status'])) {
            $cwhere['status'] = $_POST['status'];
        }
        $list = $askM->getCommentsList($cwhere, array('utype' => 'admin'));

        if ($_POST['id']) {
            $aid = intval($_POST['id']);
            $answer = $askM->getCommentBackQuestion($aid);
        }

        $this->render_json(0, 'ok', compact('list', 'answer'));
    }

    /**
     * 评论审核
     */
    function statusAnswerReview_action()
    {
        if (empty($_POST['id']) || empty($_POST['status'])) {
            $this->render_json(1, '参数错误');
        }

        $askM = $this->MODEL('ask');

        $statusData = array(
            'status' => intval($_POST['status']),
            'statusbody' => trim($_POST['statusbody'])
        );

        $return = $askM->statusAnswerReview($_POST['id'], $statusData);

        if ($return['errcode'] == 0) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    // 修改评论
    function save_review_action()
    {
        if (empty($_POST['id']) || empty($_POST['content'])) {
            $this->render_json(1, '参数错误');
        }

        $askM = $this->MODEL('ask');

        $id = intval($_POST['id']);
        $return = $askM->upReview(array('id' => $id), $_POST);

        if ($return) {
            $this->admin_json(0, '问答评论(ID:' . $id . ')修改成功');
        } else {
            $this->render_json(1, '问答评论修改失败');
        }
    }

    // 删除针对某回答的评论
    function delreview_action()
    {
        $askM = $this->MODEL('ask');

        $delID = $_POST['id'] ? intval($_POST['id']) : $_POST['del'];

        $return = $askM->delReview($delID);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, '问答评论(ID:' . pylode(',', $delID) . ')删除成功');
        } else {
            $this->render_json(1, '问答评论删除失败');
        }
    }

    // 问答设置
    function config_action()
    {
        $config = $this->config;
        $config = array(
            'sy_day_ask_num' => $config['sy_day_ask_num'],
            'sy_ip_ask_num' => $config['sy_ip_ask_num'],
            'ask_check' => $config['ask_check'],
            'answer_check' => $config['answer_check'],
            'answer_review_check' => $config['answer_review_check'],
            'sy_friend_icon_n' => checkpic($config['sy_friend_icon'])
        );

        $this->render_json(0, 'ok', compact('config'));
    }

    // 问答设置保存
    function configSave_action()
    {
        if (empty($_POST)) {
            $this->render_json(1, '参数错误');
        }

        if ($_FILES['sy_friend_icon']['tmp_name']) {
            $uploadM = $this->MODEL('upload');

            $upData['file'] = $_FILES['sy_friend_icon'];
            $upData['dir'] = 'logo';

            $upRes = $uploadM->newUpload($upData);
            if ($upRes && !empty($upRes['msg'])) {
                $this->render_json(1, $upRes['msg']);
            } else {
                $configData['sy_friend_icon'] = $upRes['picurl'];
            }
        }

        $configM = $this->MODEL('config');

        $configData['sy_day_ask_num'] = $_POST['sy_day_ask_num'];
        $configData['sy_ip_ask_num'] = $_POST['sy_ip_ask_num'];
        $configData['ask_check'] = trim($_POST['ask_check']);
        $configData['answer_check'] = trim($_POST['answer_check']);
        $configData['answer_review_check'] = trim($_POST['answer_review_check']);

        $configM->setConfig($configData);

        $this->web_config();

        $this->admin_json(0, '问答设置配置修改成功');
    }
}

?>
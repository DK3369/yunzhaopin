<?php



class question_controller extends adminCommon
{
    // Configure advanced search filters.
    function set_search()
    {
        $search_list[] = array("param" => "is_recom", "name" => 'admin_00231', "value" => array("1" => 'admin_01339', "2" => 'admin_system_00448'));
        $search_list[] = array("param" => "status", "name" => 'wap_com_00406', "value" => array("0" => 'wap_user_00166', "1" => 'wap_user_00165', "2" => 'wap_user_00167'));

        $ad_time = array('1' => 'common_01940', '3' => 'admin_user_00179', '7' => 'admin_user_00178', '15' => 'admin_user_00180', '30' => 'admin_user_00175');
        $search_list[] = array("param" => "end", "name" => 'admin_00251', "value" => $ad_time);

        return $search_list;
    }


    function getGroup_action(){
        $search_list = $this->set_search();
        $this->render_json(0, 'ok', compact('search_list'));
    }
    // List questions.
    function index_action()
    {
        $askM = $this->MODEL('ask');

        if ($_POST['id']) { // Parameter passed from the report page.
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

    // Toggle recommendation.
    function recommend_action()
    {
        if (empty($_POST['id']) || !isset($_POST['rec'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $askM = $this->Model('ask');

        $id = intval($_POST['id']);
        $nid = $askM->upRecommend(array('id' => $id), array('is_recom' => intval($_POST['rec'])));

        if ($nid) {
            $this->admin_json(0, yun_t('admin_model_00007', array('{id}' => $id)));
        } else {
            $this->render_json(1, yun_at('admin_01340'));
        }
    }

    // Load add/edit data.
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
                    if (isset($newClassList[$val['id']])) { // Merge the parent when child records were added first.
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

    // Load child categories; currently unused.
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
                echo $html = '<div class="yun_admin_select_box_list">' . yun_t('admin_model_00015') . '</div>';
            }
        }
    }

    // Save question.
    function save_action()
    {
        $post = $this->post_trim($_POST);
        if (empty($post['title']) || empty($post['cid'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $askM = $this->Model('ask');

        $id = intval($post['id']);
        $post['content'] = str_replace("&amp;", "&", $_POST['content']);
        $nbid = $askM->upAskInfo(array('id' => $id), $post);

        if ($nbid) {
            $this->admin_json(0, yun_t('admin_model_00008', array('{id}' => $id)));
        } else {
            $this->render_json(1, yun_at('admin_neirong_00025'));
        }
    }

    // Delete questions.
    function del_action()
    {
        if (empty($_POST['del']) && empty($_POST['id'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        if (!empty($_POST['del'])) { // Batch delete.
            $ids = pylode(',', $_POST['del']);
        } else { // Single delete.
            $ids = $_POST['id'];
        }

        $askM = $this->MODEL('ask');

        $return = $askM->delquestion($ids, array('utype' => 'admin'));

        if ($return) {
            $this->admin_json(0, yun_t('admin_model_00009', array('{ids}' => $ids)));
        } else {
            $this->render_json(1, yun_at('admin_01341'));
        }
    }

    // Update review status.
    function status_action()
    {
        if (empty($_POST['id']) || empty($_POST['status'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $askM = $this->MODEL('ask');

        $id = $_POST['id'];

        $data['state'] = $_POST['status'];
        $data['lastupdate'] = time();

        $nid = $askM->upStatusInfo($id, $where = array(), $data);

        $ids = pylode(',', $id);
        $List = $askM->getList(array('id' => array('in', $ids)), array('field' => '`id`,`uid`,`title`'));

        if (!empty($List)) {
            foreach ($List as $v) {
                $uids[] = $v['uid'];

                // Build review notification content.
                $titleLink = '<a href="answertpl,' . $v['id'] . '">' . $v['title'] . '</a>';
                if ($_POST['status'] == 2) {
                    if ($_POST['statusbody']) {
                        $statusInfo = yun_t('admin_model_00017', array('{title_link}' => $titleLink, '{reason}' => $_POST['statusbody']));
                    } else {
                        $statusInfo = yun_t('admin_model_00016', array('{title_link}' => $titleLink));
                    }

                    $msg[$v['uid']][] = $statusInfo;
                } elseif ($_POST['status'] == 1) {
                    $msg[$v['uid']][] = yun_t('admin_model_00018', array('{title_link}' => $titleLink));
                }
            }
            // Send system notifications.
            if (!empty($_POST['status'])) {
                $sysmsgM = $this->MODEL('sysmsg');
                $sysmsgM->addInfo(array('uid' => $uids, 'content' => $msg));
            }
        }

        if ($nid) {
            $this->admin_json(0, yun_t('admin_model_00010', array('{ids}' => $ids)));
        } else {
            $this->render_json(1, yun_at('admin_01342'));
        }
    }

    // Load all answers for the question.
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
     * Review answer.
     */
    function statusAnswer_action()
    {
        if (empty($_POST['id']) || empty($_POST['status'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
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

    // Update answer.
    function save_answer_action()
    {
        if (empty($_POST['id']) || empty($_POST['content'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $askM = $this->MODEL('ask');

        $data['support'] = intval($_POST['support']);
        $data['content'] = str_replace("&amp;", "&", $_POST['content']);

        $id = intval($_POST['id']);
        $return = $askM->upAnswerInfo(array('id' => $id), $data);

        if ($return) {
            $this->admin_json(0, yun_t('admin_model_00011', array('{id}' => $id)));
        } else {
            $this->render_json(1, yun_at('admin_01343'));
        }
    }

    // Delete user answers.
    function delanswer_action()
    {
        if ((empty($_POST['del']) && empty($_POST['id'])) || empty($_POST['qid'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        if (!empty($_POST['del'])) { // Batch delete.
            $id = $_POST['del'];
            $nums = count($id);
            $ids = pylode(',', $id);
        } else { // Single delete.
            $nums = 1;
            $id = $ids = intval($_POST['id']);
        }

        $askM = $this->MODEL('ask');

        $result = $askM->delAnswer('', $id);

        if ($result['errcode'] == 9) {
            $askM->upStatusInfo(intval($_POST['qid']), '', array('answer_num' => array('-', $nums)));
            $this->admin_json(0, yun_t('admin_model_00012', array('{ids}' => $ids)));
        } else {
            $this->render_json(1, yun_at('admin_01344'));
        }
    }

    // Load comments for a question answer.
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
     * Review answer comment.
     */
    function statusAnswerReview_action()
    {
        if (empty($_POST['id']) || empty($_POST['status'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
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

    // Update comment.
    function save_review_action()
    {
        if (empty($_POST['id']) || empty($_POST['content'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $askM = $this->MODEL('ask');

        $id = intval($_POST['id']);
        $return = $askM->upReview(array('id' => $id), $_POST);

        if ($return) {
            $this->admin_json(0, yun_t('admin_model_00013', array('{id}' => $id)));
        } else {
            $this->render_json(1, yun_at('admin_01345'));
        }
    }

    // Delete comments for an answer.
    function delreview_action()
    {
        $askM = $this->MODEL('ask');

        $delID = $_POST['id'] ? intval($_POST['id']) : $_POST['del'];

        $return = $askM->delReview($delID);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, yun_t('admin_model_00014', array('{ids}' => pylode(',', $delID))));
        } else {
            $this->render_json(1, yun_at('admin_01346'));
        }
    }

    // Q&A settings.
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

    // Save Q&A settings.
    function configSave_action()
    {
        if (empty($_POST)) {
            $this->render_json(1, yun_at('wap_com_00228'));
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

        $this->admin_json(0, 'admin_01347');
    }
}

?>

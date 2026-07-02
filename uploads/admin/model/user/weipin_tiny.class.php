<?php



class weipin_tiny_controller extends adminCommon
{
    //设置高级搜索功能
    function set_search()
    {
        $cacheM = $this->MODEL('cache');
        $cache = $cacheM->GetCache(array('user'));
        foreach ($cache['userdata']['user_word'] as $k => $v) {
            $ltar[$v] = $cache['userclass_name'][$v];
        }

        $search_list[] = array("param" => "sex", "name" => 'admin_01323', "value" => $cache['user_sex']);
        $search_list[] = array("param" => "exp", "name" => 'wap_00526', "value" => $ltar);
        $search_list[] = array("param" => "status", "name" => 'wap_com_00406', "value" => array("1" => 'wap_user_00165', "2" => 'wap_user_00166'));
        $search_list[] = array("param" => "time", "name" => 'admin_user_weipin_00030', "value" => array('1' => 'common_01940', '3' => 'admin_user_00179', '7' => 'admin_user_00178', '15' => 'admin_user_00180', '30' => 'admin_user_00175'));

        return $search_list;
    }

    // 普工简历列表
    function index_action()
    {
        $tinyM = $this->MODEL('tiny');

        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                $where['username'] = array('like', $keyword);
            } elseif ($keytype == 2) {
                $where['job'] = array('like', $keyword);
            } elseif ($keytype == 3) {
                $where['mobile'] = array('like', $keyword);
            } elseif ($keytype == 4) {
                $where['qq'] = array('like', $keyword);
            }
        }

        if ($_POST['sex']) {
            $where['sex'] = intval($_POST['sex']);
        }
        if ($_POST['time']) {
            if ($_POST['time'] == '1') {
                $where['time'] = array('>=', strtotime('today'));
            } else {
                $where['ctime'] = array('>=', strtotime('-' . intval($_POST['time']) . ' day'));
            }
        }
        if ($_POST['exp']) {
            $where['exp'] = intval($_POST['exp']);
        }
        if ($_POST['status']) {
            $where['status'] = intval($_POST['status']) == 2 ? 0 : intval($_POST['status']);
        }

        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('resume_tiny', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('status,asc', 'lastupdate,desc');
            }
            $where['limit'] = $pages['limit'];

            $List = $tinyM->getResumeTinyList($where);
        }
        $list = !empty($List['list']) ? $List['list'] : array();


        $this->render_json(0, 'ok', compact( 'list', 'total', 'page_sizes', 'limit', 'page'));
    }
    public function  getCacheData_action(){
        $search_list = $this->set_search();
        $this->render_json(0,'',compact('search_list'));
    }
    // 普工简历-审核
    function status_action()
    {
        if (empty($_POST['id'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $tinyM = $this->MODEL('tiny');
        $status = intval($_POST['status']);

        $nid = $tinyM->setResumeTinyStatus($_POST['id'], array('status' => $status));
        if ($nid) {
            $this->render_json(0, yun_at('admin_01324')); // The model adds the admin log, so return through render_json here.
        } else {
            $this->render_json(1, yun_at('admin_user_00113'));
        }
    }

    // 普工简历-删除
    function del_action()
    {
        if (empty($_POST['del'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $tinyM = $this->MODEL('tiny');

        $return = $tinyM->delResumeTiny($_POST['del']);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    // 获取编辑需要的缓存
    public function getCache_action()
    {
        $cacheM = $this->MODEL('cache');
        $cacheData = $cacheM->GetCache(array('user'));

        /**
         * @var $userdata
         * @var $userclass_name
         */
        extract($cacheData);

        $user_word = array();
        if (!empty($userdata['user_word'])) {
            foreach ($userdata['user_word'] as $uwval) {
                $user_word[] = ['id' => $uwval, 'name' => $userclass_name[$uwval]];
            }
        }
        $search_list = $this->set_search();
        $domain = $cacheM->GetCache('domain');
        $dname = (object)$domain['Dname'];
        $this->render_json(0, 'ok', compact('user_sex', 'user_word','search_list','dname'));
    }

    // 普工简历-保存
    function save_action()
    {
        $tinyM = $this->MODEL('tiny');

        $post = array(
            'username' => $_POST['username'],
            'sex' => $_POST['sex'],
            'exp' => $_POST['exp'],
            'job' => $_POST['job'],
            'mobile' => $_POST['mobile'],
            'provinceid' => $_POST['provinceid'],
            'cityid' => $_POST['cityid'],
            'three_cityid' => $_POST['three_cityid'],
            'production' => $_POST['production'],
            'password' => $_POST['password'],
            'status' => 1
        );
        $data = array(
            'id' => (int)$_POST['id'],
            'post' => $post,
            'type' => 'admin'
        );
        $return = $tinyM->addResumeTinyInfo($data, 'admin');
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    // General worker resume statistics.
    function tinyNum_action()
    {
        $MsgNum = $this->MODEL("msgNum");
        echo $MsgNum->tinyNum();
    }

    /**
     * @desc General worker site assignment.
     */
    function checksitedid_action()
    {
        if (empty($_POST['id']) || !isset($_POST['did'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $ids = pylode(',', $_POST['id']);
        $did = intval($_POST['did']);

        $siteM = $this->MODEL('site');

        $didData = array('did' => $did);

        $siteM->updDid(array('resume_tiny'), array('id' => array('in', $ids)), $didData);

        $this->admin_json(0, yun_t('admin_model_00142', array('ids' => $ids)));
    }

    /**
     * Refresh general worker resume.
     */
    function refresh_action()
    {
        $id = intval($_POST['id']);
        $tinyM = $this->MODEL('tiny');

        $return = $tinyM->refreshResume($id);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    // 普工设置
    function set_action()
    {
        $config = $this->config;
        $config = array(
            'sy_tiny' => isset($config['sy_tiny']) ? $config['sy_tiny'] : '',
            'sy_tiny_totalnum' => isset($config['sy_tiny_totalnum']) ? $config['sy_tiny_totalnum'] : '',
            'user_wjl' => isset($config['user_wjl']) ? $config['user_wjl'] : 1,
            'user_wjl_link' => isset($config['user_wjl_link']) ? $config['user_wjl_link'] : 0,
        );

        $this->render_json(0, 'ok', compact('config'));
    }

    // 普工设置-保存
    function tinyset_action()
    {
        $_POST = $this->post_trim($_POST);
        if (empty($_POST)) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $configM = $this->MODEL("config");

        $configData['sy_tiny'] = isset($_POST['sy_tiny']) ? intval($_POST['sy_tiny']) : '';
        $configData['sy_tiny_totalnum'] = isset($_POST['sy_tiny_totalnum']) ? intval($_POST['sy_tiny_totalnum']) : '';
        $configData['user_wjl'] = isset($_POST['user_wjl']) ? intval($_POST['user_wjl']) : 1;
        $configData['user_wjl_link'] = isset($_POST['user_wjl_link']) ? intval($_POST['user_wjl_link']) : 0;

        $configM->setConfig($configData);

        $this->web_config();
        $this->admin_json(0, 'admin_user_00112');
    }
}

?>

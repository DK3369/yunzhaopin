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

class weipin_once_controller extends adminCommon
{
    //设置高级搜索功能
    function set_search()
    {
        $search_list[] = array("param" => "status", "name" => '审核状态', "value" => array("1" => "已审核", "3" => "未审核", "2" => "已过期"));
        $search_list[] = array("param" => "time", "name" => '发布时间', "value" => array('1' => '今天', '3' => '最近三天', '7' => '最近七天', '15' => '最近半月', '30' => '最近一个月'));

        return $search_list;
    }

    // 列表
    function index_action()
    {
        $onceM = $this->MODEL('once');

        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 2) {
                $where['title'] = array('like', $keyword);
            } elseif ($keytype == 3) {
                $where['phone'] = array('like', $keyword);
            } elseif ($keytype == 4) {
                $where['linkman'] = array('like', $keyword);
            } elseif ($keytype == 5) {
                $where['companyname'] = array('like', $keyword);
            }

        }
        if ($_POST['status']) {
            $status = intval($_POST['status']);
            if ($status == 1) {
                $where['status'] = $status;
                $where['edate'] = array(">", time());
            } elseif ($status == 3) {
                $where['status'] = 0;
                $where['edate'] = array(">", time());
            } elseif ($status == 2) {
                $where['edate'] = array("<", time());
            }
        }

        if ($_POST['time']) {
            if ($_POST['time'] == '1') {
                $where['ctime'] = array('>=', strtotime('today'));
            } else {
                $where['ctime'] = array('>=', strtotime('-' . intval($_POST['time']) . ' day'));
            }
        }

        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('once_job', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = array();
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('ctime,desc');
            }
            $where['limit'] = $pages['limit'];

            $list = $onceM->getOnceList($where, array('utype' => 'admin'));
            foreach ($list as $key => $val) {
                $list[$key]['once_url'] = Url('once', array('c' => 'show', 'id' => $val['id']));
            }
        }

        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }
    public function  getCache_action(){
        $search_list = $this->set_search();
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain');
        $dname = (object)$domain['Dname'];
        $this->render_json(0,'',compact('search_list','dname'));
    }
    // 数据统计
    function onceNum_action()
    {
        $MsgNum = $this->MODEL('msgNum');
        echo $MsgNum->onceNum();
    }

    /**
     * @desc  店铺招聘分站  分站设置
     */
    function checksitedid_action()
    {
        $_POST = $this->post_trim($_POST);
        if (empty($_POST['id'])) {
            $this->render_json(1, '参数错误');
        }

        $did = intval($_POST['did']);
        $ids = pylode(',', $_POST['id']);

        $siteM = $this->MODEL('site');

        $didData = array('did' => $did);

        $siteM->updDid(array('once_job'), array('id' => array('in', $ids)), $didData);

        $this->admin_json(0, '店铺招聘(ID:' . $ids . ')分配站点成功');
    }


    // 审核功能
    function status_action()
    {
        if (empty($_POST['id'])) {
            $this->render_json(1, '参数错误');
        }

        $onceM = $this->MODEL('once');

        $status = intval($_POST['status']);

        $return = $onceM->setOnceStatus($_POST['id'], array('status' => $status));

        $this->render_json(0, '店铺招聘审核设置成功', $return);
    }

    // 批量延期
    function ctime_action()
    {
        if (empty($_POST['id']) || empty($_POST['endtime'])) {
            $this->render_json(1, '参数错误');
        }

        $onceM = $this->MODEL('once');

        $return = $onceM->setOnceCtime(pylode(',', $_POST['id']), array('endtime' => $_POST['endtime']));

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    //查询修改信息
    function edit_action()
    {
        $info = '';
        if ($_POST['id']) {
            $onceM = $this->MODEL('once');
            $id = intval($_POST['id']);
            $info = $onceM->getOnceInfo(array('id' => $id));
        }

        $this->render_json(0, 'ok', compact('info'));
    }

    //保存修改信息
    function save_action()
    {
        $onceM = $this->MODEL("once");

        $edate = strtotime("+" . (int)$_POST['edate'] . " days");

        $post = array(
            'title' => $_POST['title'],
            'companyname' => $_POST['companyname'],
            'linkman' => $_POST['linkman'],
            'phone' => $_POST['phone'],
            'provinceid' => $_POST['provinceid'],
            'cityid' => $_POST['cityid'],
            'three_cityid' => $_POST['three_cityid'],
            'address' => $_POST['address'],
            'require' => $_POST['require'],
            'edate' => $edate,
            'salary' => $_POST['salary'],
            'password' => $_POST['password'],
            'yyzz' => $_FILES['yyzz'],
            'file' => $_FILES['file'],
            'status' => 1,
            'did' => $this->config['did'],
            'ctime' => time()
        );

        $data = array(
            'id' => (int)$_POST['id'],
            'post' => $post,
            'type' => 'admin'
        );
        $return = $onceM->addOnceInfo($data, 'admin');
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(0, $return['msg']);
        }
    }

    // 店铺职位刷新
    function refresh_job_action()
    {
        $_POST = $this->post_trim($_POST);
        if (empty($_POST['id'])) {
            $this->render_json(1, '参数错误');
        }

        $onceM = $this->MODEL('once');

        $return = $onceM->refresh_job($_POST['id']);

        if ($return['errcode'] === 0) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    //删除功能
    function del_action()
    {
        if (empty($_POST['del'])) {
            $this->render_json(1, '参数错误');
        }

        $onceM = $this->Model('once');

        $return = $onceM->delOnce($_POST['del']);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    // 店铺设置
    function set_action()
    {
        $config = $this->config;
        $config = array(
            'sy_once' => isset($config['sy_once']) ? $config['sy_once'] : '',
            'sy_once_totalnum' => isset($config['sy_once_totalnum']) ? $config['sy_once_totalnum'] : '',
            'user_wzp_link' => isset($config['user_wzp_link']) ? $config['user_wzp_link'] : 0,
            'com_fast_status' => isset($config['com_fast_status']) ? $config['com_fast_status'] : 1,
            'sy_once_yyzz' => isset($config['sy_once_yyzz']) ? $config['sy_once_yyzz'] : 2,
            'com_xin' => isset($config['com_xin']) ? $config['com_xin'] : '',
            'sy_once_icon_n' => isset($config['sy_once_icon']) ? checkpic($config['sy_once_icon']) : ''
        );

        $this->render_json(0, 'ok', compact('config'));
    }

    // 店铺设置 - 保存
    function onceset_action()
    {
        $_POST = $this->post_trim($_POST);
        if (empty($_POST)) {
            $this->render_json(1, '参数错误');
        }

        if ($_FILES['sy_once_icon']['tmp_name']) {
            $uploadM = $this->MODEL('upload');

            $upData['file'] = $_FILES['sy_once_icon'];
            $upData['dir'] = 'logo';

            $upRes = $uploadM->newUpload($upData);
            if ($upRes && !empty($upRes['msg'])) {
                $this->render_json(1, $upRes['msg']);
            } else {
                $configData['sy_once_icon'] = $upRes['picurl'];
            }
        }

        $configM = $this->MODEL('config');

        $configData['sy_once'] = isset($_POST['sy_once']) ? intval($_POST['sy_once']) : '';
        $configData['sy_once_totalnum'] = isset($_POST['sy_once_totalnum']) ? intval($_POST['sy_once_totalnum']) : '';
        $configData['user_wzp_link'] = isset($_POST['user_wzp_link']) ? intval($_POST['user_wzp_link']) : 0;
        $configData['com_fast_status'] = isset($_POST['com_fast_status']) ? intval($_POST['com_fast_status']) : 1;
        $configData['sy_once_yyzz'] = isset($_POST['sy_once_yyzz']) ? intval($_POST['sy_once_yyzz']) : 2;
        $configData['com_xin'] = isset($_POST['com_xin']) ? intval($_POST['com_xin']) : '';
        $configData['sy_once_icon_n'] = isset($_POST['sy_once_icon']) ? checkpic($_POST['sy_once_icon']) : '';

        $configM->setConfig($configData);

        $this->web_config();
        $this->admin_json(0, '店铺设置保存成功');
    }

    // 招聘时长
    function price_gear_action()
    {
        $onceM = $this->MODEL('once');

        $list = $onceM->getPriceGear(array('id' => array('>', 0), 'orderby' => 'days,asc'));

        $this->render_json(0, 'ok', compact('list'));
    }

    // 招聘时长-添加
    function price_gear_add_action()
    {
        $_POST = $this->post_trim($_POST);
        if (empty($_POST['days'])) {
            $this->render_json(1, '参数错误');
        }

        $onceM = $this->MODEL('once');

        $list = $onceM->getPriceGear(array('days' => (int)$_POST['days']));

        if (empty($list)) {
            $data = array(
                'days' => (int)$_POST['days'],
                'price' => $_POST['price']
            );

            $add = $onceM->addPriceGear($data);

            if ($add) {
                $this->cache_action();
                $this->admin_json(0, "价格档位(ID:" . $add . ")添加成功");
            } else {
                $this->render_json(2, '价格档位添加失败');
            }
        } else {
            $this->render_json(1, '已有此天数，请重新输入');
        }
    }

    // 招聘时长-修改
    function price_gear_ajax_action()
    {
        $onceM = $this->MODEL('once');

        if ($_POST['days']) {
            $priceGear = $onceM->getPriceGear(array('days' => (int)$_POST['days'], 'id' => array('<>', $_POST['id'])));

            if ($priceGear) {
                $this->render_json(2, '已有此天数，请重新输入');
            }

            $nid = $onceM->upPriceGear(array('id' => (int)$_POST['id']), array('days' => $_POST['days']));
            $msg = "价格档位(ID:" . $_POST['id'] . ")修改天数";
        }

        if (isset($_POST['price'])) {
            $nid = $onceM->upPriceGear(array('id' => (int)$_POST['id']), array('price' => $_POST['price']));
            $msg = "价格档位(ID:" . $_POST['id'] . ")修改价格";
        }

        if ($nid) {
            $this->cache_action();
            $this->admin_json(0, $msg . '成功');
        } else {
            $this->render_json(1, $msg . '失败');
        }
    }

    // 招聘时长-删除
    function price_gear_del_action()
    {
        if (empty($_POST['del'])) {
            $this->render_json(1, '参数错误');
        }

        $onceM = $this->MODEL('once');

        $ids = pylode(',', $_POST['del']);

        $id = $onceM->delPriceGear($_POST['del']);

        if ($id) {
            $this->cache_action();
            $this->admin_json(0, '价格档位(ID:' . $ids . ')删除成功');
        } else {
            $this->render_json(1, '价格档位删除失败');
        }
    }

    function cache_action()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $cacheclass->oncepricegear_cache("oncepricegear.cache.php");
    }
}

?>
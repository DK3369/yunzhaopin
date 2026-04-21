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

class set_navigation_controller extends adminCommon
{
    /**
     * 导航设置
     */
    public function index_action()
    {
        $navigationM = $this->MODEL('navigation');
        $where = array();
        if ($_POST['type'] != "") {
            $where['type'] = $_POST['type'];
        }
        if ($_POST['eject']) {
            if ($_POST['eject'] == '2') {
                $where['eject'] = '0';
            } else {
                $where['eject'] = intval($_POST['eject']);
            }
        }

        if ($_POST['display']) {
            if ($_POST['display'] == '2') {
                $where['display'] = '0';
            } else {
                $where['display'] = intval($_POST['display']);
            }
        }
        if ($_POST['nid'] != "") {
            $where['nid'] = $_POST['nid'];
        }
        if (trim($_POST['keyword'])) {
            $where['name'] = array('like', trim($_POST['keyword']));
        }

        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('navigation', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                //按id降序
                $where['orderby'] = 'id';
            }

            $where['limit'] = $pages['limit'];
            $nav = $navigationM->getNavList($where);
        }
        $navinfo = $navigationM->getNavTypeList();
        $nclass = array();
        foreach ($navinfo as $key => $value) {
            foreach ($nav as $k => $v) {
                if ($value['id'] == $v['nid']) {
                    $nav[$k]['typename'] = $value['typename'];
                }
            }
            $nclass[$value['id']] = $value['typename'];
        }

        $this->render_json(0, 'ok', compact('nclass', 'nav', 'total', 'page_sizes', 'limit', 'page'));
    }

    /**
     * 导航添加
     */
    function add_action()
    {
        $navigationM = $this->MODEL('navigation');
        $data['type'] = $navigationM->getNavTypeList();

        if ($_POST['id']) {
            $data['info'] = $navigationM->getNav(array('id' => $_POST['id']));
        }
        $data['picMaxSize'] = $this->config['pic_maxsize'] ? $this->config['pic_maxsize'] : 5;
        $data['picType'] = $this->config['pic_type'] ? $this->config['pic_type'] : 'jpg,png,jpeg,bmp,gif';
        $this->render_json(0, 'ok', $data);
    }

    /**
     * 导航保存
     */
    function save_action()
    {
        $navigationM = $this->MODEL('navigation');

        $postData       =   array(

            'nid'       =>  $_POST['nid'],
            'eject'     =>  $_POST['eject'],
            'display'   =>  $_POST['display'],
            'name'      =>  $_POST['name'],
            'url'       =>  str_replace("amp;", "", $_POST['url']),
            'furl'      =>  $_POST['furl'],
            'sort'      =>  $_POST['sort'],
            'color'     =>  $_POST['color'],
            'model'     =>  $_POST['model'],
            'bold'      =>  $_POST['bold'],
            'type'      =>  $_POST['type']
        );

        if ($_FILES['file']) {

            $postData['file']  =  $_FILES['file'];
        }

        if (!empty($_POST['id'])) {
            $return = $navigationM->upNav($postData, array('id' => $_POST['id']));

            // 图片上传失败提示
            if (isset($return['msg'])) {
                $this->render_json(1, $return['msg']);
            }

            if ($return) {
                $this->cache_action();
                $this->admin_json(0, "网站导航(ID:" . $_POST['id'] . ")修改成功");
            } else {
                $this->render_json(1, '网站导航修改失败');
            }
        } else {
            $nav = $navigationM->getNav(array('name' => $_POST['name'], 'nid' => $_POST['nid']));
            if ($nav) {
                $this->render_json(1, '已经存在此导航');
            } else {
                $return = $navigationM->addNav($postData);

                // 图片上传失败提示
                if (isset($return['msg'])) {
                    $this->render_json(1, $return['msg']);
                }

                if ($return) {
                    $this->cache_action();
                    $this->admin_json(0, "网站导航(ID:" . $return . ")添加成功");
                } else {
                    $this->render_json(1, '网站导航添加失败');
                }
            }
        }
    }

    /**
     * 删除导航
     */
    function del_action()
    {
        $navigationM = $this->MODEL('navigation');
        $descriptionM = $this->MODEL('description');
        $articleM = $this->MODEL('article');
        //批量删除
        if ($_POST['del']) {
            $del = $_POST['del'];
            if (is_array($del)) {
                //更新单页面和新闻类别
                $where = array();
                $where['id'] = array('in', pylode(',', $del));
                $where['PHPYUNBTWSTART'] = '';
                $where['desc'] = array('<>', '');
                $where['news'] = array('<>', '', 'OR');
                $where['PHPYUNBTWEND'] = '';
                $rows = $navigationM->getNavList($where);
                if (is_array($rows)) {
                    foreach ($rows as $v) {
                        if ($v['desc'] != "") {
                            $desc[] = $v['desc'];
                        }
                        if ($v['news'] != "") {
                            $news[] = $v['news'];
                        }
                        $descriptionM->upDes(array('is_menu' => '0'), array('id' => array('in', pylode(',', $desc))));
                        $articleM->updGroup(array('id' => array('in', pylode(',', $news))), array('is_menu' => '0'));
                    }
                }
                $navigationM->delNav(array('id' => array('in', pylode(',', $del))));

                $this->cache_action();
                $this->admin_json(0, "导航(ID:" . @implode(',', $_POST['del']) . ")删除成功");
            } else {
                $this->render_json(1, '请选择您要删除的信息');
            }
        }
        //删除
        if (isset($_POST['id'])) {
            //更新单页面和新闻类别
            $row = $navigationM->getNav(array('id' => $_POST['id']));
            if ($row['desc'] != "") {
                $descriptionM->upDes(array('is_menu' => '0'), array('id' => $row['desc']));
            }
            if ($row['news'] != "") {
                $articleM->updGroup(array('id' => $row['news']), array('is_menu' => '0'));
            }
            $result = $navigationM->delNav(array('id' => $_POST['id']));
            $this->cache_action();

            if ($result) {
                $this->admin_json(0, '导航(ID:' . $_POST['id'] . ')删除成功');
            } else {
                $this->render_json(1, '导航删除失败');
            }
        }
    }

    /**
     * 导航设置
     */
    function navset_action()
    {
        $navigationM = $this->MODEL('navigation');

        $return = $navigationM->upNav(array('' . $_POST['type'] . '' => intval($_POST['rec'])), array('id' => intval($_POST['id'])));

        if ($_POST['type'] == 'display') {
            $msg = "导航是否显示(ID:" . $_POST['id'] . ")设置成功";
        } else {
            $msg = "导航是否新窗口打开(ID:" . $_POST['id'] . ")设置成功";
        }

        if ($return) {
            $this->cache_action();
            $this->admin_json(0, $msg);
        } else {
            $this->render_json(1, '导航设置失败');
        }
    }

    /**
     * 导航排序
     */
    function navsort_action()
    {
        $navigationM = $this->MODEL('navigation');
        $postData = array(
            'sort' => $_POST['sort'],
        );

        $return = $navigationM->upNav($postData, array("id" => $_POST['id']));
        if ($return) {
            $this->cache_action();
            $this->admin_json(0, '导航排序（ID:' . $_POST['id'] . '）修改成功');
        } else {
            $this->render_json(1, '导航排序修改失败');
        }
    }

    // 生成导航缓存
    function cache_action()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $cacheclass->menu_cache("menu.cache.php");
    }

    // 导航类别
    function type_action()
    {
        $navigationM = $this->MODEL('navigation');
        $list = $navigationM->getNavTypeList(array('orderby' => 'id'));
        $this->render_json(0, 'ok', compact('list'));
    }

    // 添加类别
    function typeadd_action()
    {
        $navigationM = $this->MODEL('navigation');

        if (!isset($_POST['typename']) || trim($_POST['typename']) === '') {
            $this->render_json(1, '请填写分类名称');
        }

        $navtype = $navigationM->getNavType(array('typename' => $_POST['typename']));
        if ($navtype) {
            $this->render_json(1, '导航类别已存在');
        } else {
            $nbid = $navigationM->addNavType(array('typename' => $_POST['typename']));
            if ($nbid) {
                $this->cache_action();
                $this->admin_json(0, "导航类别(ID:" . $nbid . ")添加成功");
            } else {
                $this->render_json(1, '导航类别添加失败');
            }
        }
    }

    // 更新类别名称
    function typename_action()
    {
        $navigationM = $this->MODEL('navigation');

        if (!isset($_POST['typename']) || trim($_POST['typename']) === '') {
            $this->render_json(1, '请填写分类名称');
        }

        $return = $navigationM->upNavType(array('id' => $_POST['id']), array('typename' => trim($_POST['typename'])));

        if ($return) {
            $this->cache_action();
            $this->admin_json(0, "导航类别(ID:" . $_POST['id'] . ")修改成功");
        } else {
            $this->render_json(1, '导航类别修改失败');
        }
    }

    // 删除导航类别
    function typedel_action()
    {
        $navigationM = $this->MODEL('navigation');
        $descriptionM = $this->MODEL('description');
        $articleM = $this->MODEL('article');

        if (empty($_POST['id'])) {
            $this->render_json(1, '非法操作');
        }

        $return = $navigationM->delNavType(array('id' => $_POST['id']));
        $where = array();
        $where['nid'] = $_POST['id'];
        $where['PHPYUNBTWSTART'] = '';
        $where['desc'] = array('<>', '');
        $where['news'] = array('<>', '', 'OR');
        $where['PHPYUNBTWEND'] = '';
        $rows = $navigationM->getNavList($where);
        if (is_array($rows)) {
            foreach ($rows as $v) {
                if ($v['desc'] != "") {
                    $desc[] = $v['desc'];
                }
                if ($v['news'] != "") {
                    $news[] = $v['news'];
                }
            }
            $descriptionM->upDes(array('is_menu' => '0'), array('id' => array('in', pylode(',', $desc))));
            $articleM->updGroup(array('id' => array('in', pylode(',', $news))), array('is_menu' => '0'));
        }
        $navigationM->delNav(array('nid' => $_POST['id']));//删除导航

        if ($return) {
            $this->cache_action();
            $this->admin_json(0, "导航类别(ID:" . $_POST['id'] . ")删除成功");
        } else {
            $this->render_json(1, '导航类别删除失败');
        }
    }
}

?>
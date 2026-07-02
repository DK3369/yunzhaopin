<?php



class set_navigation_controller extends adminCommon
{
    /**
     * Navigation settings.
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
                // Sort by ID descending.
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
     * Add navigation.
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
     * Save navigation.
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

            // Image upload failure message.
            if (isset($return['msg'])) {
                $this->render_json(1, $return['msg']);
            }

            if ($return) {
                $this->cache_action();
                $this->admin_json(0, yun_t('admin_model_00085', array('id' => $_POST['id'])));
            } else {
                $this->render_json(1, yun_at('admin_01389'));
            }
        } else {
            $nav = $navigationM->getNav(array('name' => $_POST['name'], 'nid' => $_POST['nid']));
            if ($nav) {
                $this->render_json(1, yun_at('admin_neirong_00021'));
            } else {
                $return = $navigationM->addNav($postData);

                // Image upload failure message.
                if (isset($return['msg'])) {
                    $this->render_json(1, $return['msg']);
                }

                if ($return) {
                    $this->cache_action();
                    $this->admin_json(0, yun_t('admin_model_00086', array('id' => $return)));
                } else {
                    $this->render_json(1, yun_at('admin_01390'));
                }
            }
        }
    }

    /**
     * Delete navigation.
     */
    function del_action()
    {
        $navigationM = $this->MODEL('navigation');
        $descriptionM = $this->MODEL('description');
        $articleM = $this->MODEL('article');
        // Batch delete.
        if ($_POST['del']) {
            $del = $_POST['del'];
            if (is_array($del)) {
                // Update single pages and news categories.
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
                $this->admin_json(0, yun_t('admin_model_00087', array('ids' => @implode(',', $_POST['del']))));
            } else {
                $this->render_json(1, yun_at('common_01063'));
            }
        }
        // Delete.
        if (isset($_POST['id'])) {
            // Update single pages and news categories.
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
                $this->admin_json(0, yun_t('admin_model_00087', array('ids' => $_POST['id'])));
            } else {
                $this->render_json(1, yun_at('model_00033'));
            }
        }
    }

    /**
     * Navigation setting.
     */
    function navset_action()
    {
        $navigationM = $this->MODEL('navigation');

        $return = $navigationM->upNav(array('' . $_POST['type'] . '' => intval($_POST['rec'])), array('id' => intval($_POST['id'])));

        if ($_POST['type'] == 'display') {
            $msg = yun_t('admin_model_00088', array('id' => $_POST['id']));
        } else {
            $msg = yun_t('admin_model_00089', array('id' => $_POST['id']));
        }

        if ($return) {
            $this->cache_action();
            $this->admin_json(0, $msg);
        } else {
            $this->render_json(1, yun_at('admin_01388'));
        }
    }

    /**
     * Navigation sort.
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
            $this->admin_json(0, yun_t('admin_model_00090', array('id' => $_POST['id'])));
        } else {
            $this->render_json(1, yun_at('admin_01392'));
        }
    }

    // Generate navigation cache.
    function cache_action()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $cacheclass->menu_cache("menu.cache.php");
    }

    // Navigation categories.
    function type_action()
    {
        $navigationM = $this->MODEL('navigation');
        $list = $navigationM->getNavTypeList(array('orderby' => 'id'));
        $this->render_json(0, 'ok', compact('list'));
    }

    // Add category.
    function typeadd_action()
    {
        $navigationM = $this->MODEL('navigation');

        if (!isset($_POST['typename']) || trim($_POST['typename']) === '') {
            $this->render_json(1, yun_at('admin_01393'));
        }

        $navtype = $navigationM->getNavType(array('typename' => $_POST['typename']));
        if ($navtype) {
            $this->render_json(1, yun_at('admin_system_00049'));
        } else {
            $nbid = $navigationM->addNavType(array('typename' => $_POST['typename']));
            if ($nbid) {
                $this->cache_action();
                $this->admin_json(0, yun_t('admin_model_00091', array('id' => $nbid)));
            } else {
                $this->render_json(1, yun_at('admin_01394'));
            }
        }
    }

    // Update category name.
    function typename_action()
    {
        $navigationM = $this->MODEL('navigation');

        if (!isset($_POST['typename']) || trim($_POST['typename']) === '') {
            $this->render_json(1, yun_at('admin_01393'));
        }

        $return = $navigationM->upNavType(array('id' => $_POST['id']), array('typename' => trim($_POST['typename'])));

        if ($return) {
            $this->cache_action();
            $this->admin_json(0, yun_t('admin_model_00092', array('id' => $_POST['id'])));
        } else {
            $this->render_json(1, yun_at('admin_01395'));
        }
    }

    // Delete navigation category.
    function typedel_action()
    {
        $navigationM = $this->MODEL('navigation');
        $descriptionM = $this->MODEL('description');
        $articleM = $this->MODEL('article');

        if (empty($_POST['id'])) {
            $this->render_json(1, yun_at('member_com_00320'));
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
        $navigationM->delNav(array('nid' => $_POST['id']));// Delete navigation.

        if ($return) {
            $this->cache_action();
            $this->admin_json(0, yun_t('admin_model_00093', array('id' => $_POST['id'])));
        } else {
            $this->render_json(1, yun_at('admin_01396'));
        }
    }
}

?>

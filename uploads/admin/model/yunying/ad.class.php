<?php

class ad_controller extends adminCommon
{
    /**
     * Ad management.
     */
    public function index_action()
    {
        $pageM = $this->MODEL('page');
        $adM = $this->MODEL('ad');

        $time = time();
        // Review status.
        if ($_POST['is_check']) {
            if ($_POST['is_check'] == '1') {
                $where['is_check'] = '1';
                $where['time_end'] = array('unixtime', '>', $time);
            }
            if ($_POST['is_check'] == '-1') {
                $where['is_check'] = '0';
                $where['time_end'] = array('unixtime', '>', $time);
            }
            if ($_POST['is_check'] == '2') {
                $where['time_end'] = array('unixtime', '<=', $time);
            }
        }
        // Ad category.
        if ($_POST['class_id']) {
            $where['class_id'] = $_POST['class_id'];
        }
        if ($_POST['name']) {
            $where['ad_name'] = array('like', $_POST['name']);
        }
        // Ad type.
        if ($_POST['ad']) {
            if ($_POST['ad'] == '1') {
                $where['ad_type'] = 'word';
            }
            if ($_POST['ad'] == '2') {
                $where['ad_type'] = 'pic';
            }
            if ($_POST['ad'] == '3') {
                $where['ad_type'] = 'flash';
            }
        }
        // Build pagination.
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('ad', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        // Query the list only when records exist.
        if ($pages['total'] > 0) {
            // Limit/order is only needed for list queries.
            $orderby = array();
            if ($_POST['order']) {
                $orderby[] = $_POST['t'] . ',' . $_POST['order'];
            }
            $orderby[] = 'id,desc';
            $where['orderby'] = $orderby;
            $where['limit'] = $pages['limit'];
            // Get list data.
            $List = $adM->getAdList($where);
            $list = $List['list'];
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    function get_base_data_action()
    {
        // Ad categories.
        $adM = $this->MODEL('ad');
        $adClassArr = $adM->getAdClassArr();
        $classArr = $adClassArr['classArr'];
        $classData = array();
        if(is_array($classArr)) {
            $keyOne = 0;
            foreach ($classArr['classOne'] as $valueOne) {
                $children = array();
                $keyTwo = 0;
                foreach ($classArr['classTwo'][$valueOne['id']] as $valueTwo) {
                    $children[$keyTwo]['label'] = str_replace('&nbsp;', ' ', $valueTwo['id_name']);
                    $children[$keyTwo]['value'] = strval($valueTwo['id']);
                    $keyTwo++;
                }

                $classData[$keyOne]['label'] = $valueOne['name'];
                $classData[$keyOne]['value'] = strval($valueOne['id']);
                if ($children) {
                    $classData[$keyOne]['children'] = $children;
                }
                $keyOne++;
            }
        }

        // Sites.
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain', $Options = array('needreturn' => true, 'needassign' => true, 'needall' => true));
        $domainData = array();
        if (is_array($domain) && is_array($domain['Dname'])) {
            foreach ($domain['Dname'] as $key => $value) {
                $domainData[] = array('label' => $value, 'value' => strval($key));
            }
        }
        $this->render_json(0, '', array('classData' => $classData, 'domainData' => $domainData));
    }

    function info_action()
    {
        $info = [];
        if ($_POST['id']) {
            $adM = $this->MODEL('ad');
            $info = $adM->getInfo(array('id' => $_POST['id']));
        }
        // Handle mobile ad redirects when the mobile cache exists.
        $appad = 0;
        if (file_exists(DATA_PATH . 'api/wxapp/tplapp.cache.php')) {
            $appad = 1;
            
        }
        $this->render_json(0, '', array(
            'info' => $info ? $info : (object)[],
            'appad' => $appad
        ));
    }

    // Submit ad add/update.
    function ad_saveadd_action()
    {
        $_POST = $this->post_trim($_POST);
        if (strlen($_POST['ad_name']) < 1) {
            $this->render_json(1, yun_at('admin_01413'));
        }
        if (!is_array($_POST['ad_time']) || count($_POST['ad_time']) < 1) {
            $this->render_json(2, yun_at('admin_01414'));
        } else {
            $_POST['ad_time_start'] = $_POST['ad_time'][0];
            $_POST['ad_time_end'] = $_POST['ad_time'][1];
        }

        // Ad type.
        if ($_POST['ad_type'] == 'pic') {
            // Image ad.
            if ($_POST['upload'] == 'upload') {
                // Remote image URL.
                $pictures = $_POST['pic_url_n'];
            } else {
                // Local image upload.
                if ($_FILES['file']['tmp_name']) {
                    $upArr = array(
                        'file' => $_FILES['file'],
                        'dir' => 'pimg'
                    );
                    $uploadM = $this->MODEL('upload');
                    $pic = $uploadM->newUpload($upArr);
                    if (!empty($pic['msg'])) {
                        $this->render_json(8, $pic['msg']);
                    } elseif (!empty($pic['picurl'])) {
                        $pictures = $pic['picurl'];
                    }
                }
            }
        } elseif ($_POST['ad_type'] == 'flash') {
            // Flash ad.
            if ($_POST['flash'] == 'flash') {
                // Remote URL.
                $pictures = $_POST['flash_url'];
            } else {
                // Local upload.
                $time = time();
                $flash_name = $time . rand(0, 999) . '.swf';
                move_uploaded_file($_FILES['flash_url']['tmp_name'], DATA_PATH . '/upload/flash/' . $flash_name);
                $pictures = '../data/upload/flash/' . $flash_name;
            }
        }

        // Open in a new window: 2 yes, 1 no.
        $_POST['target'] = $_POST['target'] == 2 ? 2 : 1;
        $_POST['is_check'] = 1;
        $adM = $this->MODEL('ad');
        $return = $adM->model_saveadd($_POST, $pictures);
        if($return['error'] === 0){
            $this->admin_json($return['error'], $return['msg']);
        } else {
            $this->render_json($return['error'], $return['msg']);
        }
    }

    function del_action()
    {
        $adM = $this->MODEL('ad');
        if (is_array($_POST['del'])) {
            $delid = pylode(',', $_POST['del']);
            $where['id'] = array('in', $delid);
            $data['type'] = 'all';
        } else {
            $delid = (int)$_POST['id'];
            $where['id'] = $delid;
            $data['type'] = 'one';
        }
        if (!$delid) {
            $this->render_json(1, yun_at('common_01066'));
        }

        $del = $adM->delAd($where, $data);
        $adM->model_ad_arr();
        $del ? $this->admin_json(0, yun_t('admin_model_00043', array('ids' => $delid))) : $this->render_json(2, 'admin_user_00186');
    }

    /**
     * Ad preview.
     */
    function preview_action()
    {
        $adM = $this->MODEL('ad');

        $ad = $adM->getInfo(array('id' => (int)$_POST['id']));
        if (!$ad) {
            $this->render_json(1, yun_at('wap_js_00113'));
        }
        if ($ad['ad_type'] == 'word') {
            $ad['html'] = '<a href="' . $ad['word_url'] . '">' . $ad['word_info'] . '</a>';
        } else if ($ad['ad_type'] == 'pic') {
            $height = $width = '';
            if ($ad['pic_height']) {
                $height = 'height="' . $ad['pic_height'] . '"';
            }
            if ($ad['pic_width']) {
                $width = 'width="' . $ad['pic_width'] . '"';
            }
            $ad['html'] = '<a href="' . $ad['pic_src'] . '" target="_blank" rel="nofollow"><img src="' . $ad['pic_url_n'] . '"  ' . $height . ' ' . $width . ' ></a>';
        } else if ($ad['ad_type'] == 'flash') {
            if (@!stripos('ttp://', $ad['flash_url'])) {
                $flash_url = str_replace('../', $this->config['sy_ossurl'] . '/', $ad['flash_url']);
            }
            $ad['html'] = '<object type="application/x-shockwave-flash" data="' . $flash_url . '" width="' . $ad['flash_width'] . '" height="' . $ad['flash_height'] . '"><param name="movie" value="' . $flash_url . '" /><param value="transparent" name="wmode"></object>';
        } else if ($ad['ad_type'] == 'lianmeng') {

            $ad['html'] = $ad['lianmeng_url'];
        }
        if (@strtotime($ad['time_end'] . ' 23:59:59') < time()) {
            $ad['is_end'] = '1';
        }
        $ad['src'] = $this->config['sy_weburl'] . '/data/plus/yunimg.php?classid=' . $ad['class_id'] . '&ad_id=' . $ad['id'];
        $this->render_json(0, '', $ad);
    }

    /**
     * Review.
     */
    function check_action()
    {
        $id = (int)$_POST['id'];
        if($id) {
            $adM = $this->MODEL('ad');
            $adM->upInfo(array('id' => $id), array('is_check' => $_POST['val']));
            $adM->model_ad_arr();
            if ($_POST['val'] == '1') {
                $this->admin_json(0, '<font color="green">' . yun_t('admin_model_00044', array('id' => $id)) . '</font>');
            } else {
                $this->admin_json(0, '<font color="red">' . yun_t('admin_model_00045', array('id' => $id)) . '</font>');
            }
        }
    }

    /**
     * Refresh cache.
     */
    function cache_ad_action()
    {
        $adM = $this->MODEL('ad');
        $adM->model_ad_arr();
        $this->admin_json(0, 'admin_01172');
    }

    /**
     * Batch extension.
     */
    function ctime_action()
    {
        if ($_POST) {
            if (empty($_POST['endtime']) || intval($_POST['endtime']) < 1
                || empty($_POST['jobid'])) {
                $this->render_json(2, yun_at('common_01716'));
            }
            $adM = $this->MODEL('ad');
            $upData['time_end'] = array('DATE_ADD', $_POST['endtime']);
            $upWhere['id'] = array('in', $_POST['jobid']);
            $id = $adM->upInfo($upWhere, $upData);
            $adM->model_ad_arr();
            $id ? $this->admin_json(0, yun_t('admin_model_00046', array('ids' => $_POST['jobid']))) : $this->render_json(1, 'wap_01715');
        }
    }

    /**
     * Update sort.
     */
    function upsort_action()
    {
        if ($_POST) {
            if (empty($_POST['id']) || intval($_POST['id']) <= 0) {
                $this->render_json(1, yun_at('common_01716'));
            }

            $adM = $this->MODEL('ad');
            $upData['sort'] = intval($_POST['sort']);
            $upWhere['id'] = intval($_POST['id']);
            $id = $adM->upInfo($upWhere, $upData);

            $adM->model_ad_arr();
            $this->render_json(0, '');
        }
    }
}

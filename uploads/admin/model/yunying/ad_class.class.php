<?php

class ad_class_controller extends adminCommon
{
    /**
     * Ad categories.
     */
    public function index_action()
    {
        $pageM = $this->MODEL('page');
        $adM = $this->MODEL('ad');

        $where = array();
        $typeStr = intval($_POST['type']);
        $keywordStr = trim($_POST['keyword']);
        if (!empty($keywordStr)) {
            if ($typeStr == 1) {
                $where['id'] = $keywordStr;
            } elseif ($typeStr == 2) {
                $where['class_name'] = array('like', $keywordStr);
            }
        }

        // Build pagination.
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('ad_class', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();

        // Query the list only when records exist.
        if ($pages['total']) {
            // Limit/order is only needed for list queries.
            $orderby = array();
            if ($_POST['t'] && in_array($_POST['order'], array('asc', 'desc'))) {
                $orderby[] = $_POST['t'] . ',' . $_POST['order'];
            }
            $orderby[] = 'id,desc';
            $where['orderby'] = $orderby;
            $where['limit'] = $pages['limit'];
            // Get list data.
            $List = $adM->getAdClassList($where, array('href' => true));
            $list = $List['list'];
        }
        $rt = array();
        $rt['integral_pricename'] = $this->config['integral_pricename'];
        $rt['pic_maxsize'] = $this->config['pic_maxsize'] ? $this->config['pic_maxsize'] : 5;
        $rt['pic_type'] = $this->config['pic_type'] ? $this->config['pic_type'] : 'jpg,png,jpeg,bmp,gif';
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    function info_action()
    {
        if ($_POST['id']) {
            $adM = $this->MODEL('ad');
            $info = $adM->getAdClassInfo(array('id' => intval($_POST['id'])));
            if ($info) {
                $info['hrefn'] = checkpic($info['href']);
                $this->render_json(0, '', $info);
            } else {
                $this->render_json(1, yun_at('admin_00351'));
            }
        }
    }

    /**
     * Add or update.
     * Enable purchase.
     */
    function addclass_action()
    {
        if ($_POST['class_name']) {
            if ($_FILES['file']['tmp_name'] != '') {
                $upArr = array(
                    'file' => $_FILES['file'],
                    'dir' => 'pimg'
                );
                $uploadM = $this->MODEL('upload');
                $pic = $uploadM->newUpload($upArr);
                if (!empty($pic['msg'])) {
                    $this->render_json(8, $pic['msg']);
                } elseif (!empty($pic['picurl'])) {
                    $href = $pic['picurl'];
                }
            }

            $adM = $this->MODEL('ad');

            $data = array();
            $data['class_name'] = $_POST['class_name'];
            $data['orders'] = $_POST['orders'];
            $data['place'] = $_POST['place'];
            $data['type'] = $_POST['type'];

            if (isset($_POST['type']) && $_POST['type'] == 1) {
                $data['btype'] = $_POST['btype'];
                $data['integral_buy'] = $_POST['integral_buy'];
                if (isset($href) && $href) {
                    $data['href'] = $href;
                }
                $data['x'] = $_POST['x'];
                $data['y'] = $_POST['y'];
                $data['remark'] = $_POST['remark'];
            }

            if ($_POST['id']) {
                $upWhere['id'] = $_POST['id'];
                $nid = $adM->upAdClass($upWhere, $data);
                $nid ? $this->admin_json(0, yun_t('admin_model_00039', array('id' => $_POST['id']))) : $this->render_json(1, 'admin_00187');
            } else {
                if ($_POST['type']) {
                    $nid = $adM->addAdClass($data);
                    $nid ? $this->admin_json(0, yun_t('admin_model_00040', array('id' => $nid))) : $this->render_json(2, 'api_wxapp_00012');
                }
            }
        }
    }

    function del_action()
    {
        $adM = $this->MODEL('ad');
        if ($_POST['del']) {
            // Batch delete.
            $del = $_POST['del'];
            if ($del) {
                if (is_array($del)) {
                    $cWhere['class_id'] = array('in', pylode(',', $del));
                    $ad = $adM->getAdClassList($cWhere);
                    if (is_array($ad['list'])) {
                        $this->render_json(1, yun_at('admin_yunying_00002'));
                    } else {
                        $hWhere['id'] = array('in', pylode(',', $del));
                        $adM->delAdClass($hWhere, array('type' => 'all'));
                    }
                    $this->admin_json(0, yun_t('admin_model_00041', array('ids' => @implode(',', $del))));
                }
            } else {
                $this->render_json(2, yun_at('admin_01415'));
            }
        } else {
            // Single delete.
            if (intval($_POST['id']) <= 0) {
                $this->render_json(4, yun_at('admin_01415'));
            }
            $ad = $adM->getInfo(array('class_id' => intval($_POST['id'])));
            if (is_array($ad)) {
                $this->render_json(3, yun_at('admin_yunying_00002'));
            } else {
                $adM->delAdClass(array('id' => intval($_POST['id'])), array('type' => 'one'));
                $this->admin_json(0, yun_t('admin_model_00041', array('ids' => intval($_POST['id']))));
            }
        }
    }

    /**
     * Cancel purchase.
     */
    function delbuy_action()
    {
        if (isset($_POST['id'])) {
            $adM = $this->MODEL('ad');
            $data['integral_buy'] = '';
            $data['href'] = '';
            $data['type'] = 2;
            $data['btype'] = '';
            $data['x'] = '';
            $data['y'] = '';
            $data['remark'] = '';
            $result = $adM->upAdClass(array('id' => intval($_POST['id'])), $data);
            if ($result) {
                $this->admin_json(0, yun_t('admin_model_00042', array('id' => $_POST['id'])));
            } else {
                $this->render_json(1, yun_at('model_00004'));
            }
        }
    }

    /**
     * Sort.
     */
    function upsort_action(){
        if ($_POST) {
            if (empty($_POST['id']) || intval($_POST['id']) <= 0) {
                $this->render_json(1, yun_at('common_01716'));
            }

            $adM = $this->MODEL('ad');
            $upData['orders'] = intval($_POST['orders']);
            $upWhere['id'] = intval($_POST['id']);
            $nid = $adM->upAdClass($upWhere, $upData);
            $this->render_json(0, '');
        }
    }
}

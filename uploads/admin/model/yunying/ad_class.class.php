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
class ad_class_controller extends adminCommon
{
    /**
     * 广告类别
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

        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('ad_class', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();

        //分页数大于0的情况下 执行列表查询
        if ($pages['total']) {
            //limit order 只有在列表查询时才需要
            $orderby = array();
            if ($_POST['t'] && in_array($_POST['order'], array('asc', 'desc'))) {
                $orderby[] = $_POST['t'] . ',' . $_POST['order'];
            }
            $orderby[] = 'id,desc';
            $where['orderby'] = $orderby;
            $where['limit'] = $pages['limit'];
            //获取列表
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
                $this->render_json(1, '无数据');
            }
        }
    }

    /**
     * 添加/修改
     * 设为购买
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
                $nid ? $this->admin_json(0, '广告类别(ID:' . $_POST['id'] . ')修改成功') : $this->render_json(1, '修改失败');
            } else {
                if ($_POST['type']) {
                    $nid = $adM->addAdClass($data);
                    $nid ? $this->admin_json(0, '广告类别(ID:' . $nid . ')添加成功') : $this->render_json(2, '添加失败');
                }
            }
        }
    }

    function del_action()
    {
        $adM = $this->MODEL('ad');
        if ($_POST['del']) {
            //批量删除
            $del = $_POST['del'];
            if ($del) {
                if (is_array($del)) {
                    $cWhere['class_id'] = array('in', pylode(',', $del));
                    $ad = $adM->getAdClassList($cWhere);
                    if (is_array($ad['list'])) {
                        $this->render_json(1, '该分类下还有广告，请清空后再执行删除！');
                    } else {
                        $hWhere['id'] = array('in', pylode(',', $del));
                        $adM->delAdClass($hWhere, array('type' => 'all'));
                    }
                    $this->admin_json(0, '广告类别(ID:' . @implode(',', $del) . ')删除成功！');
                }
            } else {
                $this->render_json(2, '请选择要删除的内容！！');
            }
        } else {
            //单个删除
            if (intval($_POST['id']) <= 0) {
                $this->render_json(4, '请选择要删除的内容！！');
            }
            $ad = $adM->getInfo(array('class_id' => intval($_POST['id'])));
            if (is_array($ad)) {
                $this->render_json(3, '该分类下还有广告，请清空后再执行删除！');
            } else {
                $adM->delAdClass(array('id' => intval($_POST['id'])), array('type' => 'one'));
                $this->admin_json(0, '广告类别(ID:' . intval($_POST['id']) . ')删除成功！');
            }
        }
    }

    /**
     * 取消购买
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
                $this->admin_json(0, '广告类别(ID:' . $_POST['id'] . ')取消可购买！');
            } else {
                $this->render_json(1, '取消失败！');
            }
        }
    }

    /**
     * 排序
     */
    function upsort_action(){
        if ($_POST) {
            if (empty($_POST['id']) || intval($_POST['id']) <= 0) {
                $this->render_json(1, '参数有误');
            }

            $adM = $this->MODEL('ad');
            $upData['orders'] = intval($_POST['orders']);
            $upWhere['id'] = intval($_POST['id']);
            $nid = $adM->upAdClass($upWhere, $upData);
            $this->render_json(0, '');
        }
    }
}
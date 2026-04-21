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
class company_comrating_controller extends adminCommon
{
    //会员-企业-套餐设置

//region 套餐设置
    /**
     * @desc 增值套餐服务 -- 套餐列表
     */
    function index_action()
    {
        $ratingM = $this->MODEL('rating');
        $where['category'] = '1';
        if ($_POST['rating']) {
            $where['id'] = intval($_POST['rating']);
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('company_rating', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('type,asc', 'sort,desc');
            }
            $where['limit'] = $pages['limit'];
            $list = $ratingM->getList($where, array('utype' => 'admin'));
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    function baseData_action()
    {
        $config = array('integral_pricename' => $this->config['integral_pricename']);
        $this->render_json(0, '', array('config' => $config));
    }

    /**
     * @desc 增值套餐服务  --  设置/修改  套餐
     */
    function rating_action()
    {
        $ratingM = $this->MODEL('rating');
        if ($_POST['id']) {
            $row = $ratingM->getInfo(array('id' => intval($_POST['id'])));
            $row['time'] = null;
            if ($row['time_start'] > 0 && $row['time_end'] > 0) {
                $row['time'] = array(date('Y-m-d', $row['time_start']), date('Y-m-d', $row['time_end']));
            }
            $this->render_json(0, '', $row);
        }
    }

    /**
     * @desc   会员-企业-套餐服务
     * @desc   新增/修改 会员套餐 -- 保存数据
     */
    function saveclass_action()
    {
        $ratingM = $this->MODEL('rating');
        $_POST['file'] = $_FILES['file'];
        $postData = $_POST;
        
        if(intval($postData['max_time'])>0 && intval($postData['max_time'])<intval($postData['service_time'])){
            $this->render_json(8,'最长有效期不能小于服务时间!');
        }

        $id = intval($_POST['id']);
        if (!empty($id)) {
            $err = $ratingM->upRating($id, $postData);
        } else {
            $err = $ratingM->addRating($postData);
        }
        $this->cache_rating();
        if ($err['errcode'] == 9) {
            $this->admin_json(0, $err['msg']);
        } else {
            $this->render_json($err['errcode'], $err['msg']);
        }
    }

    /**
     * 套餐设置 删除
     */
    function delrating_action()
    {
        if ($_POST['del']) {
            $id = $_POST['del'];
        } else if ($_POST['id']) {
            $id = $_POST['id'];
        }
        $ratingM = $this->MODEL('rating');
        $err = $ratingM->delRating($id, array('category' => '1'));
        if ($err['errcode'] == 9) {
            $this->admin_json(0, $err['msg']);
        } else {
            $this->render_json($err['errcode'], $err['msg']);
        }
    }

    function cache_rating()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $makecache = $cacheclass->comrating_cache("comrating.cache.php");
    }
//endregion

//region 增值服务
    /**
     * @desc 后台--会员--企业--增值套餐服务-- 增值服务列表
     */
    function server_action()
    {
        $ratingM = $this->MODEL('rating');
        $list = $ratingM->getComServiceList(array('orderby' => 'sort,desc'));
        $this->render_json(0, '', array('list' => $list, 'total' => 0));
    }

    /**
     * @desc 后台--会员--企业--增值套餐服务-- 设置/修改 增值类型
     */
    function srating_action()
    {
        if ($_POST['id']) {
            $where['id'] = intval($_POST['id']);
            $ratingM = $this->MODEL('rating');
            $row = $ratingM->getComServiceInfo($where);
            $this->render_json(0, '', $row);
        }
    }

    /**
     * @desc 后台--会员--企业--增值套餐服务-- 保存增值类型
     */
    function save_action()
    {
        $ratingM = $this->MODEL('rating');
        $postData = $_POST;
        $err = $ratingM->upComService($postData);
        if ($err['errcode'] == 9) {
            $this->admin_json(0, $err['msg']);
        } else {
            $this->render_json(8, $err['msg']);
        }
    }

    /**
     * @desc 后台--会员--企业--增值套餐服务-- 增值服务列表
     *  ajax 修改类型名称
     */
    function ajax_action()
    {
        $id = intval($_POST['id']);
        $name = trim($_POST['name']);
        $ratingM = $this->MODEL('rating');
        if (!empty($name) && !empty($id)) {
            $serviceNum = $ratingM->getComServiceNum(array('name' => $name, 'id' => array('<>', $id)));
            if ($serviceNum) {
                $this->render_json(2, '名称已存在！');
            }

            $ratingM->setComService(array('name' => $name), array('id' => $id));
            $this->admin_json(0, "企业增值类型(ID:" . $_POST['id'] . ")名称修改成功");
        }
    }

    /**
     * @desc 设置增值服务状态
     */
    function opera_action()
    {
        $id = intval($_POST['id']);
        $display = intval($_POST['display']);
        if (!empty($id) && !empty($display)) {
            $nid = $this->MODEL('rating')->setComService(array("display" => $display), array("id" => $id));
            if ($nid) {
                $this->render_json(0, '设置成功！');
            } else {
                $this->render_json(2, '设置失败！');
            }
        }
    }

    /**
     * @desc 删除增值服务类型
     */
    function delserver_action()
    {
        if ($_POST['del']) {
            $id = $_POST['del'];
        } else if ($_POST['id']) {
            $id = $_POST['id'];
        }
        $ratingM = $this->MODEL('rating');
        $err = $ratingM->delComService($id, array('category' => '3'));
        if ($err['errcode'] == 9) {
            $this->admin_json(0, $err['msg']);
        } else {
            $this->render_json(8, $err['msg']);
        }
    }
//endregion
//region 企业增值服务详情
    /**
     * @desc 企业增值服务详情列表查询
     */
    function list_action()
    {
        $config = array();
        $ratingM = $this->MODEL('rating');
        $id = intval($_POST['id']);
        $row = (object)array();
        $list = array();
        if (!empty($id)) {
            $row = $ratingM->getComServiceInfo(array('id' => $id), array('field' => 'name'));
            $list = $ratingM->getComSerDetailList(array('type' => $id, 'orderby' => 'sort,desc'));
        }
        $this->render_json(0, '', array(
            'row' => $row,
            'list' => $list,
            'config' => $config,
        ));
    }


    function zzData_action()
    {
        $config = array();
        //增值类型
        $ratingM = $this->MODEL('rating');
        $zzlist = $ratingM->getComServiceList(array('orderby' => 'sort,desc'));
        $this->render_json(0, '', array(
            'config' => $config,
            'zzlist' => $zzlist,
        ));
    }

    /**
     * @desc 企业增值服务套餐  -- 添加 / 修改  增值服务套餐详情
     */
    function edittc_action()
    {
        $ratingM = $this->MODEL('rating');
        $tid = intval($_POST['tid']);

        $listinfo = (object)array();
        if (!empty($tid)) {
            $listinfo = $ratingM->getComSerDetailInfo($tid);
        }
        $this->render_json(0, '', $listinfo);
    }


    /**
     * @desc  后台企业增值服务套餐
     *        添加 / 修改  增值服务套餐详情  保存
     */
    function saves_action()
    {
        $ratingM = $this->MODEL('rating');
        $postData = $_POST;
        $err = $ratingM->upComSerDetail($postData);
        if ($err['errcode'] == 9) {
            $this->admin_json(0, $err['msg']);
        } else {
            $this->render_json($err['errcode'], $err['msg']);
        }
    }

    function delpic_action(){
        
        if($_POST['id']){
            
            $ratingM     =   $this -> MODEL('rating');
            
            $err = $ratingM    ->  upRating(intval($_POST['id']),array('com_pic'=>''));
            
            $this -> cache_rating();

        }else{
            $err['errcode'] = 8;
            $err['msg'] = '参数错误请重试';
        }

        if ($err['errcode'] == 9) {
            $this->admin_json(0,'企业会员等级（ID：(ID:'.$_POST['id'].')图标删除成功！');
        }else{
            $this->render_json($err['errcode'], $err['msg']);
        }
        
    }
    /**
     * @desc 删除增值服务套餐详情
     */
    function del_action()
    {
        if ($_POST['del']) {
            $id = $_POST['del'];
        } else if ($_POST['id']) {
            $id = $_POST['id'];
        }
        $ratingM = $this->MODEL('rating');
        $err = $ratingM->delComSerDetail($id);
        if ($err['errcode'] == 9) {
            $this->admin_json(0, $err['msg']);
        } else {
            $this->render_json($err['errcode'], $err['msg']);
        }
    }
//endregion
}
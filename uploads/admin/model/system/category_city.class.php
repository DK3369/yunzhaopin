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
class category_city_controller extends adminCommon
{
    //系统-类别-城市管理

    /**
     * 城市管理
     */
    public function index_action()
    {
        $categoryM = $this->MODEL('category');
        $whereData['keyid'] = '0';
        $whereData['orderby'] = array('sort,asc', 'id,asc');
        $list = $categoryM->getCityClassList($whereData, '*, 1 as level');
        $categoryM->processCityHasChildren($list, 1);
        $this->render_json(0, '', $list);
    }

    /**
     * 获取子城市列表
     */
    public function get_city_children_action()
    {
        $keyid = $_POST['keyid'];
        $level = intval($_POST['level']);
        if (!$_POST['keyid']) {
            $this->render_json(1, '参数错误');
        }

        $whereData['keyid'] = $keyid;
        $whereData['orderby'] = array('sort,asc', 'id,asc');
        $categoryM = $this->MODEL('category');
        $list = $categoryM->getCityClassList($whereData, '*, ' . $level . ' as level');
        $categoryM->processCityHasChildren($list, $level);
        $this->render_json(0, '', array('list' => $list));
    }

    /**
     * 删除
     */
    function del_action()
    {
        if ((int)$_POST['delid']) {
            $idArr = explode(',', $_POST['delid']);
            foreach ($idArr as $id) {
                if (intval($id) <= 0) {
                    $this->render_json(1, '参数有误');
                };
            }

            $categoryM = $this->MODEL('category');
            $whereData['id'] = array('in', $_POST['delid']);
            $return = $categoryM->delCityClass($whereData);
            if ($return['error'] === 0) {
                $this->admin_json($return['error'], $return['msg']);
            } else {
                $this->render_json($return['error'], $return['msg']);
            }
        }
    }

    public function add_single_action()
    {
        $_POST = $this->post_trim($_POST);

        $addData[] = array(
            'keyid' => intval($_POST['keyid']),
            'name' => $_POST['name'],
            'letter' => $_POST['letter'],
            'display' => intval($_POST['display']),
            'sort' => intval($_POST['sort']),
            'e_name' => $_POST['e_name'],
            'code' => $_POST['code'],
        );

        $categoryM = $this->MODEL('category');
        $bool = $categoryM->addCityClass($addData);
        if ($bool) {
            $this->admin_json(0, '城市管理添加成功！');
        } else {
            $this->render_json(0, '城市管理添加失败！');
        }
    }

    /**
     * 更新单条记录
     */
    public function up_single_action()
    {
        $_POST = $this->post_trim($_POST);

        $addData = array(
            'name' => $_POST['name'],
            'letter' => $_POST['letter'],
            'display' => intval($_POST['display']),
            'sort' => intval($_POST['sort']),
            'e_name' => $_POST['e_name'],
            'code' => $_POST['code'],
        );
        $whereData['id'] = $_POST['id'];

        $categoryM = $this->MODEL('category');
        $return = $categoryM->upCityClass($whereData, $addData, array('type' => 'single'));
        if ($return['error'] === 0) {
            $this->admin_json($return['error'], $return['msg']);
        } else {
            $this->render_json($return['error'], $return['msg']);
        }
    }

    function ajax_action()
    {
        if ($_POST) {
            $categoryM = $this->MODEL('category');
            $whereData['id'] = $_POST['id'];
            $addData['name'] = $_POST['name'];
            $addData['e_name'] = $_POST['e_name'];
            $return = $categoryM->upCityClass($whereData, $addData);
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    /**
     * 生成拼音
     */
    function ajaxpinyin_action()
    {
        if ($_POST) {
            $where['e_name'][] = array('isnull');
            $where['e_name'][] = array('=', '', 'OR');
            $where['orderby'] = 'sort,desc';
            $data['field'] = '`id`,`name`,`e_name`';
            $data['type'] = 'city';
            $data['post'] = $_POST;
            $categoryM = $this->MODEL('category');
            $return = $categoryM->setPinYin($where, $data);
            if ($return['error'] === 0) {
                $this->admin_json($return['error'], '城市管理' . $return['msg']);
            } else {
                $this->render_json($return['error'], '城市管理' . $return['msg'], array('page' => $return['page']));
            }
        }
    }

    /**
     * 一键查重
     */
    function ajaxchachong_action()
    {
        if ($_POST) {
            $where['e_name'] = array('<>', '');
            $where['groupby'] = 'e_name';
            $where['having'] = array('enum' => array('>', '1'));
            $data = array(
                'field' => '*,count(*) as enum',
                'page' => $_POST['page'],
                'type' => 'city'
            );
            $categoryM = $this->MODEL('category');
            $list = $categoryM->setChaChong($where, $data);
            $this->render_json(0, '', $list);
        }
    }

    /**
     * 清空拼音
     */
    function clearpinyin_action()
    {
        $categoryM = $this->MODEL('category');
        $categoryM->clearPinYin('city_class');
        $this->admin_json(0, '城市管理清空拼音成功');
    }

    function upp_action(){

        $categoryM  =   $this -> MODEL('category');
        
        if($_POST['id_arr']!=""){
            
            $id_arr         =   @explode(",",$_POST['id_arr']);
            
            foreach($id_arr as $key=>$value){
                if($_POST["cityname_".$value]!=""){//更新城市
                    $upData['name']     =   $_POST["cityname_".$value];
                    $upData['e_name']   =   $_POST["citye_name_".$value];
                    $upData['sort']     =   $_POST["citysort_".$value];
                    $upData['letter']   =   $_POST["letter_".$value];
                    $upData['display']  =   $_POST["display_".$value];
                    $upData['sitetype'] =   $_POST["sitetype_".$value];
                    $upData['code']     =   $_POST["citycode_".$value];
                    $upWhere['id']      =   $value;
                    $categoryM  ->  upCityClass($upWhere,$upData,array('type'=>'multi'));
                }
            }
            
            $categoryM  ->  cache_action('city_cache','city');
            
            $error = 0;
            $msg = '区域修改成功！';
            
        }

        if ($error === 0) {
            $this->admin_json($error, $msg);
        } else {
            $this->render_json($error, $msg);
        }
    }
}
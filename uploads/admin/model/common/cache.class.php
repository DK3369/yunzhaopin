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

class cache_controller extends adminCommon
{
    public function poi_action(){
        $data = array();

        $_POST = $this->post_trim($_POST);
        
        $res = getPoi($_POST);
        
        $data = json_decode($res, 1);
        if ($data['tips']) {
            // 去除经纬度为空的记录
            foreach ($data['tips'] as $k => $v) {
                $data['tips'][$k]['value'] = $v['name'];
            }
            $data['tips'] = array_values($data['tips']);
            $data['count'] = count($data['tips']);
        }

        $this->render_json(0, 'ok', $data);
    }
    /**
     * 获取职位类别
     * 以下参数不共存
     * @param int $level 一次获取几级分类
     *                  1只返回一级 2返回一级、二级 3返回一级、二级、三级
     * @param int $pid 父类ID
     * @param string $name 分类名称搜索
     * return json
     */
    public function getJobClass_action()
    {
        $classList = array();

        /* @var $job_index */
        /* @var $job_type */
        /* @var $job_name */
        include_once(PLUS_PATH . 'job.cache.php');

        if (isset($_POST['name'])) { // 查询数据库
            $name = trim($_POST['name']);
            if ($name) {
                $classList = $this->MODEL('category')->getJobClassList(array(
                    'name' => array('like', $name),
                    'keyid' => array('>', 0),
                    'orderby' => 'sort,asc'
                ), 'id,name');

                if ($classList) { // 职位类别不能选一级，这里只返回二、三级
                    $newClassList = array();

                    /* @var $job_parent */
                    include_once(PLUS_PATH . 'jobparent.cache.php');
                    foreach ($classList as $val) {
                        $pid = $job_parent[$val['id']];
                        if (in_array($pid, $job_index) && !isset($newClassList[$pid])) { // 二级
                            ${'twoClassId' . $val['id']} = array(
                                'id' => $val['id'],
                                'name' => $val['name'],
                                'children' => false
                            );
                            $newClassList[$pid] = array(
                                'id' => $pid,
                                'name' => $job_name[$pid],
                                'children' => array($val['id'] => &${'twoClassId' . $val['id']})
                            );
                        } elseif ($pid > 0) { // 三级
                            $oneId = $job_parent[$pid]; // 根据二级ID，反选一级ID
                            if (isset($newClassList[$oneId]['children'][$pid])) { // 二级已存在，直接赋值三级
                                ${'twoClassId' . $pid}['children'][] = array(
                                    'id' => $val['id'],
                                    'name' => $val['name']
                                );
                            } else { // 二级不存在，补充二级，初始三级
                                ${'twoClassId' . $pid} = array(
                                    'id' => $pid,
                                    'name' => $job_name[$pid],
                                    'children' => array(array(
                                        'id' => $val['id'],
                                        'name' => $val['name']
                                    ))
                                );

                                if (isset($newClassList[$oneId])) {
                                    $newClassList[$oneId]['children'][$pid] = &${'twoClassId' . $pid};
                                } else {
                                    $newClassList[$oneId] = array(
                                        'id' => $oneId,
                                        'name' => $job_name[$oneId],
                                        'children' => array($pid => &${'twoClassId' . $pid})
                                    );
                                }
                            }
                        }
                    }

                    // 根据一级顺序，对结果排序
                    $classList = array();
                    foreach ($job_index as $one) {
                        if (isset($newClassList[$one])) {
                            $children = $newClassList[$one]['children'];
                            if (!empty($job_type[$one]) && !empty($children)) {
                                $newChildren = array();
                                foreach ($job_type[$one] as $two) { // 二级重新排序
                                    if (isset($children[$two])) {
                                        $newChildren[] = $children[$two];
                                    }
                                }
                                $newClassList[$one]['children'] = $newChildren;
                            } else {
                                $newClassList[$one]['children'] = array_values($children);
                            }
                            $classList[] = $newClassList[$one];
                        }
                    }
                }
            }
        } else { // 读缓存
            if (isset($_POST['pid'])) { // 获取子类
                if (!empty($job_type[$_POST['pid']])) {
                    foreach ($job_type[$_POST['pid']] as $val) {
                        $classList[] = array(
                            'id' => $val,
                            'name' => $job_name[$val]
                        );
                    }
                }
            } else {
                $level = !empty($_POST['level']) && intval($_POST['level']) > 0 ? intval($_POST['level']) : 2; // 默认只返回两级

                if (!empty($job_index)) {
                    foreach ($job_index as $one) {
                        $oneChildren = array();
                        if (!empty($job_type[$one]) && $level >= 2) {
                            foreach ($job_type[$one] as $two) {
                                $twoChildren = array();
                                if (!empty($job_type[$two]) && $level == 3) {
                                    foreach ($job_type[$two] as $three) {
                                        $twoChildren[] = array(
                                            'id' => $three,
                                            'name' => $job_name[$three]
                                        );
                                    }
                                }
                                $oneChildren[] = array(
                                    'id' => $two,
                                    'name' => $job_name[$two],
                                    'children' => $job_type[$two] ? $twoChildren : false
                                );
                            }
                        }
                        $classList[] = array(
                            'id' => $one,
                            'name' => $job_name[$one],
                            'children' => $oneChildren
                        );
                    }
                }
            }
        }

        $this->render_json(0, 'ok', compact('classList'));
    }

    /**
     * 获取职位类别对应的所有下级ID
     * @param int $pid 父类ID
     */
    public function getJobChildIds_action()
    {
        if (empty($_POST['pid'])) {
            return $this->render_json(1, '参数错误');
        }

        $pid = intval($_POST['pid']);

        /* @var $job_type */
        include_once(PLUS_PATH . 'job.cache.php');

        $idArr = !empty($job_type[$pid]) ? $job_type[$pid] : array();

        $this->render_json(0, 'ok', $idArr);
    }

    /**
     * 获取城市类别
     * 以下参数不共存
     * @param int $level 一次获取几级分类
     *                  1只返回一级 2返回一级、二级 3返回一级、二级、三级
     * @param int $pid 父类ID
     * @param string $name 分类名称搜索
     * return json
     */
    public function getCityClass_action()
    {
        $classList = array();

        /* @var $city_index */
        /* @var $city_type */
        /* @var $city_name */
        include_once(PLUS_PATH . 'city.cache.php');

        if (isset($_POST['name'])) { // 查询数据库
            $name = trim($_POST['name']);
            if ($name) {
                $classList = $this->MODEL('category')->getCityClassList(array(
                    'name' => array('like', $name),
                    'display' => '1',
                    'orderby' => 'sort,asc'
                ), 'id,name');

                if ($classList) { // 职位类别不能选一级，这里只返回二、三级
                    $newClassList = array();

                    /* @var $city_parent */
                    include_once(PLUS_PATH . 'cityparent.cache.php');
                    foreach ($classList as $val) {
                        $pid = $city_parent[$val['id']];
                        if ($pid == '0' && !isset($newClassList[$val['id']])) { // 一级
                            $newClassList[$val['id']] = array(
                                'id' => $val['id'],
                                'name' => $val['name'],
                                'children' => array()
                            );
                        } elseif (in_array($pid, $city_index) && !isset($newClassList[$pid])) { // 二级
                            ${'twoCityId' . $val['id']} = array(
                                'id' => $val['id'],
                                'name' => $val['name'],
                                'children' => false
                            );
                            $newClassList[$pid] = array(
                                'id' => $pid,
                                'name' => $city_name[$pid],
                                'children' => array($val['id'] => &${'twoCityId' . $val['id']})
                            );
                        } elseif ($pid > 0) { // 三级
                            $oneId = $city_parent[$pid]; // 根据二级ID，反选一级ID
                            if (isset($newClassList[$oneId]['children'][$pid])) { // 二级已存在，直接赋值三级
                                ${'twoCityId' . $pid}['children'][] = array(
                                    'id' => $val['id'],
                                    'name' => $val['name']
                                );
                            } else { // 二级不存在，补充二级，初始三级
                                ${'twoCityId' . $pid} = array(
                                    'id' => $pid,
                                    'name' => $city_name[$pid],
                                    'children' => array(array(
                                        'id' => $val['id'],
                                        'name' => $val['name']
                                    ))
                                );

                                if (isset($newClassList[$oneId])) { // 判断一级是否存在
                                    $newClassList[$oneId]['children'][$pid] = &${'twoCityId' . $pid};
                                } else { // 不存在拼接一级
                                    $newClassList[$oneId] = array(
                                        'id' => $oneId,
                                        'name' => $city_name[$oneId],
                                        'children' => array($pid => &${'twoCityId' . $pid})
                                    );
                                }
                            }
                        }
                    }

                    // 根据一级顺序，对结果排序
                    $classList = array();
                    foreach ($city_index as $one) {
                        if (isset($newClassList[$one])) {
                            $children = $newClassList[$one]['children'];
                            if (!empty($city_type[$one]) && !empty($children)) {
                                $newChildren = array();
                                foreach ($city_type[$one] as $two) { // 二级重新排序
                                    if (isset($children[$two])) {
                                        $newChildren[] = $children[$two];
                                    }
                                }
                                $newClassList[$one]['children'] = $newChildren;
                            } else {
                                $newClassList[$one]['children'] = array_values($children);
                            }
                            $classList[] = $newClassList[$one];
                        }
                    }
                }
            }
        } else { // 读缓存
            if (isset($_POST['pid'])) { // 获取子类
                if (!empty($city_type[$_POST['pid']])) {
                    foreach ($city_type[$_POST['pid']] as $val) {
                        $classList[] = array(
                            'id' => $val,
                            'name' => $city_name[$val]
                        );
                    }
                }
            } else {
                $level = !empty($_POST['level']) && intval($_POST['level']) > 0 ? intval($_POST['level']) : 2; // 默认只返回两级

                if (!empty($city_index)) {
                    foreach ($city_index as $one) {
                        $oneChildren = array();
                        if (!empty($city_type[$one]) && $level >= 2) {
                            foreach ($city_type[$one] as $two) {
                                $twoChildren = array();
                                if (!empty($city_type[$two]) && $level == 3) {
                                    foreach ($city_type[$two] as $three) {
                                        $twoChildren[] = array(
                                            'id' => $three,
                                            'name' => $city_name[$three]
                                        );
                                    }
                                }
                                $oneChildren[] = array(
                                    'id' => $two,
                                    'name' => $city_name[$two],
                                    'children' => $city_type[$two] ? $twoChildren : false,
                                    'childrenId' => $city_type[$two]  // 优先返回三级ID，用来选择父级，清空子级使用
                                );
                            }
                        }
                        $classList[] = array(
                            'id' => $one,
                            'name' => $city_name[$one],
                            'children' => $oneChildren
                        );
                    }
                }
            }
        }

        $this->render_json(0, 'ok', compact('classList'));
    }

    /**
     * 获取城市类别对应的所有下级ID
     * @param int $pid 父类ID
     */
    public function getCityChildIds_action()
    {
        if (empty($_POST['pid'])) {
            return $this->render_json(1, '参数错误');
        }

        $pid = intval($_POST['pid']);

        /* @var $city_type */
        include_once(PLUS_PATH . 'city.cache.php');

        $idArr = array();
        if (!empty($city_type[$pid])) { // 获取所有二级ID
            foreach ($city_type[$pid] as $twoId) {
                $idArr[] = $twoId;
                if (!empty($city_type[$twoId])) { // 获取所有三级ID
                    foreach ($city_type[$twoId] as $threeId) {
                        $idArr[] = $threeId;
                    }
                }
            }
        }

        $this->render_json(0, 'ok', $idArr);
    }

    // 获取城市-地区联动
    public function getCity_action()
    {
        /**
         * @var $city_index
         * @var $city_name
         * @var $city_type
         */
        include_once(PLUS_PATH . 'city.cache.php');

        $data = array();
        if (empty($_POST['level'])) {
            if (!empty($city_index)) {
                foreach ($city_index as $cival) {
                    $data['provinceList'][] = ['id' => $cival, 'name' => $city_name[$cival]];
                }
            }
            $data['cityList'] = []; // 清空二级
            $data['regionList'] = []; // 清空三级
        }

        // 获取一级的下级 市 联动切换或者赋值已选中列表
        if (!empty($_POST['provinceid'])) {
            $data['cityList'] = []; // 缓存不存在时，数组初始
            if (!empty($city_type[$_POST['provinceid']])) {
                foreach ($city_type[$_POST['provinceid']] as $cival) {
                    $data['cityList'][] = ['id' => $cival, 'name' => $city_name[$cival]];
                }
            }
            $data['regionList'] = []; // 清空三级
        }

        // 获取二级的下级 联动切换或者赋值已选中列表
        if (!empty($_POST['cityid'])) {
            $data['regionList'] = [];; // 缓存不存在时，数组初始
            if (!empty($city_type[$_POST['cityid']])) {
                foreach ($city_type[$_POST['cityid']] as $cival) {
                    $data['regionList'][] = ['id' => $cival, 'name' => $city_name[$cival]];
                }
            }
        }

        $this->render_json(0, 'ok', $data);
    }

    public function getUrl_action(){
        $sy_weburl = $this->config['sy_weburl'];
        $this->render_json('0','',['sy_weburl'=>$sy_weburl]);
    }

    public function  getDname_action(){
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain');
        $this->render_json(0,'',['Dname'=>(object)$domain['Dname']]);
    }


    public function getPriceName_action(){
        $integral_pricename = $this->config['integral_pricename'];
        $this->render_json(0,'',['integral_pricename'=>$integral_pricename]);
    }

}

?>
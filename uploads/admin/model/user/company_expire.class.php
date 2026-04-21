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
class company_expire_controller extends adminCommon
{
    function index_action(){
        $ComM = $this->MODEL('company');
        // 过期会员
        $toDay = strtotime(date('Y-m-d'));
        $where['PHPYUNBTWSTART_A'] = '';
        $where['vipetime'][] = array('>', '0', 'AND');
        $where['vipetime'][] = array('<', $toDay, 'AND');
        $where['PHPYUNBTWEND_A'] = '';
        if($_POST['keyword']){
            $keywordStr = trim($_POST['keyword']);
            $typeStr = intval($_POST['type']);
            if (!empty($keywordStr) && $typeStr == 1) {
                $where['name'] = array('like',$keywordStr);
            }elseif (!empty($keywordStr) && $typeStr == 2){
                $mwhere['username'] = array('like', $keywordStr);
            }
        }
        $mUids = array();
        $UserinfoM = $this->MODEL('userinfo');
        if(!empty($mwhere)){
            $uidList = $UserinfoM->getList($mwhere, array('field' => '`uid`'));
            if(!empty($uidList)){
                foreach($uidList as $uv){
                    $mUids[] = $uv['uid'];
                }
            }else{
                $mUids = array(0);
            }
            $where['uid'][] = array('in', pylode(',',$mUids));
        }
        //提取分页
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('company', $where, $page, array('limit' => $pageSize));
        //分页数大于0的情况下 执行列表查询
        if($pages['total'] > 0){
            //limit order 只有在列表查询时才需要
            if($_POST['order']){
                $where['orderby'] =	$_POST['t'].','.$_POST['order'];
            }else{
                $where['orderby'] =	array('uid,desc');
            }
            $where['limit']	= $pages['limit'];
            $rows = $ComM->getList($where, array('utype'=>'admin', 'concern' => 1));
        }
        $rt['list'] = $rows['list'] ? $rows['list'] : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0, '', $rt);
    }
}
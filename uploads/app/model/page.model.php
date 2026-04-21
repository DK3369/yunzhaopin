<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class page_model extends model
{

    function pageList($tableName, $whereData, $pageUrl, $pageNow, $limit = '', $sql = '', $maxPage = false)
    {


        $limit  =   $limit == '' ? $this->config['sy_listnum'] : intval($limit);
        $page   =   $pageNow < 1 ? 1 : intval($pageNow);

        if ($tableName == 'ad' && $this->config['did']) {

            $whereData['did']   =   $this->config['did'];
        }

        if ($sql) {

            $result =   $this->DB_query_all($sql);
            $num    =   $result ? $result['num'] : 0;
        } else {

            //统计总数的时候 去掉order by条件
			$whereTotalData = $whereData;
			if($whereData['orderby']){
				unset($whereTotalData[orderby]);
			}
            $num    =   $this->select_num($tableName, $whereTotalData);
        }

        // 超出最大页是否要处理
        if ($maxPage) {
            $mpage  =   ceil($num / $limit);
            $page > $mpage && $page = $mpage;
        }

        $pagenav = Page($page, $num, $limit, $pageUrl, $notpl = false, $this->tpl);

        return array('total' => $num, 'pagenav' => $pagenav, 'limit' => array(($page - 1) * $limit, $limit));
    }

    function adminPageList($tableName, $whereData, $pageNow, $data = array('limit' => '', 'sql' => '', 'maxPage' => false, 'deflimit' => ''))
    {

        $limit      =   !empty($data['limit']) ? $data['limit'] : '';
        $sql        =   !empty($data['sql']) ? $data['sql'] : '';
        $maxPage    =   !empty($data['maxPage']) ? $data['maxPage'] : '';
        $deflimit   =   !empty($data['deflimit']) ? intval($data['deflimit']) : intval($this->config['sy_listnum']);

        $limit      =   $limit == '' ? intval($this->config['sy_listnum']) : intval($limit);

        $page       =   $pageNow < 1 ? 1 : intval($pageNow);

        if ($tableName == 'ad' && $this->config['did']) {

            $whereData['did'] = $this->config['did'];
        }

        if ($sql) {

            $result =   $this->DB_query_all($sql);
            $num    =   $result ? $result['num'] : 0;
        } else {

            //统计总数的时候 去掉order by条件
			$whereTotalData = $whereData;
			if($whereData['orderby']){
				unset($whereTotalData[orderby]);
			}
            $num    =   $this->select_num($tableName, $whereTotalData);
        }

        $mpage      =   ceil($num / $limit); // 计算最后的页数
        if ($maxPage) { // 超出最大页是否要处理
            $page > $mpage && $page = $mpage;
        }

        $page_sizes =   array($deflimit, 2 * $deflimit, 3 * $deflimit, 4 * $deflimit);

        return array('total' => $num,'page_sizes'=>$page_sizes,'page_size'=>$limit,'limit' => array(($page - 1) * $limit, $limit),'page' => $page,'last_page' => $mpage);
    }

    /***********gengzs start ****************/

    /**
     * @param array $get $_GET数组
     * @return array 赋值给 $where['limit']
     */
    function pageLimit($get){
        $limit = $this->limit($get);
        $page = $this->page($get);

        return array(($page - 1) * $limit, $limit);
    }
    /**
     * 当前页
     * @param array $get $_GET数组
     * @return int
     */
    public function page($get) {
        $page = 1;
        if (isset($get['page'])) {
            $page = max(1, intval($get['page']));
        }
        return intval($page);
    }

    /**
     * 请求条数
     * @param array $get $_GET数组
     * @return int
     */
    public function limit($get) {
        $limit = $this->config['sy_listnum'];
        if (isset($get['pageSize'])) {
            $limit = intval($get['pageSize']);
            if ($limit < 1) {
                $limit = $this->config['sy_listnum'];
            }
        }
        if (isset($get['limit'])) {
            $limit = intval($get['limit']);
            if ($limit < 1) {
                $limit = $this->config['sy_listnum'];
            }
        }
        return intval($limit);
    }
    /**********gengzs end *****************/


}

?>
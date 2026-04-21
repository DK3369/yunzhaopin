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
class index_controller extends zph_controller
{

    function index_action()
    {
        $zphM   =   $this->MODEL('zph');
        $this->Zphpublic_action();
        $zphList    = $zphM->getZphList();
        $newZphInfo = $zphM->getNewZph();
        $this->yunset("zphList", $zphList);
        $this->yunset("newZph", $newZphInfo);
        $this -> seo('zph');

        //判断我是否有简历
        $eData    =   array(
            'field'   => '`lastupdate`,`jobstatus`,`id`,`name`'
        );
        $resumeM        =   $this -> MODEL('resume');
        $rlist  =  $resumeM -> getExpectByUid($this->uid,$eData);
        $this -> yunset('rlist',$rlist);
        
        $this -> yun_tpl(array('index'));
    }

    //往期回顾
    function review_action()
    {
        $this -> Zphpublic_action();
        $this -> seo('zph');
        $this -> yun_tpl(array('review'));
    }

    /**
     * @desc    招聘会预定
     */
    function reserve_action()
    {
        $this -> Zphpublic_action();
        $id         =   intval($_GET['id']);
        $zphM       =   $this->MODEL('zph');

        $row        =   $zphM->getInfo(array( 'id' => $id ), array( 'pic' => 1 ));
        $this -> yunset('Info', $row);

        $rows       =   $zphM->getZphPicList(array('zid' => $id));
        $this -> yunset('Image_info', $rows);

        $space      =   $zphM->getZphSpaceInfo(array('id' => $row['sid']), array('pic' => 1,'field' => '`pic`,`content`'));
        $this -> yunset('space', $space);

        $spacelist  =   $zphM->getZphSpaceList(array('keyid' => $row['sid'], 'orderby' => 'sort,asc'), array( 'id' => $id, 'utype' => 'index' ));

        $this -> yunset('spacelist', $spacelist);

        if ($this->usertype == 2) {
            $comInfo    =   $this->MODEL('company')->getCompanyInfo(array('uid'=>$this->uid),array('field'=>'`package`,`r_status`,`vipetime`'));
            $ratingM    =   $this->MODEL('rating');
            if (!empty($comInfo['package'])){

                $ratingList =   $ratingM->getList(array('id' => array('in', $this->comInfo['package']), 'orderby' => array('type,asc', 'sort,desc')));
            }else{

                $ratingList =   $ratingM->getList(array('display' => 1, 'orderby' => array('type,asc', 'sort,desc')));
            }
            $rating_1 = $rating_2 = $raV = array();

            if (! empty($ratingList)) {

                foreach ($ratingList as $ratingV) {

                    $raV[$ratingV['id']] = $ratingV;

                    if ($ratingV['category'] == 1) {

                        if ($ratingV['type'] == 1) {

                            $rating_1[] = $ratingV;
                        } elseif ($ratingV['type'] == 2) {

                            $rating_2[] = $ratingV;
                        }
                    }
                }
            }
            if (!empty($this->config['com_package_open']) && empty($comInfo['package'])){

                $packageOpenArr =   explode(',', $this->config['com_package_open']);
                if (in_array($comInfo['rating'], $packageOpenArr) || ($comInfo['vipetime'] > 0 && $comInfo['vipetime'] < time() && in_array('999', $packageOpenArr))){

                    $rating_1   =   array();
                    $rating_2   =   array();
                }
            }
            if (empty($rating_1) && empty($rating_2) && !empty($this->config['com_package_open'])){
                $this->yunset('noPermission', 1);
            }
            if ($comInfo['r_status'] == 4){
                $this->yunset('isPaused', 1);
            }
            if ((int)$comInfo['crm_uid'] > 0){

                $crmInfo    =   $this->obj->select_once('admin_user', array('uid' => $comInfo['crm_uid']), '`name`,`moblie`,`ewm`');
                $consultant =   array('name' => $crmInfo['name'], 'mobile' => $crmInfo['moblie'], 'wechat' => !empty($crmInfo['ewm']) ? checkpic($crmInfo['ewm']): '');
                $this->yunset(array('consultant' => $consultant, 'crm' => 1));
            }else{

                $this->yunset(array('crm' => 2, 'webTel' => $this->config['sy_comwebtel']));
            }
            $this->yunset('rating_1', $rating_1);
            $this->yunset('rating_2', $rating_2);

            $statisM    =   $this->MODEL('statis');
            $statis     =   $statisM->getInfo($this->uid, array( 'usertype' => 2));

            if (! empty($statis)) {
                $discount   =   isset($raV[$statis['rating']]) ? $raV[$statis['rating']] : array();
                $this -> yunset('discount', $discount);
                $this -> yunset('statis', $statis);
            }

            $add    =   $ratingM->getComSerDetailList(array('orderby' => array('type,asc', 'sort,desc')), array('pack' => '1'));
            $this -> yunset('add', $add);

            
        }

        $data['zph_title']  =   $row['title'];
        $data['zph_desc']   =   $this->GET_content_desc($row['body']); // 描述
        $this -> data       =   $data;
        $this -> seo("zph_reserve");
        $this -> yun_tpl(array('reserve'));
    }

    function getComList_action(){
        $this -> Zphpublic_action();

        $id     =   intval($_POST['zid']);
        $return =   array();
        if($id){
            $page   =   intval($_POST['page']);
        
            $zphM   =   $this->MODEL('zph');

            
            $zclimit            =   20;
            $zcwhere            =   array();
            $zcwhere['zid']     =   $id;
            $zcwhere['status']  =   1;
            $zcwhere['orderby'] =   array('sort,desc','ctime,asc');

            $zcnum              =   $zphM->getZphComNum($zcwhere);

            if($page > 0){
                    
                $pagenav  =  ($page - 1) * $zclimit;
                $zcwhere['limit']  =  array($pagenav,$zclimit);
                
            }else{
                
                $zcwhere['limit']  =  $zclimit;
            }

            $zcList             =   $zphM -> getZphCompanyList($zcwhere);
            $return['zclist']   =   $zcList;
            $return['error']    =   1;
            $return['msg']      =   '';
        }else{
            $return['zclist']   =   array();
            $return['error']    =   0;
            $return['msg']      =   '参数错误请重试';
        }

        echo json_encode($return);exit();
    }
    function getJobList_action(){
        $this -> Zphpublic_action();

        $id     =   intval($_POST['zid']);
        $return =   array();
        if($id){
            $page   =   intval($_POST['page']);
            $keyword   =   trim($_POST['keyword']);
            $zphM   =   $this->MODEL('zph');

            
            $zjwhere            =   array();
            $zjwhere['zid']     =   $id;
            $zjwhere['status']  =   1;
            $zjwhere['orderby'] =   array('sort,desc','ctime,asc');

            $zjwhereData        =   array('zwhereData'=>$zjwhere);

            $zjlimit            =   40;
            $jwhere['state']    =   1;
            $jwhere['status']   =   0;
            $jwhere['r_status'] =   1;
            if($page > 0){
                    
                $pagenav  =  ($page - 1) * $zjlimit;
                $jwhere['limit']  =  array($pagenav,$zjlimit);
                
            }else{
                
                $jwhere['limit']  =  $zjlimit;
            }
            if($keyword){
                $jwhere['PHPYUNBTWSTART_a'] = '';
                $jwhere['name']  =   array('like',$keyword);
                $jwhere['com_name']  =   array('like',$keyword,'OR');
                $jwhere['PHPYUNBTWEND_a']  = '';
            }
            $jwhere['orderby']  =   'lastupdate,desc';

            $zjwhereData['jwhereData'] = $jwhere;

            $zjList             =   $zphM -> getZphJobList($zjwhereData);

            $return['zjlist']   =   $zjList['list'];
            $return['error']    =   1;
            $return['msg']      =   '';
        }else{
            $return['zjlist']   =   array();
            $return['error']    =   0;
            $return['msg']      =   '参数错误请重试';
        }

        echo json_encode($return);exit();
    }
}
?>
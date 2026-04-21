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

class msgNum_model extends model
{

    function getmsgNum($uid, $usertype, $data = array())
    {

        //  设置个人中心右上角用户名展示未读系统消息数
        $msgNum =   0;
        $arr    =   array();

        if ($uid) {

            //  未读系统消息
            $sysMsgNum  =   $this->select_num('sysmsg', array('fa_uid' => $uid, 'usertype' => $usertype, 'remind_status' => '0'));
            if ($sysMsgNum > 0) {
                $msgNum             +=  $sysMsgNum;
                $arr['sysmsgNum']   =   $sysMsgNum;
            }
        	 

            if ($usertype == 1) {
                //  邀请面试
                $userid_msg         =   $this->select_num('userid_msg', array('uid' => $uid,'isdel'=>9,'is_browse' => '1'));
                if ($userid_msg > 0) {
                    $msgNum                 +=  $userid_msg;
                    $arr['userid_msgNum']   =   $userid_msg;
                }
                //  求职咨询
                if($this->config['com_message'] == 1){
                    $usermsg        =   $this->select_num('msg', array('uid' => $uid, 'status' => 1, 'user_remind_status' => '0'));
                    if ($usermsg > 0) {
                        $msgNum                 +=  $usermsg;
                        $arr['usermsgNum']      =   $usermsg;
                    }
                }
                // 邀请面试总数量
                $yqnum              =   $this->select_num('userid_msg', array('uid' => $uid,'isdel'=>9));
                $arr['yqnum']       =   $yqnum;

                // 未读邀请面试数量
                $wkyqnum            =   $this->select_num('userid_msg', array('uid' => $uid,'is_browse'=>'1','isdel'=>9));
                $arr['wkyqnum']     =   $wkyqnum;

                // 申请职位数量
                $sq_nums            =   $this->select_num('userid_job', array('uid' => $uid,'isdel'=>9));
                $arr['sq_jobnum']   =   $sq_nums;

                // 收藏的职位数量
                $favwhere['uid']    =   $uid;
                if ($data['type']) {//小程序只显示普通职位
                    $favwhere['type']   =   $data['type'];
                }
                $fav_jobnum         =   $this->select_num('fav_job', $favwhere);
                $arr['fav_jobnum']  =   $fav_jobnum;

                // 关注数量
                require_once('atn.model.php');
                $atnM   =   new atn_model($this->db, $this->def);

                $where['uid']               =   $uid;
                $where['sc_usertype']       =   '2';
                
                $atncomnum                  =   $atnM->getAtnNum($where);
               
                $atn_num                =   $atncomnum;
                $arr['atn_num']         =   $atn_num;
            } elseif ($usertype == 2) {

                //职位申请数
                $jobApplyNum            =   $this->select_num('userid_job', array('com_id' => $uid,'isdel'=>9,'type' => array('<>', 3), 'is_browse' => '1'));
                if ($jobApplyNum > 0) {
                    $msgNum             +=  $jobApplyNum;
                    $arr['jobApplyNum'] =   $jobApplyNum;
                }

                //求职咨询数
                if($this->config['com_message'] == 1){
                    $qzwhere['job_uid']         =   $uid;
                    $qzwhere['status']          =   1;
                    $qzwhere['del_status']      =   0;
                    $qzwhere['PHPYUNBTWSTART']  =   '';
                    $qzwhere['reply'][]         =   array('isnull');
                    $qzwhere['reply'][]         =   array('=', '', 'OR');
                    $qzwhere['PHPYUNBTWEND']    =   '';
                    $jobAskNum                  =   $this->select_num('msg', $qzwhere);
                    if ($jobAskNum > 0) {
                        $msgNum             +=  $jobAskNum;
                        $arr['jobAskNum']   =   $jobAskNum;
                    }
                }
                 
                $sqnum                  =   $this->select_num('userid_job', array('com_id' => $uid,'isdel'=>9,'type' => array('<>', 3)));
                $arr['sqnum']           =   $sqnum;

                $invite_num             =   $this->select_num('userid_msg', array('fid' => $uid,'isdel'=>9));
                $arr['invite_num']      =   $invite_num;

                $msgnum                 =   $this->select_num('msg', array('job_uid' => $uid, 'status' => 1));
                $arr['msgnum']          =   $msgnum;

                $attention_menum        =   $this->select_num('atn', array('sc_uid' => $uid));
                $arr['attention_menum'] =   $attention_menum;

                $companyjobnum          =   $this->select_num('company_job', array('uid' => $uid, 'state' => 1, 'status' => 0, 'r_status' => 1));
                $arr['companyjobnum']   =   $companyjobnum;

                $arr['look_jobnum']     =   $this->select_num('look_job', array('com_id' => $uid));

                $talent_pool_num        =   $this->select_num('talent_pool', array('cuid' => $uid));
                $arr['talent_pool_num'] =   $talent_pool_num;

                require_once 'company.model.php';
                $comM           =   new company_model($this->db, $this->def);
                $hitsExpour     =   $comM->getHitsExpoure($uid);
                $arr['hits']    =   $hitsExpour['hits'];
                $arr['expoure'] =   $hitsExpour['expoure'];
            }
        }
        $arr['usertype']    =   $usertype;
        $arr['msgNum']      =   $msgNum;
        return $arr;
    }

    /**
     * @desc 设置管理后台右上角用户名展示未审核消息数
     */
    function msgNum()
    {

        $msgNum =   0;
        $arr    =   array();

        //待审核企业
        $company                =   $this->select_num('company', array('r_status' => '0'));
        if ($company > 0) {
            $msgNum             +=  $company;
            $arr['company']     =   $company;
        }

        //待审核职位
        $company_job            =   $this->select_num('company_job', array('state' => '0'));
        if ($company_job > 0) {
            $msgNum             +=  $company_job;
            $arr['company_job'] =   $company_job;
        }

        //待审核兼职
        $partjob                =   $this->select_num('partjob', array('state' => '0'));
        if ($partjob > 0) {
            $msgNum             +=  $partjob;
            $arr['partjob']     =   $partjob;
        }

        //待审核企业认证
        $company_cert           =   $this->select_num('company_cert', array('status' => '0', 'type' => '3'));
        if ($company_cert > 0) {
            $msgNum             +=  $company_cert;
            $arr['company_cert']=   $company_cert;
        }

        //待审核企业logo
        $comlogo                = $this->select_num('company', array('logo' => array('<>', ''), 'logo_status' => '1'));
        if ($comlogo > 0) {
            $msgNum             +=  $comlogo;
            $arr['comlogo']     =   $comlogo;
        }
        //待审核企业环境
        $comshow                =   $this->select_num('company_show', array('picurl' => array('<>', ''), 'status' => '1'));
        if ($comshow > 0) {
            $msgNum             +=  $comshow;
            $arr['comshow']     =   $comshow;
        }
        //待审核企业横幅
        $combanner              =   $this->select_num('banner', array('pic' => array('<>', ''), 'status' => '1'));
        if ($combanner > 0) {
            $msgNum             +=  $combanner;
            $arr['combanner']   =   $combanner;
        }

        //待审核企业产品
        $company_product            =   $this->select_num('company_product', array('status' => '0'));
        if ($company_product > 0) {
            $msgNum                 +=  $company_product;
            $arr['company_product'] =   $company_product;
        }

        //待审核企业新闻
        $company_news           =   $this->select_num('company_news', array('status' => '0'));
        if ($company_news > 0) {
            $msgNum             +=  $company_news;
            $arr['company_news']=   $company_news;
        }


        //待审核简历
        $resume_expect              =   $this->select_num('resume_expect', array('state' => '0'));
        if ($resume_expect > 0) {
            $msgNum                 +=  $resume_expect;
            $arr['resume_expect']   =   $resume_expect;
        }

        //待审核委托简历
        $resumetrust                =   $this->select_num('user_entrust', array('status' => '0'));
        if ($resumetrust > 0) {
            $msgNum                 +=  $resumetrust;
            $arr['resumetrust']     =   $resumetrust;
        }

        //错误日志
        $errlog                     =   $this->select_num('error_log', array('isread' => 0));

        if ($errlog > 0) {
            $msgNum                 +=  $errlog;
            $arr['errlog']          =   $errlog;
        }


        //待审核个人认证
        $usercertNum                =   $this->select_num('resume', array('idcard_pic' => array('<>', ''), 'idcard_status' => '0'));
        if ($usercertNum > 0) {
            $msgNum                 +=  $usercertNum;
            $arr['usercertNum']     =   $usercertNum;
        }

        //会员申诉
        $appealnum                  =   $this->select_num('member', array('appealtime' => array('>', '0'), 'appealstate' => '1'));
        if ($appealnum > 0) {
            if (!$this->config['did'] || $this->config['did'] == 0) {
                $msgNum             +=  $appealnum;
            }
            $arr['appealnum']       =   $appealnum;
        }

        //待审核店铺招聘
        $once_job                   =   $this->select_num('once_job', array('status' => '0', 'edate' => array('>', time())));
        if ($once_job > 0) {
            $msgNum                 +=  $once_job;
            $arr['once_job']        =   $once_job;
        }

        //待审核普工简历
        $tiny                       =   $this->select_num('resume_tiny', array('status' => '0'));
        if ($tiny > 0) {
            $msgNum                 +=  $tiny;
            $arr['tiny']            =   $tiny;
        }

        
        //待审核参会企业
        $zphcom                     =   $this->select_num('zhaopinhui_com', array('status' => '0'));
        if ($zphcom > 0) {
            $msgNum                 +=  $zphcom;
            $arr['zphcom']          =   $zphcom;
        }

        //待审核问答
        $ask                        =   $this->select_num('question', array('state' => '0'));
        if ($ask > 0) {
            $msgNum                 +=  $ask;
            $arr['ask']             =   $ask;
        }

        //待付款订单
        $order                      =   $this->select_num('company_order', array('order_state' => '1'));
        if ($order > 0) {
            $msgNum                 +=  $order;
            $arr['order']           =   $order;
        }

       
        //待处理举报职位
        $rjwhere['usertype']        =   1;
        $rjwhere['type']            =   0;
		$rjwhere['status']			=	0;
        $rjwhere['PHPYUNBTWSTART']  =   '';
        $rjwhere['result'][]        =   array('=', '');
        $rjwhere['result'][]        =   array('isnull', '', 'OR');
        $rjwhere['PHPYUNBTWEND']    =   '';
        $reportjob                  =   $this->select_num('report', $rjwhere);
        if ($reportjob > 0) {
            $msgNum                 +=  $reportjob;
            $arr['reportjob']       =   $reportjob;
        }

        //待处理举报简历
        $rrwhere['usertype']        =   2;
        $rrwhere['type']            =   0;
		$rrwhere['status']			=	0;
        $rrwhere['PHPYUNBTWSTART']  =   '';
        $rrwhere['result'][]        =   array('=', '');
        $rrwhere['result'][]        =   array('isnull', '', 'OR');
        $rrwhere['PHPYUNBTWEND']    =   '';
        $reportresume               =   $this->select_num('report', $rrwhere);
        if ($reportresume > 0) {
            $msgNum                 +=  $reportresume;
            $arr['reportresume']    =   $reportresume;
        }
        //待处理举报问答
        $rtwhere['status']          =   0;
        $rtwhere['type']            =   1;
		$rtwhere['status']			=	0;
        $rtwhere['PHPYUNBTWSTART']  =   '';
        $rtwhere['result'][]        =   array('=', '');
        $rtwhere['result'][]        =   array('isnull', '', 'OR');
        $rtwhere['PHPYUNBTWEND']    =   '';
        $reportask                  =   $this->select_num('report', $rtwhere);
        if ($reportask > 0) {
            $msgNum                 +=  $reportask;
            $arr['reportask']       =   $reportask;
        }
        //待处理举报顾问
        $rgwhere['status']          =   0;
        $rgwhere['type']            =   2;
		$rgwhere['status']			=	0;
        $rgwhere['PHPYUNBTWSTART']  =   '';
        $rgwhere['result'][]        =   array('=', '');
        $rgwhere['result'][]        =   array('isnull', '', 'OR');
        $rgwhere['PHPYUNBTWEND']    =   '';
        $reportgw                   =   $this->select_num('report', $rgwhere);
        if ($reportgw > 0) {
            $msgNum                 +=  $reportgw;
            $arr['reportgw']        =   $reportgw;
        }

        //待审核个人头像
        $userpic                    =   $this->select_num('resume', array('photo' => array('<>', ''),'defphoto'=>1,'photo_status' => '1'));
        if ($userpic > 0) {
            $msgNum                 +=  $userpic;
            $arr['userpic']         =   $userpic;
        }
        //待审核个人作品
        $usershow                   =   $this->select_num('resume_show', array('picurl' => array('<>', ''), 'status' => '1'));
        if ($usershow > 0) {
            $msgNum                 +=  $usershow;
            $arr['usershow']        =   $usershow;
        }
        
        if (!$this->config['did']) {
            //待审核友情链接
            $linkNum                =   $this->select_num('admin_link', array('link_state' => '0'));
            if ($linkNum > 0) {
                $msgNum             +=  $linkNum;
                $arr['linkNum']     =   $linkNum;
            }
            //待审核商品兑换
            $redeem                 =   $this->select_num('change', array('status' => '0'));
            if ($redeem > 0) {
                $msgNum             +=  $redeem;
                $arr['redeem']      =   $redeem;
            }
            //待付款订单
            $specialcom             =   $this->select_num('special_com', array('status' => '0'));
            if ($specialcom > 0) {
                $msgNum             +=  $specialcom;
                $arr['specialcom']  =   $specialcom;
            }
            //待审核转换申请
            $userchangenum              =   $this->select_num('user_change', array('status' => '0'));
            if ($userchangenum > 0) {
                $msgNum                 +=  $userchangenum;
                $arr['userchangenum']   =   $userchangenum;
            }
            //待处理意见反馈
            $handlenum              =   $this->select_num('advice_question', array('status' => '1'));
            if ($handlenum > 0) {
                $msgNum             +=  $handlenum;
                $arr['handlenum']   =   $handlenum;
            }
        }
        //待处理注销账号
        $logout                     =   $this->select_num('member_logout', array('status' => 1));
        if ($logout > 0) {
            $msgNum                 +=  $logout;
            $arr['logout']          =   $logout;
        }

        //待审核面试模板
        $yqmb_msg                   =   $this->select_num('yqmb', array('status' => 0));
        if ($yqmb_msg > 0) {
            $msgNum                 +=  $yqmb_msg;
            $arr['yqmb_msg']         =  $yqmb_msg;
        }

        //待审核求职咨询
        $usermsg_msg                =   $this->select_num('msg', array('status' => 0));
        if ($usermsg_msg > 0) {
            $msgNum                 +=  $usermsg_msg;
            $arr['usermsg_msg']     =   $usermsg_msg;
        }

        //待审核问答回复
        $answer_msg                 =   $this->select_num('answer', array('status' => 0));
        if ($answer_msg > 0) {
            $msgNum                 +=  $answer_msg;
            $arr['answer_msg']      =   $answer_msg;
        }

        //待审核问答评价
        $answerreview_msg           =   $this->select_num('answer_review', array('status' => 0));
        if ($answerreview_msg > 0) {
            $msgNum                 +=  $answerreview_msg;
            $arr['answerreview_msg']=   $answerreview_msg;
        }

        //待查看预警
        $warning                    =   $this->select_num('warning', array('status' => 2));
        if ($warning > 0) {
            $msgNum                 +=  $warning;
            $arr['warning']         =   $warning;
        }

       	$arr['msgNum']  =   $msgNum;
        return json_encode($arr);
    }

    function companyNum($where = array())
    {

        $arr = array();

        //企业总数
        $companyAllNum = $this->select_num('company', $where);
        if ($companyAllNum > 0) {
            $arr['companyAllNum']       =   $companyAllNum;
        }

        //待审核企业
        $where['r_status'] = 0;
        $companyStatusNum1 = $this->select_num('company', $where);
        if ($companyStatusNum1 > 0) {
            $arr['companyStatusNum1'] = $companyStatusNum1;
        }

        //未通过企业
        $where['r_status'] = 3;
        $companyStatusNum2 = $this->select_num('company', $where);
        if ($companyStatusNum2 > 0) {
            $arr['companyStatusNum2'] = $companyStatusNum2;
        }

        //锁定企业
        $where['r_status'] = 2;
        $companyStatusNum3 = $this->select_num('company', $where);
        if ($companyStatusNum3 > 0) {
            $arr['companyStatusNum3'] = $companyStatusNum3;
        }
        return json_encode($arr);
    }

    function hotNum()
    {

        $arr    =   array();
        //名企总数
        $hotAllNum  =   $this->select_num('hotjob');
        if ($hotAllNum > 0) {
            $arr['hotAllNum']   =   $hotAllNum;
        }
        //已过期名企
        $hoted      =   $this->select_num('hotjob', array('time_end' => array('<=', time())));
        if ($hoted > 0) {
            $arr['hoted']       =   $hoted;
        }

        return json_encode($arr);
    }

    function comCertNum($isArray = false)
    {
        $arr    =   array();
        //企业认证总数
        $comCertAll =   $this->select_num('company_cert', array('type' => '3'));
        if ($comCertAll > 0) {
            $arr['comCertAll']  =   $comCertAll;
        }
        //未审核企业认证
        $comCert1   =   $this->select_num('company_cert', array('type' => '3', 'status' => '0'));
        if ($comCert1 > 0) {
            $arr['comCert1']    =   $comCert1;
        }
        //未通过认证
        $comCert2   =   $this->select_num('company_cert', array('type' => '3', 'status' => '2'));
        if ($comCert2 > 0) {
            $arr['comCert2']    =   $comCert2;
        }

        return $isArray ? $arr : json_encode($arr);
    }

    function jobNum()
    {
        $arr    =   array();

        //职位总数
        $jobAllNum      =   $this->select_num('company_job');
        if ($jobAllNum > 0) {
            $arr['jobAllNum']       =   $jobAllNum;
        }
        //待审核职位
        $jobStatusNum1  =   $this->select_num('company_job', array('state' => '0'));
        if ($jobStatusNum1 > 0) {
            $arr['jobStatusNum1']   =   $jobStatusNum1;
        }
        // 未通过职位
        $jobStatusNum2  =   $this->select_num('company_job', array('state' => '3'));
        if ($jobStatusNum2 > 0) {
            $arr['jobStatusNum2']   =   $jobStatusNum2;
        }
        //下架职位
        $jobStatusNum3  =   $this->select_num('company_job', array('status' => '1'));
        if ($jobStatusNum3 > 0) {
            $arr['jobStatusNum3']   =   $jobStatusNum3;
        }

        return json_encode($arr);
    }

    function partNum()
    {

        $arr    =   array();
        //兼职总数
        $partAllNum     =   $this->select_num('partjob');
        if ($partAllNum > 0) {
            $arr['partAllNum']      =   $partAllNum;
        }
        //待审核兼职
        $partStatusNum1 =   $this->select_num('partjob', array('state' => '0'));
        if ($partStatusNum1 > 0) {
            $arr['partStatusNum1']  =   $partStatusNum1;
        }
        //未通过兼职
        $partStatusNum2 =   $this->select_num('partjob', array('state' => '3'));
        if ($partStatusNum2 > 0) {
            $arr['partStatusNum2']  =   $partStatusNum2;
        }
        //已过期兼职
        $ewhere =   array(
            'PHPYUNBTWSTART_A'  =>  '',
            'edate'             =>  array(
                '0' =>  array('<', time(), 'AND'),
                '1' =>  array('>', '0', 'AND')
            ),
            'PHPYUNBTWEND_A'    =>  '',
        );
        $partStatusNum3             =   $this->select_num('partjob', $ewhere);
        if ($partStatusNum3 > 0) {
            $arr['partStatusNum3']  =   $partStatusNum3;
        }
        return json_encode($arr);
    }

    function orderSum($where=array())
    {
        $order_state_where = isset($where['order_state'])?$where['order_state']:'';
        unset($where['order_state']);
        if($order_state_where!=''){
            $where['order_state'][]= array('=',$order_state_where);
        }
        

        $arr = array(); 
        //订单总额
        $where1 = $where;

        $where1['order_price'] = array('>','0');

        $orderAll = $this->select_once('company_order',$where1, 'sum(`order_price`) as `pricesum`');
        if ($orderAll['pricesum'] > 0) {
            $arr['orderPriceAll'] = number_format($orderAll['pricesum'],2);
        }
        //已支付金额
        $where2 = $where;
        $where2['order_price'] = array('>','0');
        $where2['order_state'][] = array('=',2,'and');
        
        $orderPayed = $this->select_once('company_order',$where2, 'sum(`order_price`) as `orderPayed`');
        
        if ($orderPayed['orderPayed'] > 0) {
            $arr['orderPayed'] = number_format($orderPayed['orderPayed'],2);
        }
        //待支付金额
        $where3 = $where;
        $where3['order_price'] = array('>','0');
        $where3['order_state'][] = array('=',1,'and');
        $orderPay = $this->select_once('company_order',$where3, 'sum(`order_price`) as `orderPay`');
        if ($orderPay['orderPay'] > 0) {
            $arr['orderPay'] = number_format($orderPay['orderPay'],2);
        }
        //等待确认金额
        $where4 = $where;
        $where4['order_price'] = array('>','0');
        $where4['order_state'][] = array('=',3,'and');
        $orderPaying = $this->select_once('company_order',$where4, 'sum(`order_price`) as `orderPaying`');
        if ($orderPaying['orderPaying'] > 0) {
            $arr['orderPaying'] = number_format($orderPaying['orderPaying'],2);
        }

        return json_encode($arr);
    }

    function userNum()
    {
        $arr = array();
        //个人总数
        $userAllNum = $this->select_num('resume');
        if ($userAllNum > 0) {
            $arr['userAllNum'] = $userAllNum;
        }
        //锁定个人
        $userStatusNum3 = $this->select_num('resume', array('r_status' => '2'));
        if ($userStatusNum3 > 0) {
            $arr['userStatusNum3'] = $userStatusNum3;
        }
        return json_encode($arr);
    }

    function resumeNum()
    {
        $arr = array();
        //简历总数
        $resumeAllNum = $this->select_num('resume_expect');
        if ($resumeAllNum > 0) {
            $arr['resumeAllNum'] = $resumeAllNum;
        }
        //待审核简历
        $resumeStatusNum1 = $this->select_num('resume_expect', array('state' => '0'));
        if ($resumeStatusNum1 > 0) {
            $arr['resumeStatusNum1'] = $resumeStatusNum1;
        }
        //未通过简历
        $resumeStatusNum2 = $this->select_num('resume_expect', array('state' => '3'));
        if ($resumeStatusNum2 > 0) {
            $arr['resumeStatusNum2'] = $resumeStatusNum2;
        }
        //锁定简历
        $resumeStatusNum3 = $this->select_num('resume_expect', array('r_status' => '2'));
        if ($resumeStatusNum3 > 0) {
            $arr['resumeStatusNum3'] = $resumeStatusNum3;
        }

        //未成年
        $datetime=strtotime('-16 years');
        $resumeTeenNum = $this->select_num('resume_expect', array('birthday' => array('unixtime','>',$datetime)));
        if ($resumeTeenNum > 0) {
            $arr['resumeTeenNum'] = $resumeTeenNum;
        }
        return json_encode($arr);
    }

    function idCardNum()
    {
        $arr = array();
        //个人认证总数
        $idCardAll = $this->select_num('resume', array('idcard_pic' => array('<>', ''), 'idcard_status' => '1'));
        if ($idCardAll > 0) {
            $arr['idCardAll'] = $idCardAll;
        }
        //未审核个人认证
        $idCardNum1 = $this->select_num('resume', array('idcard_pic' => array('<>', ''), 'idcard_status' => '0'));
        if ($idCardNum1 > 0) {
            $arr['idCardNum1'] = $idCardNum1;
        }
        //未通过身份认证
        $idCardNum2 = $this->select_num('resume', array('idcard_pic' => array('<>', ''), 'idcard_status' => '2'));
        if ($idCardNum2 > 0) {
            $arr['idCardNum2'] = $idCardNum2;
        }

        return json_encode($arr);
    }

    function trustNum()
    {
        $arr = array();
        //简历总数
        $resumeAllNum = $this->select_num('user_entrust');
        if ($resumeAllNum > 0) {
            $arr['resumeAllNum'] = $resumeAllNum;
        }
        //待审核委托
        $resumeStatusNum1 = $this->select_num('user_entrust', array('status' => '0'));
        if ($resumeStatusNum1 > 0) {
            $arr['resumeStatusNum1'] = $resumeStatusNum1;
        }
        //未接受委托
        $resumeStatusNum2 = $this->select_num('user_entrust', array('status' => '2'));
        if ($resumeStatusNum2 > 0) {
            $arr['resumeStatusNum2'] = $resumeStatusNum2;
        }

        return json_encode($arr);
    }

     

    function gresumeNum()
    {
        $arr = array();
        //优质人才简历总数
        $resumeAllNum = $this->select_num('resume_expect', array('height_status' => array('<>', '0')));
        if ($resumeAllNum > 0) {
            $arr['resumeAllNum'] = $resumeAllNum;
        }
        //待审核简历
        $resumeStatusNum1 = $this->select_num('resume_expect', array('height_status' => '1'));
        if ($resumeStatusNum1 > 0) {
            $arr['resumeStatusNum1'] = $resumeStatusNum1;
        }
        //未通过简历
        $resumeStatusNum2 = $this->select_num('resume_expect', array('height_status' => '3'));
        if ($resumeStatusNum2 > 0) {
            $arr['resumeStatusNum2'] = $resumeStatusNum2;
        }

        return json_encode($arr);
    }

    
    function onceNum()
    {
        $arr = array();
        //店铺总数
        $onceAllNum = $this->select_num('once_job');
        if ($onceAllNum > 0) {
            $arr['onceAllNum'] = $onceAllNum;
        }
        //未审核
        $onceStatusNum1 = $this->select_num('once_job', array('status' => '0', 'edate' => array('>', time())));
        if ($onceStatusNum1 > 0) {
            $arr['onceStatusNum1'] = $onceStatusNum1;
        }
        //已过期
        $onceStatusNum2 = $this->select_num('once_job', array('edate' => array('<', time())));
        if ($onceStatusNum2 > 0) {
            $arr['onceStatusNum2'] = $onceStatusNum2;
        }
        return json_encode($arr);
    }

    function tinyNum()
    {
        $arr = array();
        //普工总数
        $tinyAllNum = $this->select_num('resume_tiny');
        if ($tinyAllNum > 0) {
            $arr['tinyAllNum'] = $tinyAllNum;
        }
        //未审核
        $tinyStatusNum = $this->select_num('resume_tiny', array('status' => '0'));
        if ($tinyStatusNum > 0) {
            $arr['tinyStatusNum'] = $tinyStatusNum;
        }
        return json_encode($arr);
    }

   
   

    function memNum()
    {
        $arr    =   array();
        //会员总数
        $memAllNum      =   $this->select_num('member', array('pid' => 0));
        if ($memAllNum > 0) {
            $arr['memAllNum']       =   $memAllNum;
        }
       

        //待审核会员
        $memStatusNum1  =   $this->select_num('member', array('status' => '0'));
        if ($memStatusNum1 > 0) {
            $arr['memStatusNum1']   =   $memStatusNum1;
        }
        //未通过会员
        $memStatusNum2  =   $this->select_num('member', array('status' => '3'));
        if ($memStatusNum2 > 0) {
            $arr['memStatusNum2']   =   $memStatusNum2;
        }
        //锁定会员
        $memStatusNum3  =   $this->select_num('member', array('status' => '2'));
        if ($memStatusNum3 > 0) {
            $arr['memStatusNum3']   =   $memStatusNum3;
        }
        return json_encode($arr);
    }
    function memNumV1()
    {
        $arr    =   array();
        //会员总数
        $arr['memAllNum']      =   $this->select_num('member', array('pid' => 0));

        //锁定会员
        $arr['memStatusNum3']  =   $this->select_num('member', array('status' => '2'));

        return $arr;
    }

    function memchangeNum()
    {
        $arr    =   array();
        //供求任务总数
        $memAllNum      =   $this->select_num('user_change');
        if ($memAllNum > 0) {
            $arr['memAllNum']       =   $memAllNum;
        }
        //已拒绝申请
        $memStatusNum3  =   $this->select_num('user_change', array('status' => '0'));
        if ($memStatusNum3 > 0) {
            $arr['memStatusNum3']   =   $memStatusNum3;
        }
        return json_encode($arr);
    }

    function memchangeNumV1()
    {
        $arr = array();
        //供求任务总数
        $memAllNum = $this->select_num('user_change');
        $arr['memAllNum'] = $memAllNum;
        //已拒绝申请
        $memStatusNum3 = $this->select_num('user_change', array('status' => '2'));
        $arr['memStatusNum3'] = $memStatusNum3;
        return $arr;
    }



}

?>
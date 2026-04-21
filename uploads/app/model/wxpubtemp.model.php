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

class wxpubtemp_model extends model
{
    public   $pubtoolself_publiccolumn_map = array();
    public   $pubtoolself_publiccolumn = array();

    public   $onejobcolumn_map = array();
    public   $pubtoolself_jobcolumn_map = array();
    public   $pubtoolself_jobcolumn = array();

    public   $pubtoolself_resumecolumn = array();
    public   $pubtoolself_resumecolumn_map = array();

    public   $pubtoolself_companycolumn = array();
    public   $pubtoolself_companycolumn_map = array();

    public   $pubtoolself_totalcolumn = array();
    public   $pubtoolself_totalcolumn_map = array();

    function __construct($db,$def,$logininfo = array(),$tpl='') {

        parent::__construct($db,$def,$logininfo,$tpl='');

        $this->onejobcolumn_map = array(
                '{职位名称}'=>array('php'=>'name'),
                '{职位网址}'=>array(
                    'php'=>array(
                        'type'=>'url',
                        'urltype'=>'job',
                    )
                ),
                '{企业名称}'=>array('php'=>'com_name'),
                '{企业描述}'=>array('php'=>'com_desc'),
                '{企业网址}'=>array(
                    'php'=>array(
                        'type'=>'url',
                        'urltype'=>'company',
                    )
                ),
                '{薪资待遇}'=>array('php'=>'job_salary'),
                '{招聘人数}'=>array('php'=>'job_number'),
                '{年龄要求}'=>array('php'=>'job_age'),
                '{性别要求}'=>array('php'=>'job_sex'),
                '{经验要求}'=>array('php'=>'job_exp'),
                '{学历要求}'=>array('php'=>'job_edu'),
                '{一级城市}'=>array('php'=>'job_city_one'),
                '{二级城市}'=>array('php'=>'job_city_two'),
                '{三级城市}'=>array('php'=>'job_city_three'),
                '{联系电话}'=>array('php'=>'phone'),
                '{联系地址}'=>array('php'=>'address'),
                '{福利开始}'=>array(
                    'php'=>array(
                        'type'=>'foreach',
                        'from'=>'arraywelfare',
                        'begin'=>'1'
                    )
                ),
                '{职位福利}'=>array(
                    'php'=>array(
                        'type'=>'foreach',
                        'from'=>'arraywelfare',
                        'item'=>'welv'
                    )
                ),
                '{福利结束}'=>array(
                    'php'=>array(
                        'type'=>'foreach',
                        'from'=>'arraywelfare',
                        'end'=>'1'
                    )
                ),
                '{职位描述}'=>array('php'=>'description'),
            );

        //职位参数
        $this->pubtoolself_jobcolumn_map = array(
                '{职位名称}'=>array('php'=>'{yun:}$v.name{/yun}','js'=>'xx职位'),
                '{职位网址}'=>array('php'=>'{yun:}url m=wap c=job a=comapply id=$v.id{/yun}','js'=>Url('wap')),
                '{企业名称}'=>array('php'=>'{yun:}$v.com_name{/yun}','js'=>'xx企业'),
                '{企业描述}'=>array('php'=>'{yun:}$v.com_desc{/yun}','js'=>'企业简介就是对一个公司进行一个简单全面的介绍，这种介绍不是一句话带过，也不是长篇大论...'),
                '{企业网址}'=>array('php'=>'{yun:}url m=wap c=company a=show id=$v.uid{/yun}','js'=>Url('wap')),
                '{薪资待遇}'=>array('php'=>'{yun:}$v.job_salary{/yun}','js'=>'10000-15000'),
                '{招聘人数}'=>array('php'=>'{yun:}$v.job_number{/yun}','js'=>'1-2人'),
                '{年龄要求}'=>array('php'=>'{yun:}$v.job_age{/yun}','js'=>'35岁以上'),
                '{性别要求}'=>array('php'=>'{yun:}$v.job_sex{/yun}','js'=>'男'),
                '{经验要求}'=>array('php'=>'{yun:}$v.job_exp{/yun}','js'=>'5-10年'),
                '{学历要求}'=>array('php'=>'{yun:}$v.job_edu{/yun}','js'=>'本科'),
                '{一级城市}'=>array('php'=>'{yun:}$v.job_city_one{/yun}','js'=>'江苏'),
                '{二级城市}'=>array('php'=>'{yun:}$v.job_city_two{/yun}','js'=>'宿迁'),
                '{三级城市}'=>array('php'=>'{yun:}$v.job_city_three{/yun}','js'=>'沭阳'),
                '{联系电话}'=>array('php'=>'{yun:}$v.phone{/yun}','js'=>'0527-83698666'),
                '{联系地址}'=>array('php'=>'{yun:}$v.address{/yun}','js'=>'宿迁市沭阳县迎宾大道613号'),
                '{福利开始}'=>array('php'=>'{yun:}foreach from=$v.job_welfare item = welv{/yun}','js'=>'{forstart_1}'),
                '{职位福利}'=>array('php'=>'{yun:}$welv{/yun}','js'=>'职位福利'),
                '{福利结束}'=>array('php'=>'{yun:}/foreach{/yun}','js'=>'{forend_1}'),
                '{职位描述}'=>array(
                    'php'=>'{yun:}$v.job_description{/yun}',
                    'js'=>'职责描述：1.负责数据中心机房整体规划、部署，架构优化、容灾优化、性能优化；2.负责数据中心基础设施...'
                ),
                
            );

    
        $this->pubtoolself_jobcolumn = array(
                    'jobcolumn_name'        =>  array('职位名称','{职位名称}','job_column'),
                    'jobcolumn_jobwapurl'   =>  array('职位网址','{职位网址}','job_column'),
                    'jobcolumn_comname'     =>  array('企业名称','{企业名称}','job_column'),
                    'jobcolumn_comdesc'     =>  array('企业描述|字数','{str|企业描述|length:200}','job_column'),
                    'jobcolumn_comwapurl'   =>  array('企业网址','{企业网址}','job_column'),
                    'jobcolumn_salary'      =>  array('薪资待遇','{薪资待遇}','job_column'),
                    'jobcolumn_number'      =>  array('招聘人数','{招聘人数}','job_column'),
                    'jobcolumn_age'         =>  array('年龄要求','{年龄要求}','job_column'),
                    'jobcolumn_sex'         =>  array('性别要求','{性别要求}','job_column'),
                    'jobcolumn_exp'         =>  array('经验要求','{经验要求}','job_column'),
                    'jobcolumn_edu'         =>  array('学历要求','{学历要求}','job_column'),
                    'jobcolumn_city'        =>  array('地址','{一级城市}{二级城市}{三级城市}','job_column'),
                    'jobcolumn_address'     =>  array('联系地址','{联系地址}','job_column'),
                    'jobcolumn_phone'       =>  array('联系电话','{联系电话}','job_column'),
                    'jobcolumn_welfare'     =>  array('职位福利','{福利开始}{职位福利}{福利结束}','job_column'),
                    'jobcolumn_description' =>  array('职位描述|字数','{str|职位描述|length:200}','job_column'),
                    
                );
        //简历参数
        
        $this->pubtoolself_resumecolumn_map =   array(
                    
                    '{期望职位}'            =>  array('php'=>'{yun:}$v.list.customjob{/yun}','js'=>'xx职位'),
                    '{简历网址}'            =>  array('php'=>'{yun:}url m=wap c=resume a=show id=$v.list.id{/yun}','js'=>Url('wap')),
                    '{姓名}'              =>  array('php'=>'{yun:}$v.list.username_n{/yun}','js'=>'王xx'),
                    '{年龄}'              =>  array('php'=>'{yun:}$v.list.age{/yun}','js'=>'25岁'),
                    '{经验}'              =>  array('php'=>'{yun:}$v.list.user_exp{/yun}','js'=>'应届生经验'),
                    '{学历}'              =>  array('php'=>'{yun:}$v.list.useredu{/yun}','js'=>'初中学历'),
                    '{求职状态}'            =>  array('php'=>'{yun:}$v.list.jobstatus{/yun}','js'=>'在职'),
                    '{到岗时间}'            =>  array('php'=>'{yun:}$v.list.report{/yun}','js'=>'随时到岗'),
                    '{工作经历}'            =>  array('php'=>'{yun:}$v.list.resume_workjj{/yun}','js'=>'2021-04-至今参加过1份工作，平均工作时长2个月，涉及技术开发岗位。'),
                    '{教育经历}'            =>  array('php'=>'{yun:}$v.list.resume_edujj{/yun}','js'=>'2016-09-2020-06已完成本科段学业。'),
                    '{期望薪资}'            =>  array('php'=>'{yun:}$v.list.salary{/yun}','js'=>'1000-18000'),
                    '{期望地点开始}'      =>  array('php'=>'{yun:}foreach from=$v.list.expectcity item=v1 key=key{/yun}','js'=>'{forstart_1}'),
                    '{期望地点}'            =>  array('php'=>'{yun:}$v1{/yun}','js'=>'上海'),
                    '{期望地点结束}'      =>  array('php'=>'{yun:}/foreach{/yun}','js'=>'{forend_1}'),

                    '{工作职能开始}'      =>  array('php'=>'{yun:}foreach from=$v.list.expectjob item=v2 key=key{/yun}','js'=>'{forstart_2}'),
                    '{工作职能}'            =>  array('php'=>'{yun:}$v2{/yun}','js'=>'公务员'),
                    '{工作职能结束}'      =>  array('php'=>'{yun:}/foreach{/yun}','js'=>'{forend_2}'),
                    '{头像}'              =>  array(
                        'php'=>'<img src="{yun:}$v.list.photo{/yun}" style=style_v/>',
                        'js'=>checkpic($this->config['sy_member_icon'])
                    ),
                );
        $this->pubtoolself_resumecolumn = array(
                    'resumecolumn_username'     =>  array('用户姓名','{姓名}','resume_column'),
                    'resumecolumn_photo'        =>  array('用户头像','{img|头像|样式=&quot;width:100px;height:100px;&quot;}','resume_column'),
                    'resumecolumn_age'          =>  array('年龄','{年龄}','resume_column'),
                    'resumecolumn_exp'          =>  array('经验','{经验}','resume_column'),
                    'resumecolumn_edu'          =>  array('学历','{学历}','resume_column'),
                    'resumecolumn_name'         =>  array('期望职位','{期望职位}','resume_column'),
                    'resumecolumn_wapurl'       =>  array('简历网址','{简历网址}','resume_column'),
                    'resumecolumn_jobstatus'    =>  array('求职状态','{求职状态}','resume_column'),
                    'resumecolumn_report'       =>  array('到岗时间','{到岗时间}','resume_column'),
                    'resumecolumn_salary'       =>  array('期望薪资','{期望薪资}','resume_column'),
                    'resumecolumn_city'         =>  array('期望地点','{期望地点开始}{期望地点}{期望地点结束}','resume_column'),
                    'resumecolumn_job'          =>  array('工作职能','{工作职能开始}{工作职能}{工作职能结束}','resume_column'),
                    'resumecolumn_workjj'       =>  array('工作经历','{工作经历}','resume_column'),
                    'resumecolumn_edujj'        =>  array('教育经历','{教育经历}','resume_column'),
                );
        //企业参数
        
        $this->pubtoolself_companycolumn = array(
                    'companycolumn_name'        =>  array('企业名称','{企业名称}','company_column'),
                    'companycolumn_welfare'     =>  array('企业福利','{企业福利开始}{企业福利}{企业福利结束}','company_column'),
                    'companycolumn_desc'        =>  array('企业描述|字数','{str|企业描述|length:200}','company_column'),
                    'companycolumn_comwapurl'   =>  array('企业网址','{企业网址}','company_column'),
                    'companycolumn_linkman'     =>  array('企业联系人','{企业联系人}','company_column'),
                    'companycolumn_linktel'     =>  array('企业联系电话','{企业联系电话}','company_column'),
                    'companycolumn_linkaddress' =>  array('企业联系地址','{企业联系地址}','company_column'),

                    'companycolumn_job'         =>  array('企业职位','{职位列表开始}{职位列表结束}','company_column'),
                    'companycolumn_jobname'     =>  array('职位名称','{职位名称}','company_column'),
                    'companycolumn_jobwapurl'   =>  array('职位网址','{职位网址}','company_column'),
                    'companycolumn_jobsalary'   =>  array('职位薪资','{职位薪资}','company_column'),
                    'companycolumn_jobexp'      =>  array('职位经验要求','{职位经验要求}','company_column'),
                    'companycolumn_jobedu'      =>  array('职位学历要求','{职位学历要求}','company_column'),
                    'companycolumn_jobdesc'     =>  array('职位描述|字数','{str|职位描述|length:200}','company_column'),
                    'companycolumn_jobxcxurl'   =>  array('职位小程序外链','{职位小程序外链}','company_column'),
                    
                );
        $this->pubtoolself_companycolumn_map = array(
                    '{企业名称}'            =>  array('php'=>'{yun:}$v.name{/yun}','js'=>'xx企业'),
                    '{企业福利开始}'        =>  array('php'=>'{yun:}foreach from=$v.welfare_arr item = cwel{/yun}','js'=>'{forstart_1}'),
                    '{企业福利}'            =>  array('php'=>'{yun:}$cwel{/yun}','js'=>'企业福利'),
                    '{企业福利结束}'        =>  array('php'=>'{yun:}/foreach{/yun}','js'=>'{forend_1}'),

                    '{企业描述}'            =>  array('php'=>'{yun:}$v.desc{/yun}','js'=>'企业简介就是对一个公司进行一个简单全面的介绍，这种介绍不是一句话带过，也不是长篇大论...'),
                    '{企业网址}'            =>  array('php'=>'{yun:}url m=wap c=company a=show id=$v.uid{/yun}','js'=>Url('wap')),

                    '{职位列表开始}'      =>  array('php'=>'{yun:}foreach item=v1 key=k1 from=$v.row{/yun}','js'=>''),
                    '{职位名称}'            =>  array('php'=>'{yun:}$v1.name{/yun}','js'=>'xx职位'),
                    '{职位网址}'            =>  array('php'=>'{yun:}url m=wap c=job a=comapply id=$v1.id{/yun}','js'=>Url('wap')),
                    '{职位薪资}'            =>  array('php'=>'{yun:}$v1.job_salary{/yun}','js'=>'15000-25000(元/月)'),
                    '{职位经验要求}'      =>  array('php'=>'{yun:}$v1.job_exp{/yun}','js'=>'1-3年经验'),
                    '{职位学历要求}'      =>  array('php'=>'{yun:}$v1.job_edu{/yun}','js'=>'硕士学历'),
                    '{职位描述}'            =>  array('php'=>'{yun:}$v1.description{/yun}','js'=>'1.负责数据中心机房整体规划、部署，架构优化、容灾优化、性能优化；2.负责数据中心基础设施...'),
                    '{职位小程序外链}'       =>      array(
                        'php'=>'{yun:}xcxurl  type=job id=$v1.id{/yun}',
                        'js'=>'https://wxaurl.cn/job_xxx'
                    ),
                    '{职位列表结束}'      =>  array('php'=>'{yun:}/foreach{/yun}','js'=>''),

                    '{企业联系人}'       =>  array('php'=>'{yun:}$v.linkman{/yun}','js'=>'小王'),
                    '{企业联系电话}'      =>  array('php'=>'{yun:}$v.linktel{/yun}','js'=>'18888888888'),
                    '{企业联系地址}'      =>  array('php'=>'{yun:}$v.address{/yun}','js'=>'人民路xx号'),
                );
        //模板类型公共参数
        $this->pubtoolself_publiccolumn_map = array(
                    '{移动端二维码}'=>array(
                        'php'=>'<img src="{yun:}pubqrcode  toc=toc_v toa=toa_v toid=toid_v totype=wap{/yun}" style=style_v/>',
                        'js'=>Url('ajax',array("c"=>"wappubqrcode")),
                    ),
                    '{小程序场景码}'=>array(
                        'php'=>'<img src="{yun:}pubqrcode  toc=toc_v toa=toa_v toid=toid_v totype=weixinxcx{/yun}" style=style_v/>',
                        'js'=>Url('ajax',array("c"=>"wappubqrcode"))
                    ),
                    '{公众号场景码}'=>array(
                        'php'=>'<img src="{yun:}pubqrcode  toc=toc_v toa=toa_v toid=toid_v totype=weixin{/yun}" style=style_v/>',
                        'js'=>Url('ajax',array("c"=>"wappubqrcode"))
                    ),
                    '{小程序H5跳转}'=>array(
                        'php'=>'minipath',
                        'js'=>''
                    ),
                    '{小程序APPID}'=>array(
                        'php'=>'{yun:}$config.sy_xcxappid{/yun}',
                        'js'=>''
                    ),
                    '{小程序外链}'=>array(
                        'php'=>'{yun:}xcxurl  type=xcxurltype_v id=xcxurlid_v{/yun}',
                        'js'=>'https://wxaurl.cn/xxx'
                    ),
                );
        $this->pubtoolself_publiccolumn = array(
                    'wapewm'        =>  array('移动端二维码','{img|移动端二维码|样式=&quot;width:100px;height:100px;&quot;}','public_column'),
                    'weixinewm'     =>  array('公众号场景码','{img|公众号场景码|样式=&quot;width:100px;height:100px;&quot;}','public_column'),
                    'weixinxcxewm'  =>  array('小程序场景码','{img|小程序场景码|样式=&quot;width:100px;height:100px;&quot;}','public_column'),
                    'xcxewm'        =>  array('小程序H5跳转','H5xcx_{小程序APPID}_{小程序H5跳转}','public_column'),
                    'xcxurl'        =>  array('小程序外链','{小程序外链}','public_column'),
                );
        //整体公共参数
        $this->pubtoolself_totalcolumn_map = array(
                    
                    '{网站名称}'=>array('php'=>$this->config['sy_webname'],'js'=>$this->config['sy_webname']),
                    '{网站地址}'=>array('php'=>$this->config['sy_weburl'],'js'=>$this->config['sy_weburl']),
                    '{当前日期}'=>array('php'=>date('Y-m-d',time()),'js'=>date('Y-m-d',time())),
                    '{admin_style}'=>array('php'=>$this->config['sy_weburl']."/app/template/admin",'js'=>$this->config['sy_weburl']."/app/template/admin")
                );
        $this->pubtoolself_totalcolumn = array(
                    'webname'       =>  array('网站名称','{网站名称}','total_column'),
                    'weburl'        =>  array('网站地址','{网站地址}','total_column'),
                    'datetime'      =>  array('当前日期','{当前日期}','total_column'),
                );
    }
    
    public function columnForm($data=array()){

        $column = !empty($data['column'])?$data['column']:array();

        $newcolumn = array();

        foreach ($column as $key => $value) {
            $newcolumn[] = array(
                'key'=>$key,
                'data'=>$value
            );
        }

        return $newcolumn;

    }
    public function getTempList($whereData = array(),$data=array()){

        $list = array();

        if(!empty($whereData)){

            $data['field']  =   empty($data['field']) ? '*' : $data['field'];

            $list = $this -> select_all('wxpub_temps',$whereData, $data['field']);

        }

        return $list;
    }
    public function getTemp($whereData = array(),$data=array()){
        
        if(!empty($whereData)){
            
            $data['field']  =   empty($data['field']) ? '*' : $data['field'];

            $whb            =   $this -> select_once('wxpub_temps',$whereData,$data['field']);
            
            return $whb;
        }
    }
    public function updateTemp($updata = array(),$whereData = array()){

        if(!empty($whereData)){
            
            $return  =     $this -> update_once('wxpub_temps',$updata, $whereData);
            
        }
    }
    public function delTemp($delId){

        if(!empty($delId)){
            
            $return['layertype']    =   0;
            
            if(is_array($delId)){
                
                $delId  =   pylode(',',$delId);
                
                $return['layertype']    =   1;
            }
        }
        
        $where['id']        =   array('in',$delId);
        $where['type']      =   array('<>','onejob');

        $return['id']       =   $this -> delete_all('wxpub_temps',$where,'');
        
        $return['msg']      =   '模板(ID:'.$delId.')';
        
        $return['errcode']  =   $return['id'] ? '9' :'8';
        
        $return['msg']      =   $return['id'] ? $return['msg'].'删除成功！' :$return['msg'].'删除失败！';
        
        return  $return;
    }
    public function setTemp($updata = array(),$whereData = array()){

        if(!empty($whereData)){
            
            $return['id']   =     $this -> update_once('wxpub_temps',$updata, $whereData);
            
            $return['msg']  =   '模板更新';
        }else{
            
            $return['id']   =     $this -> insert_into('wxpub_temps',$updata);
            
            $return['msg']  =   '模板添加';
        }

        $return['errcode']  =   $return['id'] ? '9' :'8';
        
        $return['msg']      =   $return['id'] ? $return['msg'].'成功！' :$return['msg'].'失败！';
        
        return  $return;
    }
    
    function getOneJob($jobid,$provider=''){

        $html =     '';
        
        if($jobid){

            require_once ('job.model.php');
        
            $jobM    =   new job_model($this->db, $this->def);

            $job    =   $jobM->getInfo(array('id' => $jobid));

            if(!empty($job)){
                
                $job['description'] = str_replace(array('&quot;', '&nbsp;', '<>'), array('', '', ''),strip_tags($job['description']));
                
                $cominfo  =   $this->select_once('company',array('uid'=>$job['uid']));
                
                $job['com_desc'] = str_replace(array('&quot;', '&nbsp;', '<>'), array('', '', ''),strip_tags($cominfo['description']));
				
				if($job['link_id']>0){
					$linkinfo		=	$this->select_once('company_job_link',array('id'=>$job['link_id']));
					$job['phone']	=	!empty($linkinfo['link_moblie'])?$linkinfo['link_moblie']:$linkinfo['link_phone'];
					$job['address']	=	$linkinfo['link_address'];
				}else{
					$job['phone']	=	!empty($cominfo['linktel'])?$cominfo['linktel']:$cominfo['linkphone'];
					$job['address']	=	$cominfo['address'];
				}
                

                $temp = $this->getTemp(array('type'=>'onejob'));
 
                $onejobcolumn_map = $this->onejobcolumn_map;
                $pubtoolself_totalcolumn_map = $this->pubtoolself_totalcolumn_map;
                
                $search = array();
                $replace = array();
                
                //是否含有约束字符串的相关标签{str|xxx|length}
                $str_result     =   array();
                preg_match_all('#\{str\|(.*?)\}#i', $temp['body'], $str_result);

                foreach ($str_result[1] as $sr_k => $sr_v) {

                    $str_arr    =   explode("|", $sr_v);

                    if (count($str_arr) > 1) {

                        $tempmap_key    =   '{' . $str_arr[0] . '}';

                        $jk = $onejobcolumn_map[$tempmap_key]['php'];
                        
                        $fun_arr    =   explode(":",$str_arr[1]);
                        //约束字符串长度
                        if($fun_arr[0]=='length'){

                            $search[]    =   '{str|'.$sr_v.'}';
                            
                            if(intval($fun_arr[1])>0){

                                if(mb_strlen($job[$jk])>intval($fun_arr[1])){

                                    $replace[]  =   mb_substr($job[$jk],0,intval($fun_arr[1])).'...';

                                }else{

                                    $replace[]  =   mb_substr($job[$jk],0,intval($fun_arr[1]));
                                    
                                }
                            }else{
                                $replace[]  =   $job[$jk];
                            }
                        }
                    }
                }
                
                //是否含有约束字符串的相关标签end

                foreach ($onejobcolumn_map as $key => $value) {

                    if(is_array($value['php'])){

                        if($value['php']['type']=='url'){
                            $search[]   = $key;
                            if($value['php']['urltype']=='job'){
                                $replace[]  = Url('wap',array("c"=>"job",'a'=>'comapply',"id"=>$job['id']));
                            }else if($value['php']['urltype']=='company'){
                                $replace[]  = Url('wap',array("c"=>"company",'a'=>'show',"id"=>$job['uid']));
                            }
                        }

                    }else{
                        $search[]   = $key;
                        $replace[]  = $job[$value['php']];
                    }

                }

                foreach ($pubtoolself_totalcolumn_map as $tk => $tv) {
                    $search[]   = $tk;
                    $replace[]  = $tv['php'];
                }

                
                //福利待遇循环
                $result = array();

                $preg = "#\{福利开始}(.*?)\{福利结束}#i";

                preg_match_all($preg,$temp['body'], $result);

                
                foreach ($result[0] as $rk => $rv) {
                    $whtml = '';
                    $wv    =   str_replace(array('{福利开始}','{福利结束}'),array('',''), $rv);
                    
                    foreach ($job['arraywelfare'] as $k => $v) {
                        $whtml .= str_replace('{职位福利}',$v, $wv);
                    }
                    
                    $temp['body'] = str_replace($rv,$whtml,$temp['body']);
                }



                $search[] = '&amp;';
                $replace[] = '&';

                if($temp['header']){
                    $html .=    $temp['header'];
                    $html .=    "\r\n";

                }

                $html .=    $temp['body'];

                if($temp['footer']){
                    $html .=    "\r\n";
                    $html .=    $temp['footer'];
                }
                
                
                if ($provider == 'baidu' || $provider == 'toutiao'){
                    
                    // 将微信汉字替换，防止审核时被判断为诱导分享
                    $html  =  str_ireplace(array('微信','同号','{小程序外链}','小程序外链'), '', $html);
                    
                }else{
                    //小程序外链
                    require_once ('xcx.model.php');
                    $xcxM    =   new xcx_model($this->db, $this->def);

                    $xcxdata = array(
                        'type'=>'job',
                        'id'=>$job['id']
                    );

                    $xcxurl = $xcxM->getUrlLink($xcxdata);

                    $search[] = '{小程序外链}';
                    $replace[] = $xcxurl;
                }

                
                
                $html = str_replace($search, $replace,$html);

                if ($provider == 'admin' || $provider == 'wap'){//后台职位复制文本 电脑端微信换行
                    $html = str_replace("\n","</br>",$html);
                }

            }
        }
        return $html;
    }

    /**
     * @param array $post
     * @param array $data
     * @return array
     */
    public function addTwTask($post = array(), $data = array())
    {

        if (empty($post['type']) || (empty($post['jobid']) && empty($post['cuid'])) || empty($data['auid'])) {

            return array('code' => 8, 'msg' => '参数错误，请重试');
        } else {

            if ($post['type'] == 1) {

                $jobIdArr   =   @explode(',', $post['jobid']);

                $jobList    =   $this->select_all('company_job', array('id' => array('in', pylode(',', $jobIdArr))), '`id`,`uid`,`name`,`com_name`,`sdate`,`state`');

                $valueData  =   array();
                $time       =   time();
                foreach ($jobList as $k => $v) {

                    $valueData[$k]['jobid']     =   $v['id'];
                    $valueData[$k]['cuid']      =   $v['uid'];
                    $valueData[$k]['jobname']   =   $v['name'];
                    $valueData[$k]['comname']   =   $v['com_name'];
                    $valueData[$k]['jobsdate']  =   $v['sdate'];
                    $valueData[$k]['auid']      =   $data['auid'];
                    $valueData[$k]['content']   =   $post['content'];
                    $valueData[$k]['urgent']    =   $post['urgent'];
                    $valueData[$k]['wcmoments'] =   $post['wcmoments'];
                    $valueData[$k]['gzh']       =   $post['gzh'];
                    $valueData[$k]['ctime']     =   $time;
                    $valueData[$k]['status']    =   0;
                    $valueData[$k]['type']      =   1;
                }

                $valueData  =   array_values($valueData);

                $result     =   $this->DB_insert_multi('wxpub_twtask', $valueData);

                $return['code'] =   $result ? 9 : 8;
                $return['msg']  =   $result ? '推文任务添加成功' : '推文任务添加失败';
                return $return;

            } elseif ($post['type'] == 2) {

                $uidArr     =   @explode(',', $post['cuid']);

                $comList    =   $this->select_all('company', array('uid' => array('in', pylode(',', $uidArr))), '`uid`,`name`,`lastupdate`');

                $valueData  =   array();
                $time       =   time();
                foreach ($comList as $k => $v) {

                    $valueData[$k]['cuid']      =   $v['uid'];
                    $valueData[$k]['comname']   =   $v['name'];
                    $valueData[$k]['jobsdate']  =   $v['lastupdate'];
                    $valueData[$k]['auid']      =   $data['auid'];
                    $valueData[$k]['content']   =   $post['content'];
                    $valueData[$k]['urgent']    =   $post['urgent'];
                    $valueData[$k]['wcmoments'] =   $post['wcmoments'];
                    $valueData[$k]['gzh']       =   $post['gzh'];
                    $valueData[$k]['ctime']     =   $time;
                    $valueData[$k]['status']    =   0;
                    $valueData[$k]['type']      =   2;
                }

                $valueData  =   array_values($valueData);

                $result     =   $this->DB_insert_multi('wxpub_twtask', $valueData);

                $return['code'] =   $result ? 9 : 8;
                $return['msg']  =   $result ? '推文任务添加成功' : '推文任务添加失败';
                return $return;
            }
        }
    }

    function upTwtask($where = array(),$upData = array()){

        $nid    =   $this -> update_once('wxpub_twtask', $upData, $where);

        return $nid;
    }
    
    public function getTwTaskList($whereData = array(),$data=array()){

        $list = array();

        if(!empty($whereData)){

            $data['field']  =   empty($data['field']) ? '*' : $data['field'];

            $list = $this -> select_all('wxpub_twtask',$whereData, $data['field']);

            if(!empty($list)){

                $jobid_arr = array();
                $jobdata = array();

                $auid_arr = array();
                $alist = array();

                
                foreach ($list as $key => $value) {
                    if($value['jobid']){
                        $jobid_arr[] = $value['jobid'];
                    }
                    $auid_arr[] = $value['auid'];
                }

                if(!empty($jobid_arr)){

                    $joblist = $this -> select_all('company_job',array('id'=>array('in',pylode(',',$jobid_arr))),'`id`,`status`');
                    foreach ($joblist as $jkey => $jvalue) {
                        $jobdata[$jvalue['id']] = $jvalue;
                    }

                }

                if(!empty($auid_arr)){

                    $alist = $this -> select_all('admin_user',array('uid'=>array('in',pylode(',',$auid_arr))),'`uid`,`username`,`name`');

                }

                foreach ($list as $k => $v) {

                    $list[$k]['jobsdate_n'] = date('Y-m-d H:i',$v['jobsdate']);
                    
                    
                    $list[$k]['comurl']      =  Url('company', array('c' => 'show', 'id' => $v['cuid']));

                    if(!empty($jobdata[$v['jobid']])){
                        if($jobdata[$v['jobid']]['status']=='1'){
                            $list[$k]['jobstatus'] = 2;
                        }
                        $list[$k]['joburl']  =  Url('job', array('c' => 'comapply', 'look' => 'admin', 'id' => $v['jobid']));
                    }else{
                        $list[$k]['jobstatus'] = 1;
                    }

                    foreach ($alist as $ak => $av) {
                        if($av['uid']==$v['auid']){
                            $list[$k]['admin_username'] = $av['name'] ? $av['name'] : $av['username'];// 没有真实姓名才使用用户名
                        }
                    }

                    $list[$k]['ctime_n'] = date('Y-m-d H:i',$v['ctime']);
                }
            }
        }

        return $list;
    }

    public function getComTwTaskList($whereData = array(), $data = array())
    {

        $list   =   array();

        if (!empty($whereData)) {

            $data['field']  =   empty($data['field']) ? '*' : $data['field'];

            $list           =   $this->select_all('wxpub_twtask', $whereData, $data['field']);

            if (!empty($list)) {

                $cuid_arr   =   array();
                $comdata    =   array();

                $auid_arr   =   array();
                $alist      =   array();

                foreach ($list as $key => $value) {
                    if ($value['cuid']) {

                        $cuid_arr[] = $value['cuid'];
                        $auid_arr[] = $value['auid'];
                    }
                }

                if (!empty($cuid_arr)) {

                    $comlist = $this->select_all('company', array('uid' => array('in', pylode(',', $cuid_arr))), '`uid`,`r_status`');
                    foreach ($comlist as $ckey => $cvalue) {
                        $comdata[$cvalue['uid']] = $cvalue;
                    }
                }

                if (!empty($auid_arr)) {

                    $alist = $this->select_all('admin_user', array('uid' => array('in', pylode(',', $auid_arr))), '`uid`,`username`,`name`');
                }

                foreach ($list as $k => $v) {

                    $list[$k]['jobsdate_n'] =   date('Y-m-d H:i', $v['jobsdate']);
                    $list[$k]['comurl']     =   Url('company', array('c' => 'show', 'id' => $v['cuid']));

                    if (!empty($comdata[$v['cuid']])) {
                        if ($comdata[$v['cuid']]['r_status'] == '1') {
                            $list[$k]['comstatus']  =   1;
                        }
                    } else {
                        $list[$k]['comstatus']      =   2;
                    }

                    foreach ($alist as $ak => $av) {
                        if ($av['uid'] == $v['auid']) {
                            $list[$k]['admin_username'] = $av['name'] ? $av['name'] : $av['username'];// 没有真实姓名才使用用户名
                        }
                    }

                    $list[$k]['ctime_n'] = date('Y-m-d H:i', $v['ctime']);
                }
            }
        }

        return $list;
    }

    public function delTwtask($delId){

        if(!empty($delId)){
            
            $return['layertype']    =   0;
            
            if(is_array($delId)){
                
                $delId  =   pylode(',',$delId);
                
                $return['layertype']    =   1;
            }
        }
        
        $where['id']        =   array('in',$delId);
        
        $return['id']       =   $this -> delete_all('wxpub_twtask',$where,'');
        
        $return['msg']      =   '推文任务(ID:'.$delId.')';
        
        $return['errcode']  =   $return['id'] ? '9' :'8';
        
        $return['msg']      =   $return['id'] ? $return['msg'].'删除成功！' :$return['msg'].'删除失败！';
        
        return  $return;
    }
}

?>
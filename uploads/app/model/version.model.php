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
class version_model extends model{
   
    // 查询最新版本号
    public function getVerion(){
        
        $row = $this->select_once('version',array('orderby'=>'id'));
        
        $version = !empty($row) ? $row['version'] : 'v6.0';
        
        return $version;
    }
	
	// 查询最新版本code
    public function getLatestCode(){
        $row = $this->select_once('version',array('orderby'=>'id'));
        $code = isset($row['code']) && $row['code'] ? $row['code'] : '';
        return $code;
    }
	
    // 查询版本号记录
    public function getVersionList(){
        
        $list = $this->select_all('version', array('orderby'=>'id'));
        foreach ($list as $k=>$v){
            if ($v['ctime']){
                $list[$k]['ctime_n'] = date('Y-m-d H:i:s',$v['ctime']);
            }
        }
        return $list;
    }
    // phpyun版本比较
    function phpyunVersionCompare($newV){
        
        include (APP_PATH . 'version.php');
        // 更新文件运行版本
        $sqlv = $this->phpyunVersion($newV);
        // version.php里的版本
        $vv = $this->phpyunVersion($version);
        // version.php里的版本大于更新文件运行版本，说明有更新文件没运行
        $res = $this->versionCompare($vv, $sqlv);
        if ($res == 1){
            // 有升级文件没运行
            return 1;
        }else{
            return 0;
        }
    }
    
    // 版本号比较
    function versionCompare($v1,$v2){
        $length = strlen($this->reg($v1))>strlen($this->reg($v2)) ? strlen($this->reg($v1)): strlen($this->reg($v2));
        $v1 = $this->add($this->reg($v1),$length);
        $v2 = $this->add($this->reg($v2),$length);
        if($v1 == $v2) {
            return 0;
        }else{
            return $v1>$v2?1:-1;
        }
    }
    
    private function reg($str){
        return preg_replace('/[^0-9]/','',$str);
    }
    //根据length的长度进行补0的操作，$length的值为两个版本号中最长的那个
    private function add($str,$length){
        return str_pad($str,$length,"0");
    }
    // 对phpyun版本号进行处理，获取纯数字版本号
    private function phpyunVersion($version){
        
        $p = stripos($version,'(');
        $str = substr($version, 0, $p);
        
        return preg_replace('/[^0-9]/','',$str);
    }
}
?>
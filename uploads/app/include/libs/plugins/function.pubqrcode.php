<?php
function smarty_function_pubqrcode($paramer,&$smarty){
		global $config,$db_config,$db,$views;
		
		if($paramer['totype'] == 'wap'){//移动端二维码
			$wapUrl = Url('wap');
			if( isset($paramer['toid']) && $_GET['toid'] != ''){
				$wapUrl = Url('wap',array('c'=>$paramer['toc'],'a'=>$paramer['toa'],'id'=>(int)$paramer['toid']));
			}
			
			/**
			 *	新加;
			 *	目前pubqrcode只用在微信发布工具模板中;
			 *	这里新加参数 totype：wxpubtool ,获取二维码时，发布工具二维码独立返回，不涉及其他参数判断
			 */
			$qrcode = Url('ajax',array('c'=>'pubqrcode','toc'=>$paramer['toc'],'toa'=>$paramer['toa'],'toid'=>(int)$paramer['toid'],'totype' => 'wxpubtool'));
			//$qrcode = Url('ajax',array('c'=>'pubqrcode','toc'=>$paramer['toc'],'toa'=>$paramer['toa'],'toid'=>(int)$paramer['toid']));

		}elseif($paramer['totype'] == 'weixin'){//微信公众号带参数二维码
		
			
            
            include_once(APP_PATH.'app/model/weixin.model.php');
          
            $WxM  =  new weixin_model($db,$db_config['def']);
            
			$qrcode = $WxM->pubWxQrcode($paramer['toc'],$paramer['toid'],'weixin');
		    
		

		}
		
		return $qrcode;

	}
?>
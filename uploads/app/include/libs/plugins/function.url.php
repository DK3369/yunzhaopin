<?php
function smarty_function_url($paramer,$template){
	global $config,$seo;
    $index = isset($paramer['index']) ? $paramer['index'] : '';
    $module = isset($paramer['m']) ? $paramer['m'] : '';
	if(!$index && $module=='member'){
		$index='member';
		unset($paramer['m']);
	}
    
     unset($paramer['index']);
     $module = isset($paramer['m']) ? $paramer['m'] : '';
     $url  =  get_url($paramer,$config,$seo,$module,$index,$template);
	return $url;
}
?>
<?php

function smarty_outputfilter_i18n($source)
{
    if (!function_exists('yun_auto_t')) {
        return $source;
    }

    $store = array();
    $source = preg_replace_callback('#<(script|style|textarea|pre)[^>]*>.*?</\\1>#is', 'yun_i18n_store_block', $source);

    $source = preg_replace_callback('/>([^<>]*[\\x{4e00}-\\x{9fff}][^<>]*)</u', 'yun_i18n_translate_text_node', $source);
    $source = preg_replace_callback('/\\b(alt|title|placeholder|value|content)=("|\')([^"\']*[\\x{4e00}-\\x{9fff}][^"\']*)\\2/iu', 'yun_i18n_translate_attr', $source);

    if (!empty($GLOBALS['yun_i18n_filter_store'])) {
        foreach ($GLOBALS['yun_i18n_filter_store'] as $key => $value) {
            $source = str_replace($key, $value, $source);
        }
        $GLOBALS['yun_i18n_filter_store'] = array();
    }

    return $source;
}

function yun_i18n_store_block($matches)
{
    if (!isset($GLOBALS['yun_i18n_filter_store']) || !is_array($GLOBALS['yun_i18n_filter_store'])) {
        $GLOBALS['yun_i18n_filter_store'] = array();
    }
    $key = '@@YUN_I18N_BLOCK_' . count($GLOBALS['yun_i18n_filter_store']) . '@@';
    $GLOBALS['yun_i18n_filter_store'][$key] = $matches[0];
    return $key;
}

function yun_i18n_translate_text_node($matches)
{
    return '>' . yun_auto_t($matches[1]) . '<';
}

function yun_i18n_translate_attr($matches)
{
    return $matches[1] . '=' . $matches[2] . yun_auto_t($matches[3]) . $matches[2];
}
?>

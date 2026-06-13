<?php

function smarty_function_t($paramer, $template)
{
    $key = isset($paramer['key']) ? $paramer['key'] : '';
    $default = isset($paramer['default']) ? $paramer['default'] : '';
    $escape = isset($paramer['escape']) ? $paramer['escape'] : '';

    unset($paramer['key'], $paramer['default'], $paramer['escape']);

    if (function_exists('yun_t')) {
        $text = yun_t($key, $paramer, $default);
    } else {
        $text = $default !== '' ? $default : $key;
    }

    if ($escape == 'html') {
        return htmlspecialchars($text, ENT_QUOTES, 'UTF-8');
    }

    return $text;
}
?>

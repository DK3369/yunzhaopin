<?php

/**
 * DB-aligned enum literals for WAP / wxapp (single PHP hub).
 */
class WapDbEnum
{
    const SEX_MALE = '男';
    const SEX_FEMALE = '女';
    const UNLIMITED = '不限';
    const PRESENT = '至今';

    const CODE_WEB_REGISTER = '注册会员';
    const CODE_WEB_FRONT_LOGIN = '前台登录';
    const CODE_WEB_FORGET_PW = '找回密码';
    const CODE_WEB_FEEDBACK = '意见反馈';
    const CODE_WEB_ONCE_JOB = '店铺招聘';
    const CODE_WEB_TINY_RESUME = '普工简历';
    const CODE_WEB_ASK_QUESTION = '职场提问';

    const INTEGRAL_BIND_WX = '微信扫码绑定';
    const INTEGRAL_LOGIN = '会员登录';
    const CONFIG_PRIVACY = '隐私政策';
    const ADDR_PREFIX = '收货地址：';
}

// 获取EXP
function getExp() {
    return comFormat('job_exp');
}

// 获取学历
function getEdu() {
    return comFormat('job_edu');
}

// 获取到岗时间
function getReport() {
    return comFormat('job_report');
}

// 获取企业性质
function getPr() {
    return comFormat('job_pr');
}

// 获取企业规模
function getMun() {
    return comFormat('job_mun');
}

// 获取性别要求
function getSex() {
    return {
        'id': [3, 1, 2],
        'name': ['不限', '男', '女']
    };
}
function getSexReq() {
    return {
        'id': [3, 2],
        'name': ['不限', '女']
    };
}

// 获取婚姻状况
function getMarriage() {
    return comFormat('job_marriage');
}

//获取简历备注
function getremark() {
    return comFormat('job_remark');
}

// 获取语言要求
function getLang() {
    var data = [];
    if (typeof comd['job_lang'] !== 'undefined') {
        var arr = comd['job_lang'];
        for (var i = 0; i < arr.length; i++) {
            var val = arr[i];
            data.push({id: val, name: comn[val], checked: false})
        }
    }
    return data;
}

// 格式化数据
function comFormat(key) {
    var data = {
        name: [],
        id: []
    };
    if (typeof comd[key] !== 'undefined') {
        var arr = comd[key];
        for (var i = 0; i < arr.length; i++) {
            var val = arr[i];
            data.name.push(comn[val]);
            data.id.push(val);
        }
    }

    return data
}

// 获取行业
function getHy(defaultOptionName) {
    if (defaultOptionName) {
        var data = {
            name: [defaultOptionName],
            id: [0]
        };
    } else {
        var data = {
            name: [],
            id: []
        };
    }

    if (typeof hi !== 'undefined') {
        var arr = hi;
        for (var i = 0; i < arr.length; i++) {
            var val = arr[i];
            data.name.push(hyname[val]);
            data.id.push(val);
        }
    }

    return data
}

// 获取经验要求
function getExpReq() {
    let exp = userFormat('user_word');
    exp.name.unshift('请选择工作经验');
    exp.id.unshift('0');
    return exp;
}

// 获取学历要求
function getEduReq() {
    let edu = userFormat('user_edu');
    edu.name.unshift('请选择学历');
    edu.id.unshift('0');
    return edu;
}

// 格式化数据
function userFormat(key) {
    var data = {
        name: [],
        id: []
    };
    if (typeof useri[key] !== 'undefined') {
        var arr = useri[key];
        for (var i = 0; i < arr.length; i++) {
            var val = arr[i];
            data.name.push(usern[val]);
            data.id.push(val);
        }
    }

    return data
}

// 获取创办时间年
function getFoundedYear(){
    var date = new Date(),
        year = date.getFullYear(),
        yearArr = [];
    for (var i = year; i >= 1900; i--){
        yearArr.push({date: i, text: i + '年'});
    }
    return yearArr;
}

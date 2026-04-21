// 获取性别要求
function getSex() {
    return {
        'id': [0, 3, 1, 2],
        'name': ['请选择性别', '不限', '男', '女']
    };
}

// 兼职类型
function getType() {
    let data = partFormat('part_type');
    data.id.unshift(0);
    data.name.unshift('请选择兼职类型');
    return data;
}

// 结薪方式
function getBillingcycle() {
    let data = partFormat('part_billing_cycle');
    data.id.unshift(0);
    data.name.unshift('请选择结算周期');
    return data;
}

// 薪资类型
function getSalarytype() {
    let data = partFormat('part_salary_type');
    data.id.unshift(0);
    data.name.unshift('请选择薪资类型');
    return data;
}

// 格式化数据
function partFormat(key) {
    var data = {
        name: [],
        id: []
    };
    if (typeof partd[key] !== 'undefined') {
        var arr = partd[key];
        for (var i = 0; i < arr.length; i++) {
            var val = arr[i];
            data.name.push(partn[val]);
            data.id.push(val);
        }
    }

    return data
}

// 获取兼职时效
function getTimetype() {
    return {
        id: [1, 2],
        name: ['短期招聘', '长期招聘']
    };
}

// 兼职时间-早
function getMorning(data) {
    if (!data) {
        data = []
    }
    let morning = new Array('0101', '0201', '0301', '0401', '0501', '0601', '0701');
    let newMorning = new Array();
    for (var i = 0; i < morning.length; i++) {
        newMorning.push({name: morning[i], checked: data.indexOf(morning[i]) < 0 ? false : true})
    }
    return newMorning;
}

// 兼职时间-中
function getNoon(data) {
    if (!data) {
        data = []
    }
    let noon = new Array('0102', '0202', '0302', '0402', '0502', '0602', '0702');
    let newNoon = new Array();
    for (var i = 0; i < noon.length; i++) {
        newNoon.push({name: noon[i], checked: data.indexOf(noon[i]) < 0 ? false : true})
    }
    return newNoon;
}

// 兼职时间-晚
function getAfternoon(data) {
    if (!data) {
        data = []
    }
    let afternoon = new Array('0103', '0203', '0303', '0403', '0503', '0603', '0703');
    let newAfternoon = new Array();
    for (var i = 0; i < afternoon.length; i++) {
        newAfternoon.push({name: afternoon[i], checked: data.indexOf(afternoon[i]) < 0 ? false : true})
    }
    return newAfternoon;
}

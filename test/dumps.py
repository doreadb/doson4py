# Doson Parser Example - dumps
#
# 本代码用于测试程序将对象转换为字符集
# 代码函数：
# doson.dumps(object)

import doson4py as doson

def main():

    # 基本数据类型测试
    basetype = [
        doson.dumps("string"),
        doson.dumps(3.14),
        doson.dumps(True),
    ]
    assert (basetype[0] == '"string"' and basetype[1] == "3.14" and basetype[2] == "true")

    # 元组转换
    tuple = doson.dumps((1, "value"))
    assert tuple == '(1,"value")'

    # 字典转换
    dict = doson.dumps({
        "name": "ZhuoEr Liu",
    })
    assert dict == '{"name":"ZhuoEr Liu"}'

    print("代码测试成功：doson4py")

if __name__ == "__main__":
    main()
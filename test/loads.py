# Doson Parser Example - loads
#
# 本代码用于测试程序从字符串中加载到相应结构
# 代码函数：
# doson.loads(str)

import doson4py as doson

def main():

    # 内包含两个数字的元组集
    tuple = doson.loads('(1,2)')
    assert tuple == (1, 2)

    # 内含不同类型的数组列表
    list = doson.loads('[3.14, "string", true, (2, 3)]')
    assert list == [3.14, "string", True, (2, 3)]

    print("代码测试成功：doson4py")

if __name__ == "__main__":
    main()
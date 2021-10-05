import doson4py

class test(object):
    pass

if __name__ == "__main__":
    # print(doson4py.loads("(1,2)"))
    print(doson4py.dumps(
        {
            "hello": "world",
            "123": True,
            123: 444
        }
    ))
from rust_sort import bubble

def bubble_test():
    # homogeneous
    bubble([5,-2,3,0,11])
    """
    5
    -2
    3
    0
    11
    """
    # works for heterogeneous too
    bubble([5,-2,"a",0,11])


if __name__ == "__main__":
    bubble_test()
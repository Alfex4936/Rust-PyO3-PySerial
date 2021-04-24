from rust_sort import bubble, Human


def bubble_test():
    # homogeneous
    bubble([5, -2, 3, 0, 11])
    """
    5
    -2
    3
    0
    11
    """
    # works for heterogeneous too
    bubble([5, -2, "a", 0, 11])


def class_test():
    print()
    human = Human("A1", 37, 1200)
    print(human.salary)  # 1200
    human.promote(1500)
    print(human.salary)  # 1500
    print(human.string())  # I am A1!


if __name__ == "__main__":
    bubble_test()
    class_test()

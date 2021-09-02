from rust_sort import Human, bubble


def bubble_test():
    # homogeneous
    a = [1, 42024, -1, 22, 0, 333333]
    bubble(a)
    print(a)


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

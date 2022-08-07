from uwb_serial import UWB


def main():
    print(UWB.get_available_ports())
    uwb = UWB("/dev/ttyS50", log_file="log.txt")

    print(uwb.port_name, uwb.baudrate, uwb.log_file)

    uwb.connect(stdout=True, append=True)
    print("Came here")


if __name__ == "__main__":
    main()

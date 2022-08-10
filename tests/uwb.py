import argparse

from rich import print
from uwb_serial import UWB


def main(port="/dev/ttyACM0", log_file="log.txt", stdout=False, interactive_mode=False):
    print(UWB.get_available_ports())
    uwb = UWB(port=port, log_file=log_file)

    print(uwb.port_name, uwb.baudrate, uwb.log_file)

    uwb.connect(
        stdout=stdout, append=False, interactive=interactive_mode
    )  # infinite loop
    print("Came here")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="option")
    parser.add_argument(
        "-p",
        "--port",
        required=False,
        default="/dev/ttyACM0",
        type=str,
        help="tty port",
    )
    parser.add_argument(
        "-l",
        "--log_file",
        required=False,
        default="log.txt",
        type=str,
        help="output to logfile",
    )

    parser.add_argument(
        "--stdout",
        required=False,
        default=False,
        action=argparse.BooleanOptionalAction,
    )

    parser.add_argument(
        "--interactive",
        required=False,
        default=False,
        action=argparse.BooleanOptionalAction,
    )

    args = parser.parse_args()
    # print(args.port)

    main(
        port=args.port,
        log_file=args.log_file,
        stdout=args.stdout,
        interactive_mode=args.interactive,
    )

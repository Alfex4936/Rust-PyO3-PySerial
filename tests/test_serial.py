import sys
import time

# sudo socat PTY,link=/dev/ttyS50 PTY,link=/dev/ttyS51
# make S50 listen from S51
with open("/dev/pts/6", "wb+", buffering=0) as term:
    while True:
        term.write("hello".encode())
        time.sleep(1)

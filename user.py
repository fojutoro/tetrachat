def main(address: str, port: int):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_address = (address, port)
    sock.connect(server_address)
    try:
        message = input("> ")
        sock.sendall(str.encode(message))
        data = None
        while not data:
            data = sock.recv(32)
            print(f"{data}")
    except KeyboardInterrupt:
        sock.close()


if __name__ == "__main__":
    import socket
    import sys
    from config import port, address
    main(address, port)
    input()

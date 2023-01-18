def main(address: str, port: int):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_address = (address, port)
    while True:
        try:
            sock.connect(server_address)
        except Exception as e:
            print("Trying to connect to the server...")
            time.sleep(2)
        else:
            print("Connected to the server: " + server_address[0])
            connected = True
            break
    while connected:
        message = input("Message (q to quit) > ")
        if message.lower() == "q":
            break
        sock.sendall(str.encode(message))


if __name__ == "__main__":
    import socket
    import sys
    import time
    from config import port, address
    main(address, port)

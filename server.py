def process_data(data, client_address):
    print(f"{client_address[0]} > {data.decode('utf-8')}")


def main(address: str, port: int):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.bind((host, port))
        sock.listen()
        connection, client_address = sock.accept()
        with connection:
            print(f"User {client_address[0]} connected to the server.")
            while True:
                data = connection.recv(1024)
                if not data:
                    print(
                        f"User {client_address[0]} has disconnected from the server.")
                    break
                process_data(data, client_address)


if __name__ == "__main__":
    import socket
    import sys
    from config import host, port
    main(host, port)

def process_data(data, client_address):
    print(f"{client_address[0]} > {data.decode('utf-8')}")


def main(address: str, port: int):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        # THIS SHOULD BE `sock.create_server()` <- im scared to touch it without testing
        sock.bind((host, port))
        sock.listen()
        connection, client_address = sock.accept()
        with connection:
            print(f"User {client_address[0]} connected to the server.")
            while True:
                data = connection.recv(1024)
                if data[0:3] == "ACR":
                    uuid = choices('abcdefghijklmnopqrstuvwxyz', k=4)
                    account_creation(uuid, data, client_address[0])
                    sock.sendall("ACS" + uuid)
                if not data:
                    print(
                        f"User {client_address[0]} has disconnected from the server.")
                    break
                process_data(data, client_address)


if __name__ == "__main__":
    import socket
    import sys
    from db.db_managment import account_creation
    from old.config import host, port
    from random import choices
    main(host, port)

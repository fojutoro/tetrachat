def process_data(data, client_address):
    print(client_address[0] + " > " + data.decode("utf-8"))


def main(address: str, port: int):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_address = (address, port)
    sock.bind(server_address)
    sock.listen(1)
    while True:
        connection, client_address = sock.accept()
        print("User " + client_address[0] + " connected to the server.")
        while True:
            # buffer size - vobec neviem adekvatnu velkost bude sa treba pohrat
            data = connection.recv(256)
            if data:
                process_data(data, client_address)
            else:
                print("User " + client_address[0] +
                      " has disconnected from the server.")
                break


if __name__ == "__main__":
    import socket
    import sys
    from config import port, address
    main(address, port)

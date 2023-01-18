def main(address: str, port: int):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_address = (address, port)
    sock.bind(server_address)
    sock.listen()
    try:
        while True:
            connection, client_address = sock.accept()
            while True:
                data = connection.recv(32) # buffer size - vobec neviem adekvatnu velkost bude sa treba pohrat
                if data:
                    print(f"sender: {client_address}> {data}")
                    print()
                break
    except KeyboardInterrupt:
        sock.close()
        sys.exit(0)

if __name__ == "__main__":
    import socket
    import sys
    from config import port, address
    main(address, port)

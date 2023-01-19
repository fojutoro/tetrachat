def main(address: str, port: int):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        while True:
            try:
                sock.connect((host, port))
            except Exception:
                print("Trying to connect to the server...")
                time.sleep(2)
            else:
                print(f"Connected to the server: {host}")
                break
        while True:
            message = input("Message (q to quit) > ")
            if message.lower() == "q":
                break
            sock.sendall(str.encode(message))


if __name__ == "__main__":
    import socket
    import sys
    import time
    from config import host, port
    main(host, port)

import socket


def client_program():
    port = 8089  # socket server port number

    client_socket = socket.socket()  # instantiate
    client_socket.connect(("127.0.0.1", port))  # connect to the server

    data = ""

    while True:
        message = input(" -> ")  # take input
        message = message + "\r\n"
        client_socket.sendall(message.encode())
        data = client_socket.recv(1024).decode()  # receive response
        if data.lower().split() == "!bye":
            break
        print("Received from server: " + data)  # show in terminal

    client_socket.close()  # close the connection


if __name__ == "__main__":
    client_program()

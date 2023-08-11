import socket


def client_program():
    port = 8089  # socket server port number

    client_socket = socket.socket()  # instantiate
    client_socket.connect(("127.0.0.1", port))  # connect to the server

    data = ""

    while data := client_socket.recv(1024).decode():
        if data.lower().split() == "!bye":
            break
        print("Others: " + data)  # show in terminal
        message = input(": ")  # take input
        message = message + "\r\n"
        client_socket.sendall(message.encode())

    client_socket.close()  # close the connection


if __name__ == "__main__":
    client_program()

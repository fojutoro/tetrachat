from sys import quit
from random import choices
def menu():
    while True:
        choice = input("1. Create new account\n 2. Login\n  3. Quit")
        if choice == '1':
            create_acc()
        elif choice == '2':
            pass
        elif choice =='3':
            quit(0)

# WHOLE CODE IS A BUG. ONE BIG BUG https://www.thoughtco.com/thmb/BftbgSVcK-i94Fb5RHLYwZ_pJpY=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/GettyImages-535090295-584b3f963df78c491e7378bd.jpg

def create_acc(sock):
    name = input("Input name: ")
    if not name[0].isalpha():
        n = 1
        name = f"user{n}".strip()
        
    password = choices('abcdefghijklmnopqrstuvwxyz123456789', k=16)
    hashed_password = password
    # ACR for account creation request
    sock.sendall(str.encode("ACR" + hashed_password + name))
    pinging = 5
    while pinging:
        # I HAVE NO CLUE HOW TO RECIEVE A MESSAGE WHEN ITS CONNECTED AND NOT BINDED 
        # this should work not sure i'll test it out tmrw
        data = sock.recv(1024)
        # ACS for account creation succesfull <- i'm so creative
        if data[0:3] == 'ACS':
            print("Succesfully created account!")
            print("_" * 32 + "\n" + " " * 8 + f"{password}\n" + "-" * 32)
            print("DO NOT SHARE THIS WITH ANYONE THIS IS YOUR ONLY INFORMATION TO YOUR ACCOUNT DO NOT SHARE DO NOT SHARE DO NOT SHARE")
            print(f"name: {name}")
            print(f"Your friend code: {data[3:]}")
            print("press any button to continue")
            input()
            break
        else:
            print(f"There was an error creating the account.\n{data}")
            print("press any button to continue")
            input()
            
        pinging -= 1
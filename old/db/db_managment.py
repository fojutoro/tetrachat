import mysql.connector as dbcon
from db_config import host, user, password, db_name, table
def account_creation(uuid, data, ip):
    sensitive = data[3:19]
    name = data[19:]
    
    con = dbcon.connect(
        host=host,
        user=user,
        password=password,
        database=db_name
    )
    
    cursor = con.cursor()
    cmd= f"INSERT INTO {table} (uuid, privatekey, username, ip) VALUES (%s, %s, %s, %s)"
    val = (uuid, sensitive, name, ip)
    cursor.execute(cmd, val)
    con.commit()
    print("Created new user!")
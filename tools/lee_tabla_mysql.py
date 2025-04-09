import os
import sys
import argparse
import mysql.connector
from mysql.connector import Error
from dotenv import load_dotenv
from tabulate import tabulate

def get_db_connection(user, password, host='localhost', database='telemedicina'):
    """Establece conexión con la base de datos"""
    try:
        return mysql.connector.connect(
            host=host,
            database=database,
            user=user,
            password=password,
            connect_timeout=5
        )
    except Error as e:
        raise ConnectionError(f"No se pudo conectar a MySQL: {e}")

def get_table_metadata(table_name, connection):
    """Obtiene metadatos de las columnas de la tabla"""
    try:
        with connection.cursor() as cursor:
            cursor.execute(f"SHOW COLUMNS FROM {table_name}")
            return cursor.fetchall()
    except Error as e:
        raise ValueError(f"Error al obtener metadatos: {e}")

def display_columns(columns, table_name):
    """Muestra las columnas en formato de tabla"""
    if not columns:
        print(f"⚠️ La tabla '{table_name}' no existe o está vacía")
        return
    
    headers = ["Campo", "Tipo", "Nulo", "Clave", "Por Defecto", "Extra"]
    rows = []
    
    for col in columns:
        rows.append([col[0], col[1], col[2], col[3], col[4] or 'NULL', col[5] or ''])
    
    print(f"\n📋 Estructura de la tabla '{table_name}':")
    print(tabulate(rows, headers=headers, tablefmt="grid"))

def main():
    # Configuración de argumentos
    parser = argparse.ArgumentParser(description='Consultor de estructura de tablas MySQL')
    parser.add_argument('tabla', help='Nombre de la tabla a consultar')
    parser.add_argument('--host', default='localhost', help='Host de la base de datos')
    args = parser.parse_args()

    # Cargar variables de entorno
    load_dotenv()
    
    user = os.getenv('MYSQL_DEV_USER')
    password = os.getenv('MYSQL_DEV_PASS')
    
    if not all([user, password]):
        print("❌ Error: Configura las variables MYSQL_DEV_USER y MYSQL_DEV_PASS")
        print("   Puedes crearlas en un archivo .env o en tu entorno")
        return

    try:
        # Establecer conexión
        conn = get_db_connection(user, password, args.host)
        print(f"✅ Conectado a la base de datos '{conn.database}'")
        
        # Obtener y mostrar metadatos
        columns = get_table_metadata(args.tabla, conn)
        display_columns(columns, args.tabla)
        
    except Exception as e:
        print(f"❌ Error: {e}")
    finally:
        if 'conn' in locals() and conn.is_connected():
            conn.close()
            print("🔌 Conexión cerrada")

if __name__ == "__main__":
    main()

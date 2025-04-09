import os
import mysql.connector
from mysql.connector import Error
from typing import Dict, Any

def get_connection(config: Dict[str, Any] = None):
    """Establece conexión a MySQL"""
    config = config or {
        'host': os.getenv('MYSQL_HOST', 'localhost'),
        'user': os.getenv('MYSQL_DEV_USER'),
        'password': os.getenv('MYSQL_DEV_PASS'),
        'database': os.getenv('MYSQL_DB', 'telemedicina')
    }
    
    try:
        return mysql.connector.connect(**config)
    except Error as e:
        raise ConnectionError(f"Error conectando a MySQL: {str(e)}")

def get_columns(table_name: str, config: Dict[str, Any] = None) -> list:
    """Obtiene metadatos de columnas desde MySQL"""
    conn = get_connection(config)
    try:
        with conn.cursor() as cursor:
            cursor.execute(f"SHOW FULL COLUMNS FROM {table_name}")
            return cursor.fetchall()
    except Error as e:
        raise ValueError(f"Error obteniendo columnas: {str(e)}")
    finally:
        if conn.is_connected():
            conn.close()

def mysql_to_rust_type(mysql_type: str, is_nullable: str) -> str:
    """Convierte tipos MySQL a Rust"""
    mysql_type = mysql_type.lower()
    rust_type = ""
    
    if 'int' in mysql_type:
        rust_type = 'i64' if 'big' in mysql_type else 'i32'
        if 'unsigned' in mysql_type:
            rust_type = 'u' + rust_type[1:]
    elif 'float' in mysql_type:
        rust_type = 'f32'
    elif 'double' in mysql_type or 'decimal' in mysql_type:
        rust_type = 'f64'
    elif 'bool' in mysql_type:
        rust_type = 'bool'
    elif 'date' in mysql_type or 'time' in mysql_type:
        rust_type = 'chrono::NaiveDateTime'
    elif 'char' in mysql_type or 'text' in mysql_type:
        rust_type = 'String'
    else:
        rust_type = 'String'
    
    return f"Option<{rust_type}>" if is_nullable == 'YES' else rust_type

def get_metadata(table_name: str, config: Dict[str, Any] = None) -> Dict[str, Any]:
    """Genera diccionario de metadatos desde MySQL"""
    columns = get_columns(table_name, config)
    
    # Encontrar primary key
    primary_key = next((col[0] for col in columns if 'PRI' in str(col[3])), 'id')
    
    return {
        'table_name': table_name,
        'singular_name': table_name.rstrip('s'),
        'struct_name': table_name.capitalize(),
        'primary_key': primary_key,
        'id_type': mysql_to_rust_type(
            next(col[1] for col in columns if col[0] == primary_key),
            next(col[3] for col in columns if col[0] == primary_key)
        ),
        'columns': [
            {
                'name': col[0],
                'type': mysql_to_rust_type(col[1], col[3]),
                'is_nullable': col[3] == 'YES',
                'is_primary': 'PRI' in str(col[3]),
                'is_auto_increment': 'auto_increment' in str(col[5]).lower(),
                'mysql_type': col[1],
                'default_value': col[4],
                'comment': col[8] or ''
            }
            for col in columns
        ]
    }

# Exportar la función principal
get_metadata = get_metadata

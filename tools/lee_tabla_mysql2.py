import os
import sys
import mysql.connector
from mysql.connector import Error

def print_table(columns, headers):
    """Imprime los datos en formato de tabla"""
    # Calcular el ancho máximo para cada columna
    col_widths = [
        max(len(str(item)) for item in col)
        for col in zip(*([headers] + columns))
    ]
    
    # Imprimir encabezados
    header_row = " | ".join(
        f"{h:<{w}}" for h, w in zip(headers, col_widths)
    )
    print("\n" + header_row)
    print("-" * len(header_row))
    
    # Imprimir filas
    for row in columns:
        print(" | ".join(
            f"{str(item):<{w}}" for item, w in zip(row, col_widths)
        ))

def mysql_to_rust_type(mysql_type, is_nullable):
    """Convierte tipos MySQL a tipos Rust, considerando NULL como Option<T>"""
    mysql_type = mysql_type.lower()
    rust_type = ""

    # Determinar el tipo base
    if 'int' in mysql_type:
        if 'unsigned' in mysql_type:
            rust_type = 'u64' if 'big' in mysql_type else 'u32'
        else:
            rust_type = 'i64' if 'big' in mysql_type else 'i32'
    elif 'float' in mysql_type:
        rust_type = 'f32'
    elif 'double' in mysql_type or 'decimal' in mysql_type:
        rust_type = 'f64'
    elif 'bool' in mysql_type:
        rust_type = 'bool'
    elif 'date' in mysql_type or 'time' in mysql_type:
        rust_type = 'chrono::NaiveDateTime'
    elif 'char' in mysql_type or 'text' in mysql_type or 'blob' in mysql_type:
        rust_type = 'String'
    else:
        rust_type = 'String'  # Tipo por defecto

    # Convertir a Option<T> si el campo permite NULL
    return f"Option<{rust_type}>" if is_nullable == 'YES' else rust_type

def generate_rust_struct(table_name, columns):
    """Genera una estructura Rust a partir de los metadatos de la tabla"""
    rust_code = f"#[derive(Debug, Clone, sqlx::FromRow)]\n"
    rust_code += f"pub struct {table_name.capitalize()} {{\n"
    
    for column in columns:
        field_name = column[0]
        rust_type = mysql_to_rust_type(column[1], column[2])  # column[2] es NULL
        rust_code += f"    pub {field_name}: {rust_type},\n"
    
    rust_code += "}\n"
    return rust_code

def get_table_columns(table_name, command=None):
    """Obtiene las columnas de una tabla y las muestra o genera estructura Rust"""
    try:
        user = os.getenv('MYSQL_DEV_USER')
        password = os.getenv('MYSQL_DEV_PASS')
        
        if not user or not password:
            print("Error: Configura MYSQL_DEV_USER y MYSQL_DEV_PASS en tu entorno")
            return
        
        connection = mysql.connector.connect(
            host='localhost',
            database='telemedicina',
            user=user,
            password=password
        )
        
        if connection.is_connected():
            cursor = connection.cursor()
            cursor.execute(f"SHOW COLUMNS FROM {table_name}")
            columns = cursor.fetchall()
            
            if not columns:
                print(f"La tabla '{table_name}' no existe o no tiene columnas")
                return
            
            if command == "mkstruc":
                rust_struct = generate_rust_struct(table_name, columns)
                print("\n// Estructura generada automáticamente para Rust")
                print("// Asegúrate de tener estas dependencias en tu Cargo.toml:")
                print("// sqlx = { version = \"0.6\", features = [ \"mysql\", \"runtime-tokio-rustls\" ] }")
                print("// chrono = \"0.4\"\n")
                print(rust_struct)
            else:
                headers = ["Field", "Type", "Null", "Key", "Default", "Extra"]
                print_table(columns, headers)
            
    except Error as e:
        print(f"Error de MySQL: {e}")
    except Exception as e:
        print(f"Error inesperado: {e}")
    finally:
        if 'connection' in locals() and connection.is_connected():
            cursor.close()
            connection.close()

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Uso: python script.py <nombre_tabla> [comando]")
        print("Comandos disponibles:")
        print("  mkstruc - Genera una estructura Rust para la tabla")
        sys.exit(1)
    
    table_name = sys.argv[1]
    command = sys.argv[2] if len(sys.argv) > 2 else None
    get_table_columns(table_name, command)

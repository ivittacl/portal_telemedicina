import json
from pathlib import Path
from typing import Dict, Any

def load_json_schema(file_path: str) -> Dict[str, Any]:
    """Carga y valida el esquema JSON"""
    path = Path(file_path)
    if not path.exists():
        raise FileNotFoundError(f"Archivo no encontrado: {file_path}")
    
    with open(path, 'r', encoding='utf-8') as f:
        schema = json.load(f)
    
    # Validación básica
    required_keys = {'table_name', 'columns'}
    if not all(key in schema for key in required_keys):
        raise ValueError(f"Esquema inválido. Faltan campos requeridos: {required_keys}")
    
    return schema

def json_to_metadata(file_path: str) -> Dict[str, Any]:
    """Convierte esquema JSON a metadatos para generación"""
    schema = load_json_schema(file_path)
    
    # Procesar columnas
    processed_columns = []
    for col in schema['columns']:
        processed_columns.append({
            'name': col['name'],
            'type': col.get('rust_type', 'String'),  # Tipo Rust personalizado o String por defecto
            'is_nullable': col.get('nullable', False),
            'is_primary': col.get('primary_key', False),
            'is_auto_increment': col.get('auto_increment', False),
            'mysql_type': col.get('db_type', ''),
            'default_value': col.get('default'),
            'comment': col.get('comment', '')
        })
    
    # Determinar primary key
    primary_key = next(
        (col['name'] for col in schema['columns'] if col.get('primary_key')),
        'id'
    )
    
    return {
        'table_name': schema['table_name'],
        'singular_name': schema.get('singular_name', schema['table_name'].rstrip('s')),
        'struct_name': schema.get('struct_name', schema['table_name'].capitalize()),
        'primary_key': primary_key,
        'id_type': next(
            (col['type'] for col in processed_columns if col['name'] == primary_key),
            'i32'
        ),
        'columns': processed_columns
    }

def get_metadata(table_name: str, config: Dict[str, Any] = None) -> Dict[str, Any]:
    """
    Interface consistente con otros generadores.
    table_name se usa para buscar el archivo correspondiente si config no especifica ruta.
    """
    if config and 'file_path' in config:
        file_path = config['file_path']
    else:
        file_path = f"schemas/{table_name}.json"
    
    return json_to_metadata(file_path)

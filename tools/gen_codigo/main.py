#!/usr/bin/env python3
import argparse
import importlib
import os
from pathlib import Path
from typing import Dict, Any

def load_generator(source_type: str):
    """Carga dinámicamente el módulo generador adecuado"""
    try:
        module = importlib.import_module(f"generators.{source_type}")
        return module
    except ImportError:
        raise ValueError(f"Tipo de fuente no soportado: {source_type}")

def ensure_output_dir():
    """Crea el directorio de salida si no existe"""
    output_dir = Path("generated")
    output_dir.mkdir(exist_ok=True)
    return output_dir

def write_output(content: str, filename: str, subdir: str = None):
    """Escribe el contenido generado en un archivo"""
    output_dir = ensure_output_dir()
    if subdir:
        (output_dir / subdir).mkdir(exist_ok=True)
        path = output_dir / subdir / filename
    else:
        path = output_dir / filename
    
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"✓ Archivo generado: {path}")

def generate_code(metadata: Dict[str, Any], command: str):
    """Genera el código basado en los metadatos y el comando"""
    from generators.base import generate_all_handlers, generate_dtos, generate_entity
    
    if command == "all":
        # Generar todo
        handlers = generate_all_handlers(metadata)
        dtos = generate_dtos(metadata)
        entity = generate_entity(metadata)
        
        # Escribir archivos
        write_output(entity, f"{metadata['table_name']}.rs", "entities")
        write_output("\n\n".join(dtos.values()), f"{metadata['table_name']}_dtos.rs", "dtos")
        
        handlers_file = f"{metadata['table_name']}_handlers.rs"
        write_output("\n\n".join(handlers.values()), handlers_file, "handlers")
        
        # Generar módulo principal
        mod_content = f"pub mod entities::{metadata['table_name']};\n"
        mod_content += f"pub mod handlers::{metadata['table_name']}_handlers;\n"
        mod_content += f"pub mod dtos::{metadata['table_name']}_dtos;"
        write_output(mod_content, "mod.rs")
        
    elif command == "handlers":
        handlers = generate_all_handlers(metadata)
        write_output("\n\n".join(handlers.values()), 
                    f"{metadata['table_name']}_handlers.rs", 
                    "handlers")
    # ... otros comandos ...

def main():
    parser = argparse.ArgumentParser(
        description="Generador de código para APIs RESTful"
    )
    parser.add_argument("table", help="Nombre de la tabla a generar")
    parser.add_argument("--source", default="mysql", 
                       choices=["mysql", "postgres", "json"],
                       help="Fuente de los metadatos")
    parser.add_argument("--command", default="all",
                       choices=["all", "handlers", "dtos", "entity"],
                       help="Tipo de código a generar")
    parser.add_argument("--config", help="Archivo de configuración para la fuente")
    
    args = parser.parse_args()

    try:
        # 1. Cargar el generador adecuado
        generator = load_generator(args.source)
        
        # 2. Obtener metadatos
        metadata = generator.get_metadata(args.table, args.config)
        
        # 3. Generar código
        generate_code(metadata, args.command)
        
        print("\nGeneración completada exitosamente!")
        
    except Exception as e:
        print(f"\n❌ Error durante la generación: {str(e)}")
        if os.getenv("DEBUG"):
            raise

if __name__ == "__main__":
    main()

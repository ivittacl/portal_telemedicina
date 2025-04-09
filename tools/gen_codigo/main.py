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

def generate_code(metadata: Dict[str, Any], command: str) -> bool:
    """
    Genera el código basado en los metadatos.
    Devuelve True si tuvo éxito, False si falló.
    """
    from generators.base import (
        validate_context,
        generate_entity,
        generate_dtos,
        generate_all_handlers
    )
    from pathlib import Path
    import traceback

    try:
        # Validar y completar el contexto
        context = validate_context(metadata)
        
        if command == "all":
            print(f"\n🔧 Generando código para tabla: {context['table_name']}")
            
            # Generar componentes
            entity = generate_entity(context)
            dtos = generate_dtos(context)
            handlers = generate_all_handlers(context)
            
            # Crear directorios
            output_dir = Path("generated")
            (output_dir / "entities").mkdir(exist_ok=True, parents=True)
            (output_dir / "dtos").mkdir(exist_ok=True)
            (output_dir / "handlers").mkdir(exist_ok=True)
            
            # Escribir archivos
            (output_dir / "entities" / f"{context['table_name']}.rs").write_text(entity)
            (output_dir / "dtos" / f"{context['table_name']}_dtos.rs").write_text("\n\n".join(dtos.values()))
            (output_dir / "handlers" / f"{context['table_name']}_handlers.rs").write_text("\n\n".join(
                f"// {name.upper()}\n{code}" for name, code in handlers.items()
            ))
            
            print(f"✅ Código generado en: {output_dir}")
            return True
            
        else:
            raise ValueError(f"Comando no soportado: {command}")
            
    except Exception as e:
        print(f"\n❌ Error durante la generación: {str(e)}")
        if os.getenv("DEBUG"):
            traceback.print_exc()
        return False

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

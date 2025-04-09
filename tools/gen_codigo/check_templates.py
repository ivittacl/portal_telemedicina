from pathlib import Path

def verify_templates(verbose=False):
    """Verifica que todas las plantillas requeridas existan."""
    required = {
        'entity.jinja2': "Estructura principal de la entidad",
        'get.jinja2': "Handler GET por ID",
        'get_all.jinja2': "Handler GET todos los registros",
        'post.jinja2': "Handler POST",
        'put.jinja2': "Handler PUT",
        'delete.jinja2': "Handler DELETE",
        'create_dto.jinja2': "DTO para creación",
        'update_dto.jinja2': "DTO para actualización"
    }
    
    templates_dir = Path(__file__).parent / "templates"
    missing = []

    for tpl, desc in required.items():
        if not (templates_dir / tpl).exists():
            missing.append(f"- {tpl.ljust(20)} ({desc})")

    if missing:
        if verbose:
            print("❌ Plantillas faltantes:")
            print("\n".join(missing))
            print(f"\n💡 Solución: Crea estos archivos en: {templates_dir}")
        return False
    
    if verbose:
        print("✓ Todas las plantillas están presentes")
    return True

if __name__ == "__main__":
    verify_templates(verbose=True)

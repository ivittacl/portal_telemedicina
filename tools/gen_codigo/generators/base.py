from jinja2 import Environment, FileSystemLoader
from pathlib import Path

def setup_templates():
    """Configura el entorno de plantillas"""
    templates_path = Path(__file__).parent.parent / "templates"
    env = Environment(
        loader=FileSystemLoader(templates_path),
        trim_blocks=True,
        lstrip_blocks=True,
        keep_trailing_newline=True
    )
    env.filters['indent'] = lambda text, n: text.replace('\n', '\n' + ' ' * n)
    return env

def generate_from_template(template_name, context):
    """Función base para generación desde plantilla"""
    env = setup_templates()
    template = env.get_template(template_name)
    return template.render(context)

def generate_entity(metadata):
    """Genera la estructura principal de la entidad"""
    return generate_from_template("entity.jinja2", metadata)

def generate_all_handlers(metadata):
    """Genera todos los handlers para una entidad"""
    templates = {
        'get': 'get.jinja2',
        'get_all': 'get_all.jinja2',
        'post': 'post.jinja2',
        'put': 'put.jinja2',
        'delete': 'delete.jinja2'
    }
    
    return {
        name: generate_from_template(tpl, metadata)
        for name, tpl in templates.items()
    }

def generate_dtos(metadata):
    """Genera los DTOs para creación y actualización"""
    return {
        'create_dto': generate_from_template("create_dto.jinja2", {
            **metadata,
            'struct_name': f"{metadata['struct_name']}Create"
        }),
        'update_dto': generate_from_template("update_dto.jinja2", {
            **metadata,
            'struct_name': f"{metadata['struct_name']}Update"
        })
    }

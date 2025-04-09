from jinja2 import Environment, FileSystemLoader, TemplateNotFound
from pathlib import Path
import os

# --- Constantes para valores por defecto ---
DEFAULT_CONTEXT = {
    'connection_code': "let mut conn = get_conn().await?;",
    'response_code': "Ok(HttpResponse::Ok().json(item))",
    'error_handler': """
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound("Registro no encontrado"),
        _ => AppError::DatabaseError(e.to_string()),
    })?""",
    'pagination_query': """
    #[derive(Debug, Deserialize)]
    pub struct PaginationQuery {
        #[serde(default = "default_limit")]
        pub limit: Option<i64>,
        #[serde(default)]
        pub offset: Option<i64>,
    }"""
}

def setup_templates():
    """Configura el entorno de plantillas"""
    templates_path = Path(__file__).parent.parent / "templates"
    if not templates_path.exists():
        raise FileNotFoundError(f"Directorio de plantillas no encontrado: {templates_path}")
    
    env = Environment(
        loader=FileSystemLoader(templates_path),
        trim_blocks=True,
        lstrip_blocks=True,
        keep_trailing_newline=True
    )
    env.filters['indent'] = lambda text, n: text.replace('\n', '\n' + ' ' * n)
    return env

def validate_context(context: dict) -> dict:
    """
    Valida y completa el contexto con valores por defecto.
    Devuelve un diccionario con todas las claves necesarias.
    """
    required_keys = {
        'table_name',
        'singular_name',
        'struct_name',
        'primary_key',
        'id_type',
        'columns'
    }
    
    missing = required_keys - set(context.keys())
    if missing:
        raise KeyError(f"Faltan claves obligatorias en el contexto: {missing}")
    
    return {**DEFAULT_CONTEXT, **context}

def generate_from_template(template_name: str, context: dict) -> str:
    """Genera código desde una plantilla con contexto validado"""
    env = setup_templates()
    try:
        template = env.get_template(template_name)
        return template.render(context)
    except TemplateNotFound as e:
        available = os.listdir(env.loader.searchpath)
        raise ValueError(
            f"Plantilla faltante: {template_name}\n"
            f"Disponibles: {', '.join(available)}"
        ) from e

def generate_entity(context: dict) -> str:
    """Genera la estructura principal de la entidad"""
    return generate_from_template("entity.jinja2", context)

def generate_all_handlers(context: dict) -> dict:
    """Genera todos los handlers para una entidad"""
    templates = {
        'get': 'get.jinja2',
        'get_all': 'get_all.jinja2',
        'post': 'post.jinja2',
        'put': 'put.jinja2',
        'delete': 'delete.jinja2'
    }
    return {
        name: generate_from_template(tpl, context)
        for name, tpl in templates.items()
    }

def generate_dtos(context: dict) -> dict:
    """Genera los DTOs para creación y actualización"""
    return {
        'create_dto': generate_from_template("create_dto.jinja2", {
            **context,
            'struct_name': f"{context['struct_name']}Create"
        }),
        'update_dto': generate_from_template("update_dto.jinja2", {
            **context,
            'struct_name': f"{context['struct_name']}Update"
        })
    }

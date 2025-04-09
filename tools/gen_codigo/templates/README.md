# Plantillas necesarias

El generador requiere estas plantillas:

- `entity.jinja2`       - Para estructuras Rust principales
- `create_dto.jinja2`   - Para DTOs de creación
- `update_dto.jinja2`   - Para DTOs de actualización
- `get.jinja2`          - Handler GET por ID
- `get_all.jinja2`      - Handler GET todos los registros
- `post.jinja2`         - Handler POST
- `put.jinja2`          - Handler PUT
- `delete.jinja2`       - Handler DELETE

Ejemplo mínimo para `get.jinja2`:

```rust
#[get("/{{ table_name }}/{id}")]
async fn get_{{ singular_name }}(
    id: web::Path<{{ id_type }}>,
) -> Result<HttpResponse, AppError> {
    // Contenido del handler...
}

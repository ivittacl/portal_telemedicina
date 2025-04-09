// GET
#[get("/usuarios/{id}")]
async fn get_usuario(id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let mut conn = get_conn().await?;
    
    let query = "SELECT * FROM usuarios WHERE id = ?";
    
    let usuario: Usuarios = sqlx::query_as(query)
        .bind(id.into_inner())
        .fetch_one(&mut conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("usuarios no encontrado"),
            _ => AppError::DatabaseError(e.to_string()),
        })?;

    Ok(HttpResponse::Ok().json(item))
}


// GET_ALL
#[get("/usuarios")]
async fn get_all_usuarios(
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, AppError> {
    let mut conn = get_conn().await?;

    let usuarios = sqlx::query_as!(
        Usuarios,
        r#"
        SELECT * FROM usuarios
        LIMIT ?
        OFFSET ?
        "#,
        query.limit.unwrap_or(100),
        query.offset.unwrap_or(0)
    )
    .fetch_all(&mut conn)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(usuarios))
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}

fn default_limit() -> Option<i64> {
    Some(100)
}


// POST
#[post("/usuarios")]
async fn create_usuario(
    item: web::Json<UsuariosCreate>,
) -> Result<HttpResponse, AppError> {
    let mut conn = get_conn().await?;

    let created_id = sqlx::query!(
        r#"INSERT INTO usuarios (
            id,             rut,             nombre,             ap_paterno,             ap_materno,             email,             telefonos,             cod_zona,             nivel_acceso,             cod_cliente,             clave_acceso,             estatus,             fecha_creacion,             fecha_actualizacion        ) VALUES (
            ?,             ?,             ?,             ?,             ?,             ?,             ?,             ?,             ?,             ?,             ?,             ?,             ?,             ?        )"#,
        item.id,        item.rut,        item.nombre,        item.ap_paterno,        item.ap_materno,        item.email,        item.telefonos,        item.cod_zona,        item.nivel_acceso,        item.cod_cliente,        item.clave_acceso,        item.estatus,        item.fecha_creacion,        item.fecha_actualizacion    )
    .execute(&mut conn)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .last_insert_id();

    Ok(HttpResponse::Ok().json(item))
}


// PUT
#[put("/usuarios/{id}")]
async fn update_usuario(
    id: web::Path<i32>,
    item: web::Json<UsuariosUpdate>,
) -> Result<HttpResponse, AppError> {
    let mut conn = get_conn().await?;

    let rows_affected = sqlx::query!(
        r#"
        UPDATE usuarios
        SET
            id = ?,            rut = ?,            nombre = ?,            ap_paterno = ?,            ap_materno = ?,            email = ?,            telefonos = ?,            cod_zona = ?,            nivel_acceso = ?,            cod_cliente = ?,            clave_acceso = ?,            estatus = ?,            fecha_creacion = ?,            fecha_actualizacion = ?        WHERE id = ?
        "#,
        item.id,        item.rut,        item.nombre,        item.ap_paterno,        item.ap_materno,        item.email,        item.telefonos,        item.cod_zona,        item.nivel_acceso,        item.cod_cliente,        item.clave_acceso,        item.estatus,        item.fecha_creacion,        item.fecha_actualizacion        id.into_inner()
    )
    .execute(&mut conn)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::NotFound("Registro no encontrado"));
    }

    let updated_item = sqlx::query_as!(
        Usuarios,
        "SELECT * FROM usuarios WHERE id = ?",
        id.into_inner()
    )
    .fetch_one(&mut conn)
    .await?;

    Ok(HttpResponse::Ok().json(updated_item))
}


// DELETE
#[delete("/usuarios/{id}")]
async fn delete_usuario(
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = get_conn().await?;

    let rows_affected = sqlx::query!(
        "DELETE FROM usuarios WHERE id = ?",
        id.into_inner()
    )
    .execute(&mut conn)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::NotFound("Registro no encontrado"));
    }

    Ok(HttpResponse::NoContent().finish())
}

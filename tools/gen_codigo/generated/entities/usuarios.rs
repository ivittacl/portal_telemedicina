#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usuarios {
    pub id: i32,     pub rut: String,     pub nombre: String,     pub ap_paterno: String,     pub ap_materno: Option<String>,     pub email: Option<String>,     pub telefonos: Option<String>,     pub cod_zona: Option<String>,     pub nivel_acceso: Option<i32>,     pub cod_cliente: Option<i32>,     pub clave_acceso: Option<String>,     pub estatus: Option<i32>,     pub fecha_creacion: chrono::NaiveDateTime,     pub fecha_actualizacion: chrono::NaiveDateTime, }

impl Usuarios {
    /// Crea una nueva instancia de Usuarios
    pub fn new(id: i32, rut: String, nombre: String, ap_paterno: String, ap_materno: Option<String>, email: Option<String>, telefonos: Option<String>, cod_zona: Option<String>, nivel_acceso: Option<i32>, cod_cliente: Option<i32>, clave_acceso: Option<String>, estatus: Option<i32>, fecha_creacion: chrono::NaiveDateTime, fecha_actualizacion: chrono::NaiveDateTime) -> Self {
        Self {
            id: id,
            rut: rut,
            nombre: nombre,
            ap_paterno: ap_paterno,
            ap_materno: ap_materno,
            email: email,
            telefonos: telefonos,
            cod_zona: cod_zona,
            nivel_acceso: nivel_acceso,
            cod_cliente: cod_cliente,
            clave_acceso: clave_acceso,
            estatus: estatus,
            fecha_creacion: fecha_creacion,
            fecha_actualizacion: fecha_actualizacion,
        }
    }
}

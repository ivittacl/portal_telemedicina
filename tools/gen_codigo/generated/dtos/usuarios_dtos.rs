#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsuariosCreateCreate {
    #[validate(
        required(message = "El campo id es requerido"),
    )]
    pub id: i32,
    #[validate(
        required(message = "El campo rut es requerido"),
        length(min = 1, max = 12)
    )]
    pub rut: String,
    #[validate(
        required(message = "El campo nombre es requerido"),
        length(min = 1, max = 50)
    )]
    pub nombre: String,
    #[validate(
        required(message = "El campo ap_paterno es requerido"),
        length(min = 1, max = 50)
    )]
    pub ap_paterno: String,
    #[validate(
        length(min = 1, max = 50)
    )]
    pub ap_materno: Option<Option<String>>,
    #[validate(
        length(min = 1, max = 100)
    )]
    pub email: Option<Option<String>>,
    #[validate(
        length(min = 1, max = 100)
    )]
    pub telefonos: Option<Option<String>>,
    #[validate(
        length(min = 1, max = 6)
    )]
    pub cod_zona: Option<Option<String>>,
    #[validate(
    )]
    pub nivel_acceso: Option<Option<i32>>,
    #[validate(
    )]
    pub cod_cliente: Option<Option<i32>>,
    #[validate(
        length(min = 1, max = 100)
    )]
    pub clave_acceso: Option<Option<String>>,
    #[validate(
    )]
    pub estatus: Option<Option<i32>>,
    #[validate(
        required(message = "El campo fecha_creacion es requerido"),
    )]
    pub fecha_creacion: chrono::NaiveDateTime,
    #[validate(
        required(message = "El campo fecha_actualizacion es requerido"),
    )]
    pub fecha_actualizacion: chrono::NaiveDateTime,
}


#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsuariosUpdateUpdate {
    #[validate(
        required(message = "El campo id es requerido"),
    )]
    pub id: i32,
    #[validate(
        required(message = "El campo rut es requerido"),
        length(min = 1, max = 12)
    )]
    pub rut: String,
    #[validate(
        required(message = "El campo nombre es requerido"),
        length(min = 1, max = 50)
    )]
    pub nombre: String,
    #[validate(
        required(message = "El campo ap_paterno es requerido"),
        length(min = 1, max = 50)
    )]
    pub ap_paterno: String,
    #[validate(
        length(min = 1, max = 50)
    )]
    pub ap_materno: Option<Option<String>>,
    #[validate(
        length(min = 1, max = 100)
    )]
    pub email: Option<Option<String>>,
    #[validate(
        length(min = 1, max = 100)
    )]
    pub telefonos: Option<Option<String>>,
    #[validate(
        length(min = 1, max = 6)
    )]
    pub cod_zona: Option<Option<String>>,
    #[validate(
    )]
    pub nivel_acceso: Option<Option<i32>>,
    #[validate(
    )]
    pub cod_cliente: Option<Option<i32>>,
    #[validate(
        length(min = 1, max = 100)
    )]
    pub clave_acceso: Option<Option<String>>,
    #[validate(
    )]
    pub estatus: Option<Option<i32>>,
    #[validate(
        required(message = "El campo fecha_creacion es requerido"),
    )]
    pub fecha_creacion: chrono::NaiveDateTime,
    #[validate(
        required(message = "El campo fecha_actualizacion es requerido"),
    )]
    pub fecha_actualizacion: chrono::NaiveDateTime,
}

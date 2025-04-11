use serde::{Deserialize, Serialize};
use mysql_async::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Usuario {
    pub id: u32,
    pub rut: String,
    pub nombre: String,
    pub ap_paterno: String,
    pub ap_materno: String,
    pub email: String,
    pub telefonos: String,
    pub cod_zona: Option<String>,
    pub nivel_acceso: Option<u8>,
    pub cod_cliente: Option<u8>,
    pub clave_acceso: String,
    pub estatus: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsuarioInput {
    pub id: u32,
    pub rut: String,
    pub nombre: String,
    pub ap_paterno: String,
    pub ap_materno: String,
    pub email: String,
    pub telefonos: String,
    pub cod_zona: String,
    pub nivel_acceso: u8,
    pub cod_cliente: u8,
    pub clave_acceso: String,
    pub estatus: u8,
}

impl TryFrom<UsuarioInput> for Usuario {
    type Error = crate::error::AppError;

    fn try_from(input: UsuarioInput) -> Result<Self, Self::Error> {
        Ok(Self {
            id: input.id,
            rut: input.rut,
            nombre: input.nombre,
            ap_paterno: input.ap_paterno,
            ap_materno: input.ap_materno,
            email: input.email,
            telefonos: input.telefonos,
            cod_zona: Some(input.cod_zona),
            nivel_acceso: Some(input.nivel_acceso),
            cod_cliente: Some(input.cod_cliente),
            clave_acceso: input.clave_acceso,
            estatus: input.estatus,
        })
    }
}
CREATE TABLE usuarios (
    id INT AUTO_INCREMENT PRIMARY KEY,
    rut VARCHAR(12) NOT NULL,
    nombre VARCHAR(50) NOT NULL,
    ap_paterno VARCHAR(50) NOT NULL,
    ap_materno VARCHAR(50),
    email VARCHAR(100),
    telefonos VARCHAR(100),
    zona_acceso 
    nivel_acceso INT DEFAULT 0,
    cliente ENUM('', 'MOSTRAR', 'ATENCIONES DOMICILIARIAS', 'ONCOVIDA') DEFAULT '',
    clave_acceso VARCHAR(100),
    estatus ENUM('', 'MOSTRAR', 'BLOQUEADO', 'FALLECIDO', 'RETIRADO', 'REVISAR', 'VIGENTE') DEFAULT '',
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    UNIQUE KEY (rut),
    INDEX (email)
);
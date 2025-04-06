USE telemedicina;

INSERT INTO usuarios(rut, nombre, ap_paterno, ap_materno, email, telefonos, cod_zona, nivel_acceso, cod_cliente, clave_acceso, estatus)
VALUES ('0010895960-6', 'CLAUDIO', 'SAEZ', 'CATALAN', 'claudiosaez@telemed-chile.cl', '', NULL, 0, NULL, sha1('Tele2024'), 1),
       ('0011625357-7', 'LUZ VALERIA', 'RIQUELME', '', 'controlgestion@telemed-chile.cl', '', '047', 0, 2, sha1('LVR88'), 1),
       ('0000000001-9', '', '', '', 'controlgestion@telemed-chile.cl', '', NULL, NULL, 2, sha1('candelabro'), 1),
       ('0012006773-7', 'MARIA DE LA PAZ', 'GAZMURI', 'BARROS', '', '', NULL, 0, NULL, sha1('6773'), 1);

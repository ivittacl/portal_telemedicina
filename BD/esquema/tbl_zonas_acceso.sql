USE telemedicina;

CREATE TABLE zonas_acceso (
    cod_zona   CHAR(6) NOT NULL,
    nom_zona   VARCHAR(100) NOT NULL,
    orden_zona INT,
    PRIMARY KEY (cod_zona)
);

insert into zonas_acceso(cod_zona, nom_zona, orden_zona)
      select '082', ' San Javier', 6
union select '076', ' Vicuña', 7
union select '071', 'Ancud', 8
union select '017', 'Antofagasta', 9
union select '040', 'Arauco', 10
union select '004', 'Arica', 11
union select '039', 'CASTRO', 12
union select '049', 'Calama', 13
union select '008', 'Calera', 14
union select '056', 'Cauquenes', 15
union select '066', 'Chanco', 16
union select '019', 'Chiguayante', 17
union select '005', 'Chillán', 18
union select '055', 'Chiloe', 19
union select '020', 'Chimbarongo', 20
union select '057', 'Collipulli', 21
union select '021', 'Coltauco', 22
union select '012', 'Concepción', 23
union select '006', 'Copiapó', 24
union select '022', 'Coquimbo', 25
union select '045', 'Coyhaique', 26
union select '023', 'Curacaví', 27
union select '007', 'Curicó', 28
union select '024', 'Dalcahue', 29
union select '025', 'El Tabo', 30
union select '026', 'Fresia', 31
union select '074', 'Frutillar', 32
union select '009', 'Iquique', 33
union select '068', 'La Ligua', 34
union select '077', 'La Serena', 35
union select '011', 'La Unión', 36
union select '075', 'Laja', 37
union select '058', 'Lanco', 38
union select '069', 'Lautaro', 39
union select '080', 'Licantén', 40
union select '015', 'Linares', 41
union select '018', 'Litoral', 42
union select '044', 'Llanquihue', 43
union select '053', 'Los Andes', 44
union select '041', 'Los Angeles', 45
union select '079', 'Metropolitana', 46
union select '047', 'Metropolitana Norte', 47
union select '042', 'Metropolitana Sur', 48
union select '050', 'Molina', 49
union select '028', 'Mostazal', 50
union select '052', 'Nacimiento', 51
union select '072', 'Natales', 52
union select '054', 'Negrete', 53
union select '010', 'Osorno', 54
union select '027', 'Ovalle', 55
union select '081', 'Parral', 56
union select '067', 'Peralillo', 57
union select '029', 'Pichilemu', 58
union select '013', 'Pitrufquén', 59
union select '062', 'Puchuncaví', 60
union select '078', 'Pucón', 61
union select '002', 'Puerto Montt', 62
union select '035', 'Punta Arenas', 63
union select '051', 'Quillota', 64
union select '037', 'Quillón', 65
union select '038', 'Quillón-Chillan', 66
union select '048', 'Quilpué', 67
union select '059', 'Quintero', 68
union select '065', 'Quirihue', 69
union select '043', 'RANCAGUA', 70
union select '064', 'Salamanca', 71
union select '003', 'San Felipe', 72
union select '030', 'San Fernando', 73
union select '031', 'San Vicente', 74
union select '032', 'Santa Cruz', 75
union select '070', 'Talagante', 76
union select '061', 'Talca', 77
union select '060', 'Temuco', 78
union select '033', 'Teodoro Schmidt', 79
union select '046', 'Traiguén', 80
union select '001', 'V Región', 81
union select '014', 'VI Región', 82
union select '063', 'Valdivia', 83
union select '073', 'Vallenar', 84
union select '034', 'Villarrica', 85
union select '016', 'Viña del Mar', 86
union select '036', 'Chimbarongo - San Fernando', 87
;

use telemedicina;

drop table if exists tablas;

/*==============================================================*/
/* Table: tablas                                                */
/*==============================================================*/
create table tablas
(
   cod_tabla            int not null,
   nom_tabla            varchar(100) not null,
   primary key (cod_tabla)
);

insert into tablas(cod_tabla, nom_tabla)
values (1, 'ESTADO USUARIO'),
       (2, 'CLIENTES USUARIO');
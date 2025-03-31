use telemedicina;

drop table if exists subtablas;

/*==============================================================*/
/* Table: subtablas                                             */
/*==============================================================*/
create table subtablas
(
   cod_tabla            int not null,
   codigo               int not null,
   valor                varchar(100) not null,
   primary key (cod_tabla, codigo)
);

alter table subtablas add constraint fk_subtablas_tablas foreign key (cod_tabla)
      references tablas (cod_tabla) on delete restrict on update restrict;

insert into subtablas(cod_tabla, codigo, valor)
values (1,  1, 'VIGENTE'),
       (1, 40, "BLOQUEADO"),
       (1, 50, "FALLECIDO"),
       (1, 98, "REVISAR"),
       (1, 99, "RETIRADO"),
       (2,  1, "ONCOVIDA"),
       (2,  2, "ATENCIONES DOMICILIARIAS");
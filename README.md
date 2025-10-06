# ProyectoSED022025-Terraplanistas

[Rust sin frameworks](https://doc.rust-lang.org/stable/)
###### Necesita PostgreSQL funcionando localmente (no contenedor) y rustc (con cargo) instalados

### Dependencias (de rust)
+ Para la base de datos
  + [Postgres](https://docs.rs/postgres/latest/postgres/)
  + [Postgres-types](https://docs.rs/postgres-types/latest/postgres_types/)
  + [UUID](https://docs.rs/uuid/latest/uuid/)
+ Otras
  + [Argon2](https://docs.rs/argon2/latest/argon2/)
  + [Regex](https://docs.rs/regex/latest/regex/)

---

Reinicio de postgres
```bash
cd db
./cluster_reset.sh
```

Ejecución del proyecto con logs de desarrollo
```bash
./run.sh d
```
o simulación de producción
```bash
./run.sh p
```

Con respecto a la creación de writings, es necesario haber creado con anticipación los directorios donde se almacenarán las imágenes. 
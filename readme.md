To get started:

```
podman run  --env MARIADB_ROOT_PASSWORD=my-secret-pw -p 3306:3306 mariadb
DATABASE_URL=mysql://root:my-secret-pw@localhost:3306/mysql sea-orm-cli migrate refresh
cargo run
```
from os import environ
from subprocess import run

env = {
  **environ,
  "DB_HOST": "localhost",
  "DB_PORT": "8765",
  "DB_NAME": "srtest",
  "DB_USER": "postgres",
  "DB_PASSWORD": "root"
}
container = "db_postgres_srtest_8765"

try:
  run(["python", "init_db.py"], env=env, check=True)
  run(["cargo", "test"], env=env, check=True)
finally:
  run(["docker", "rm", "-f", "-v", container])
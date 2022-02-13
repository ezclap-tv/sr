from os import environ
from subprocess import run
from argparse import ArgumentParser

parser = ArgumentParser()
parser.add_argument(
  "-w", "--watch",
  help="Run the tests in watch mode",
  action="store_true",
  default=False
)
args = parser.parse_args()
watch = args.watch

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
  run(["cargo", "test", "--features", "test-database"], env=env)
  while watch:
    v = input("re-run tests (enter), quit (q): ")
    if v == "q": break
    run(["cargo", "test", "--features", "test-database"], env=env)
finally:
  run(["docker", "rm", "-f", "-v", container])
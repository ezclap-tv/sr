from os import environ
from subprocess import run, Popen, PIPE, DEVNULL
from shutil import which
from time import sleep

if not which("cargo"):
  print("Error: Cargo is not installed")
  exit(1)
if not which("docker"):
  print("Error: Docker is not installed")
  exit(1)
if not which("sqlx"):
  print("Error: sqlx is not installed")
  print("  cargo install sqlx-cli --no-default-features --features postgres")
  exit(1)

user = environ.get("DB_USER", "postgres")
password = environ.get("DB_PASSWORD", "root")
name = environ.get("DB_NAME", "sr")
port = environ.get("DB_PORT", "5678")
db_url = f"postgres://localhost:{port}/{name}?user={user}&password={password}"
container = f"db_{user}_{name}_{port}"

# check if db is running
p = Popen(["docker", "inspect", "-f", "{{.State.Running}}", container], stdout=PIPE)
status = p.communicate()[0].decode().strip("'\n")
if p.returncode != 0 or status != "true":
  # start db container
  print(f"Starting database container at 'postgres://localhost:{port}/{name}?user={user}&password=<...>")
  run(
    [
      "docker", "run",
      "-e", f"POSTGRES_USER={user}",
      "-e", f"POSTGRES_PASSWORD={password}",
      "-e", f"POSTGRES_DB={name}",
      "-p", f"{port}:5432",
      "--name", container,
      "-d", "postgres",
      "postgres", "-N", "1000" # 1000 max connections
    ],
    check=True
  )

  # wait for database to be ready
  print("Waiting for database readiness")
  while True:
    try:
      run(
        [
          "sqlx", "migrate", "info",
          f'--database-url=postgres://localhost:{port}/{name}?user={user}&password={password}'
        ],
        stdout=DEVNULL,
        stderr=DEVNULL,
        check=True)
      break
    except Exception:
      print("...")
      sleep(1)
else:
  print("Database running")

# init database
print("Creating database")
run(
  [
    "sqlx", "database", "create",
    f'--database-url=postgres://localhost:{port}/{name}?user={user}&password={password}'
  ],
  check=True
)
print("Running migrations")
run(
  [
    "sqlx", "migrate", "run",
    f'--database-url=postgres://localhost:{port}/{name}?user={user}&password={password}'
  ],
  check=True
)

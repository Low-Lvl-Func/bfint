#!/bin/bash
SERVICE="app"

case "$1" in
  run)
    shift # Remove 'run' from the arguments list
    # We tell docker to run 'cargo run --' and THEN pass your BF string
    docker compose run --rm $SERVICE cargo run -- "$@"
    ;;
  build)
    docker compose build
    ;;
  stop)
    docker compose down
    ;;
  clean)
    docker compose down -v
    ;;
  *)
    echo "Usage: ./run.sh {run|build|stop|clean} [args]"
    ;;
esac
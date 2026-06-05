# -*- mode: Python -*-

local_resource(
    'database',
    serve_cmd='docker compose up postgres --wait',
    readiness_probe=probe(
        exec=exec_action(['docker', 'compose', 'exec', 'postgres', 'pg_isready', '-U', 'dev']),
        period_secs=5,
    ),
)

#!/usr/bin/env bash
set -euo pipefail

printf '%s\n' 'import_void_templates.sh is deprecated.' >&2
printf '%s\n' 'Void templates are no longer a runtime dependency; use the already generated pag-repo/packages tree.' >&2
printf '%s\n' 'If you need to regenerate manifests, do it from an external snapshot of templates, not from a checked-in void-pac tree.' >&2
exit 1

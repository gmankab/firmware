#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Device-independent low-memory offline verification.
set -euo pipefail
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
exec "$ROOT/tools/check-local.sh" --full

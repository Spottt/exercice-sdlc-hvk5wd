#!/usr/bin/env bash
# PreToolUse sur Edit/Write : exit 2 refuse l'écriture et renvoie le message à l'agent.
input=$(cat)
path=$(jq -r '.tool_input.file_path // .tool_input.notebook_path // empty' <<<"$input")
[ -z "$path" ] && exit 0
root=${CLAUDE_PROJECT_DIR:-$(git rev-parse --show-toplevel)}
rel=${path#"$root"/}
step=$(awk '{print $1; exit}' "$root/.sdlc-step" 2>/dev/null)

deny() { echo "guard.sh : $rel refusé ($1). Voir sdlc/constitution.md." >&2; exit 2; }

case $rel in
  sdlc/* | .claude/* | Makefile | CLAUDE.md) deny "fichier du cadre, humain seulement" ;;
esac
if [ "$step" = implementer ]; then
  case $rel in
    api/tests/* | *.test.ts) deny "tests gelés pendant 04 Implémenter" ;;
  esac
fi
exit 0

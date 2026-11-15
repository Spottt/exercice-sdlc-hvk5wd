#!/usr/bin/env bash
# PreToolUse sur Edit/Write/Bash : exit 2 refuse l'action et renvoie le message à l'agent.
input=$(cat)
deny() { echo "guard.sh : $1. Voir sdlc/constitution.md." >&2; exit 2; }

cmd=$(jq -r '.tool_input.command // empty' <<<"$input")
if [ -n "$cmd" ]; then
  grep -Eq 'make([[:space:]].*)?[[:space:]]valider' <<<"$cmd" && deny "make valider est la signature de l'humain (R12)"
  grep -q 'Validation humaine' <<<"$cmd" && grep -Eq '>|tee|-i' <<<"$cmd" && deny "seul l'humain écrit « Validation humaine » (R12)"
  exit 0
fi

path=$(jq -r '.tool_input.file_path // .tool_input.notebook_path // empty' <<<"$input")
[ -z "$path" ] && exit 0
root=${CLAUDE_PROJECT_DIR:-$(git rev-parse --show-toplevel)}
rel=${path#"$root"/}
step=$(awk '{print $1; exit}' "$root/.sdlc-step" 2>/dev/null)

case $rel in
  sdlc/* | .claude/* | Makefile | CLAUDE.md) deny "$rel refusé : fichier du cadre, humain seulement (R3)" ;;
esac
if [ "$step" = implementer ]; then
  case $rel in
    api/tests/* | *.test.ts) deny "$rel refusé : tests gelés pendant 04 Implémenter (R2)" ;;
  esac
fi
jq -r '[.tool_input.content, .tool_input.new_string, (.tool_input.edits // [] | .[].new_string)] | map(select(.)) | join("\n")' <<<"$input" \
  | grep -q 'Validation humaine' && deny "seul l'humain écrit « Validation humaine » (R12)"
exit 0

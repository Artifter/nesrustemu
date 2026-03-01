#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
GRAY='\033[0;90m'
RESET='\033[0m'

ok()   { echo -e "${GREEN}  ok${RESET}  $1"; }
fail() { echo -e "${RED}  fail${RESET}  $1"; exit 1; }
info() { echo -e "${GRAY}  >>  $1${RESET}"; }

echo ""
echo "  GIT PUSHER"
echo ""

read -p "  commit message: " msg

[ -z "$msg" ] && fail "commit message cannot be empty."
echo ""

info "adding files..."
git add . 2>&1 && ok "added" || fail "git add failed"

info "committing..."
git commit -m "$msg" 2>&1 && ok "committed" || fail "git commit failed"

info "pulling..."
git pull origin main --rebase 2>&1 && ok "pulled" || fail "git pull failed"

info "pushing..."
git push origin main 2>&1 && ok "pushed" || fail "git push failed"

echo ""
echo "  done. code is live."
echo ""
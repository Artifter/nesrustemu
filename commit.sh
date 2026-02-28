#!/bin/bash

echo "  _   _ _____ ____  "
echo " | \ | | ____/ ___| "
echo " |  \| |  _| \___ \ "
echo " | |\  | |___ ___) |"
echo " |_| \_|_____|____/ "
echo "      git pusher     "
echo ""
echo "Enter commit message (or 'q' to quit):"
read commit_message

if [ "$commit_message" = "q" ]; then
    echo ""
    echo "( ._.) bye bye..."
    echo "c(\")(\")"
    exit 0
fi

echo ""
echo "( •_•) adding files..."
echo "c(\")(\")"
git add .

echo ""
echo "( •_•) committing..."
echo "c(\")(\")"
git commit -m "$commit_message"

echo ""
echo "( •_•)⊃ pushing to github..."
echo "c(\")(\")"
git push origin main

echo ""
echo "٩(^‿^)۶ done! code is live!"
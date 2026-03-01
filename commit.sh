#!/bin/bash

LOGFILE="/tmp/git_push_log.txt"

dialog --title "🐙 git pusher" \
       --inputbox "commit message:" 8 50 \
       2>/tmp/git_msg.txt

[ $? -ne 0 ] && clear && exit 0

msg=$(cat /tmp/git_msg.txt)

if [ -z "$msg" ]; then
    dialog --title "błąd" --msgbox "commit message nie może być pusty!" 6 40
    clear
    exit 1
fi

(
    echo "10"; echo "# dodawanie plików..."
    git add . >> $LOGFILE 2>&1

    echo "30"; echo "# commitowanie..."
    git commit -m "$msg" >> $LOGFILE 2>&1

    echo "60"; echo "# pulling..."
    git pull origin main --rebase >> $LOGFILE 2>&1

    echo "90"; echo "# pushing..."
    git push origin main >> $LOGFILE 2>&1

    echo "100"; echo "# gotowe!"
) | dialog --title "🐙 git pusher" --gauge "pracuję..." 8 50 0

if grep -qi "error\|fatal\|failed" $LOGFILE; then
    dialog --title "❌ błąd" \
           --scrollbox $LOGFILE 20 70
else
    dialog --title "✅ gotowe!" \
           --msgbox "٩(^‿^)۶ code is live!" 6 40
fi

rm -f /tmp/git_msg.txt $LOGFILE
clear
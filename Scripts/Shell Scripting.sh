#!/bin/bash

[ "$#" -ne 4 ] && { echo "Usage: $0 COURSE YEAR LETTER NUM_SECTIONS"; exit 1; }

course="$1"
year="$2"
letter="$3"
count="$4"

base="${course}-SY${year}-$((year+1))"

echo "Script Started"

for ((i=1; i<=count; i++)); do
    sec="${letter}${i}"
    folder="${base}-${sec}"

    mkdir "$folder" 2>/dev/null && echo "Folder Created: $folder" || echo "Folder Skipped: $folder"

    mkdir "$folder/Materials" 2>/dev/null && echo "Folder Created: $folder/Materials" || echo "Folder Skipped: $folder/Materials"
    mkdir "$folder/Exams"    2>/dev/null && echo "Folder Created: $folder/Exams"    || echo "Folder Skipped: $folder/Exams"
done

echo "Script Ended"

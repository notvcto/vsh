#!/usr/bin/env vsh
# Example VSH script - Backup files

# This script demonstrates VSH's readable syntax
# All three syntax styles work!

# Terse style (bash-compatible)
# cp important.txt backup/

# Verbose style (beginner-friendly)
copy important.txt to backup/

# Named parameter style (explicit)
# copy source=important.txt destination=backup/

# List the backup directory
list backup/

# You can mix styles!
list backup/ | grep ".txt"

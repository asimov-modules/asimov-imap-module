#!/bin/sh
#$ delay 80
asimov list imaps://imap.ietf.org/Shared%20Folders/json-canon -o json
#$ expect scalvin@usrobots:
#$ wait 10000

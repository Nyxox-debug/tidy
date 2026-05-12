#!/bin/bash
# reset_testfolder.sh
rm -rf testfolder/*
mkdir -p testfolder
touch testfolder/photo.jpg testfolder/resume.pdf testfolder/notes.txt testfolder/clip.mp4 testfolder/unknown.xyz

echo "testfolder reset"

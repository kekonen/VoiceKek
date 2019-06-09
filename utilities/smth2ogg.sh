#!/bin/bash
ffmpeg -i $1 -ac 1 -map 0:a -codec:a libopus -b:a 128k $2
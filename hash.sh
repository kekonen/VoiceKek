#!/bin/bash
sha256sum $1 | cut -d " " -f 1

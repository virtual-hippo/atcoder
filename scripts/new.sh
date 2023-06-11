#!/bin/sh

cargo compete new $1 && cd $1 cargo compete t a

#!/bin/bash
cp -r ./_template $1
sed -i -e "s/TEMPLATE_NAME/$1/g" "$1/Cargo.toml"
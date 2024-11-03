#!/bin/bash
# Script used to regenerate bindgen.rs
mkdir -p target
echo "use lv2_raw::*; type LV2_Feature = LV2Feature;" >./target/suil-import-lv2.rs
bindgen /usr/include/suil-0/suil/suil.h --allowlist-type "Suil.*" --blocklist-type "_*LV2.*" --allowlist-function "suil_.*" --allowlist-var "SUIL.*" >./target/suil-bindgen.rs
cat ./target/suil-import-lv2.rs ./target/suil-bindgen.rs >./src/bindgen.rs

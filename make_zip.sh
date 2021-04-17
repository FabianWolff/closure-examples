#!/bin/bash

ZIPNAME="additional_material"

rm -f "$ZIPNAME.zip"
rm -rf "$ZIPNAME"

mkdir -p "$ZIPNAME"

for f in all.rs any.rs any_err.rs blameassgn.rs blameassgn_err.rs counter.rs \
    counter_err.rs delegation.rs fold_list_rev.rs fold_list_rev.sil map_vec.rs \
    option_map.rs option_map_err.rs repeat_with_n.rs result_uoe.rs README.md; do
    cp "$f" "$ZIPNAME"
done
find "$ZIPNAME" -name ._\* -delete # OS X files on networked drives...

zip -r "$ZIPNAME.zip" "$ZIPNAME"

rm -rf "$ZIPNAME"

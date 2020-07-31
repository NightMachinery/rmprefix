arr0 123Hello hi '' 123a '' 123 ''  12j ggg | cargo run  -- '123' '\x00' '\x00' |cat -v

ec 'c\x00c' | rmprefix '' '\x00' 'HO'|cat -v

ec 'c\x00c' | rmprefix '' 'c\x00' 'HO'|cat -v

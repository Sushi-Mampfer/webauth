nssm stop webauth
cargo build --release
Copy-Item -Path "target\release\webauth.dll" -Destination "C:\Windows\System32\" -Force
Copy-Item -Path "target\release\webauth.exe" -Destination "C:\Program Files\webauth\" -Force
nssm start webauth
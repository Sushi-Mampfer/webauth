cargo build --release -p provider
Copy-Item -Path "target\release\webauth.dll" -Destination "C:\Windows\System32\" -Force
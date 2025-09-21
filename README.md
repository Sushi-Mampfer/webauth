# Webauth
A windows credential provider that allows you to unlock your PC with a web request to it.

### Installation
- Install NSSM from [GitHub](https://github.com/dkxce/NSSM) or [chocolatey](https://community.chocolatey.org/packages/NSSM)
- Go to the latest release and download `webauth.dll` to `C:\Windows\System32`
- Go to the latest release and download `webauth.exe` to some path, I used `C:\Program Files\webauth\`
- Run the register.reg file from this repo
- Run `nssm install webauth` in an elevated cmd
- Set the path to your `webauth.exe`
- In `Details` set `Startup type` to `Automatic`
- In `Dependencies` add the following:
```
Tcpip
LanmanWorkstation
Dhcp
NlaSvc
```
- Make sure that in `Log on` `Local system account` is selected
- Hit `Install service`
- Run `nssm start webauth` in an elevated cmd
- Reboot your PC

### Usage
- Open `http://<ip>:4242/?user=<username>&passwd=<passwd>` in any browser that can reach you PC
- You should be logged in
- This probably only works on local accounts(for now)

### Security
- The service is bound to all interfaces and therefore anyone that can access port 4242 can try to unlock your pc
- Nothing is encrypted, everyone in your network is able to read your username and password

### Credits
- https://github.com/SubconsciousCompute/windows-credential-provider-rs
- https://stackoverflow.com/questions/75279682/implementing-a-windows-credential-provider
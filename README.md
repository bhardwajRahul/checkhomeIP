# CheckHomeIP

A small and lightweight program to monitor your home IP 24/7 and send an email to yourself using Gmail SMTP. Useful for people who don't have DDNS and need to know if their ISP assigned IP has been changed.

## Getting started
To get started, head to https://myaccount.google.com/apppasswords and make a new app password. Once done, paste it into `src/mail.rs` line 6 along side your Gmail account. Thats it! Just compile and run.

### Systemd service setup
To enable systemd service automation, edit `checkip.service` and change the line containing `ExecStart=/path/to/checkhomeip` to where the binary is in your filesystem.

Once done, change the line `User=user` to the user account you wish the service to run as and copy/move the file to `/etc/systemd/system/`

Finally, enable the service and start it using `systemctl enable --now checkip.service`


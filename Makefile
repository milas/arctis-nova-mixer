.PHONY: system-permissions
system-permissions:
	sudo cp ./extra/linux/99-usb-arctis-nova.rules /etc/udev/rules.d
	sudo udevadm control --reload-rules
	sudo udevadm trigger

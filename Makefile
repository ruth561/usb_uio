# The bus id of xHCI.
xhci_id 	= $(shell lspci -nn | grep xHCI | grep -o '\[....\:....\]' | sed 's#\[##' | sed 's#\]##' | sed 's#\:# #')
xhci_bus_id	= $(addprefix 0000:,$(shell lspci | grep xHCI | cut -d ' ' -f 1))

.PHONY: run
run: 
	cargo build
	sudo sudo target/debug/usb_uio

# loads modules and registers the vendor/device id of the xhci
.PHONY: setup_uio_xhci
setup_uio_xhci:
	modprobe uio_pci_generic
	echo $(xhci_id) | sudo tee /sys/bus/pci/drivers/uio_pci_generic/new_id

.PHONY: enable_uio_xhci
enable_uio_xhci:
	modprobe uio_pci_generic
	echo -n $(xhci_bus_id) | sudo tee /sys/bus/pci/drivers/xhci_hcd/unbind > /dev/null
	echo -n $(xhci_bus_id) | sudo tee /sys/bus/pci/drivers/uio_pci_generic/bind > /dev/null

.PHONY: disable_uio_xhci
disable_uio_xhci:
	modprobe uio_pci_generic
	echo -n $(xhci_bus_id) | sudo tee /sys/bus/pci/drivers/uio_pci_generic/unbind > /dev/null
	echo -n $(xhci_bus_id) | sudo tee /sys/bus/pci/drivers/xhci_hcd/bind > /dev/null

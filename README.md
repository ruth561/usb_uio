# usb_uio
Simple implementation of USB Linux usermode driver.

### Goals
- Implementations that you can easily port to other non-uio program.

### How to use
You need to be able to use kernel module 'uio' and 'uio_pci_generic'.
```
$ make setup_uio_xhci
$ make enable_uio_xhci
$ make run
```
! If you successfully execute `make enable_uio_xhci`, the usb device will stop responding. !

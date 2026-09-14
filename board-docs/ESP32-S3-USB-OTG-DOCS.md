  window.dataLayer = window.dataLayer || \[\]; function gtag(){dataLayer.push(arguments);} gtag('js', new Date()); gtag('config', 'G-6GCDQQ87G0');   ESP32-S3-USB-OTG - ESP32-S3 - — esp-dev-kits latest documentation        DOCUMENTATION\_OPTIONS.PAGENAME = 'esp32-s3-usb-otg/user\_guide'; DOCUMENTATION\_OPTIONS.PROJECT\_SLUG = 'esp-dev-kits'; DOCUMENTATION\_OPTIONS.LATEST\_BRANCH\_NAME = 'master'; DOCUMENTATION\_OPTIONS.VERSIONS\_URL = '.././\_static/js/docs\_version.js'; DOCUMENTATION\_OPTIONS.LANGUAGES = \["en", "zh\_CN"\]; DOCUMENTATION\_OPTIONS.IDF\_TARGET = 'esp32s3'; DOCUMENTATION\_OPTIONS.HAS\_IDF\_TARGETS = \["esp32", "esp32s2", "esp32s3", "esp32c3", "esp32c6", "esp32h2", "esp32c2", "esp32p4", "esp32c5", "esp32c61", "esp32s31", "other"\] DOCUMENTATION\_OPTIONS.RELEASE = 'latest'; DOCUMENTATION\_OPTIONS.LANGUAGE\_URL = 'en';    

[esp-dev-kits ![Logo](../_static/espressif-logo.svg)](../index.html) 

Choose target...

Choose version...

  

ESP32-S3 Boards

*   [ESP32-S3-DevKitC-1](../esp32-s3-devkitc-1/index.html)
*   [ESP32-S3-USB-OTG](index.html)
    *   [User Guide](#)
        *   [Getting Started](#getting-started)
            *   [Description of Components](#description-of-components)
            *   [Application Examples](#application-examples)
            *   [Start Application Development](#start-application-development)
            *   [Contents and Packaging](#contents-and-packaging)
        *   [Hardware Reference](#hardware-reference)
            *   [Block Diagram](#block-diagram)
            *   [Power Supply Options](#power-supply-options)
            *   [USB HOST Interface Power Options](#usb-host-interface-power-options)
            *   [USB Interface Switch Circuit](#usb-interface-switch-circuit)
            *   [LCD Interface](#lcd-interface)
            *   [SD Card Interface](#sd-card-interface)
            *   [Charging Circuit](#charging-circuit)
            *   [Pin Layout](#pin-layout)
        *   [Related Documents](#related-documents)
*   [ESP32-S3-LCD-EV-Board](../esp32-s3-lcd-ev-board/index.html)
*   [ESP-VoCat](../esp-vocat/index.html)
*   [ESP-DualKey](../esp-dualkey/index.html)
*   [EOL (End of Life) Boards](../eol/eol-boards.html)

Resources and Legal Notices

*   [Related Documentation and Resources](../resources.html)
*   [Disclaimer and Copyright Notice](../disclaimer-and-copyright.html)

[esp-dev-kits](../index.html)

*   [](../index.html)
*   [ESP32-S3-USB-OTG](index.html)
*   ESP32-S3-USB-OTG
.download-pdf { float: right; margin-right: 20px; } [Download PDF](../esp-dev-kits-en-master-esp32s3.pdf)

* * *

ESP32-S3-USB-OTG[](#esp32-s3-usb-otg "Permalink to this heading")
==================================================================

[\[中文\]](../../../../zh_CN/latest/esp32s3/esp32-s3-usb-otg/user_guide.html)

ESP32-S3-USB-OTG is a development board that focuses on USB-OTG function verification and application development. It is based on ESP32-S3 SoC, supports Wi-Fi and BLE 5.0 wireless functions, and supports USB host and USB device functions. It can be used to develop applications such as wireless storage devices, Wi-Fi network cards, LTE MiFi, multimedia devices, virtual keyboards and mice. The development board has the following features:

*   Onboard ESP32-S3-MINI-1-N8 module, with built-in 8 MB flash
    
*   Onboard USB Type-A host and device interface, with built-in USB interface switching circuit
    
*   Onboard USB to serial debugging chip (Micro USB interface)
    
*   Onboard 1.3-inch LCD color screen, supports GUI
    
*   Onboard SD card interface, compatible with SDIO and SPI interfaces
    
*   Onboard charging IC, can be connected to lithium battery
    

[![ESP32-S3-USB-OTG](../_images/pic_product_esp32_s3_otg.png)](../_images/pic_product_esp32_s3_otg.png)

ESP32-S3-USB-OTG (click to enlarge)[](#id4 "Permalink to this image")

**The document consists of the following major sections:**

*   [Getting Started](#getting-started): Provides a brief overview of ESP32-S3-USB-OTG and necessary hardware and software information.
    
*   [Hardware Reference](#hardware-reference): Provides detailed hardware information of ESP32-S3-USB-OTG.
    
*   [Related Documents](#related-documents): Provides links to related documents.
    

Getting Started[](#getting-started "Permalink to this heading")
----------------------------------------------------------------

This section describes how to start using ESP32-S3-USB-OTG. It includes introduction to basic information about ESP32-S3-USB-OTG first, and then on how to start using the development board for application development, as well as board packaging and retail information.

### Description of Components[](#description-of-components "Permalink to this heading")

The ESP32-S3-USB-OTG development board includes the following parts:

*   **Motherboard:** ESP32-S3-USB-OTG motherboard is the core of the kit. The motherboard integrates the ESP32-S3-MINI-1 module and provides an interface of the 1.3-inch LCD screen.
    

[![ESP32-S3-USB-OTG](../_images/pic_board_top_lable.png)](../_images/pic_board_top_lable.png)

ESP32-S3-USB-OTG Top View (click to enlarge)[](#id5 "Permalink to this image")

The following table starts with the USB\_HOST Interface on the left, and introduces the main components in the above figure in an anticlockwise order.

 

Main components

Description

USB\_HOST Interface

USB Type-A female port, used to connect other USB devices.

ESP32-S3-MINI-1 Module

ESP32-S3-MINI-1 is a powerful, generic Wi-Fi + Bluetooth LE MCU module that has a rich set of peripherals. It has strong ability for neural network computing and signal processing. ESP32-S3-MINI-1 comes with a PCB antenna and is pin-to-pin compatible with ESP32-S2-MINI-1.

MENU Button

Menu button.

Micro SD Card Slot

Micro SD card can be inserted. Both four-line SDIO and SPI mode are supported.

USB Switch IC

By setting the level of USB\_SEL, you can switch USB peripherals to make them either connected to the USB\_DEV interface or the USB\_HOST interface. USB\_DEV will be connected by default.

Reset Button

Press this button to restart the system.

USB\_DEV Interface

USB Type-A male port, can be connected to the USB host, and also used as a lithium battery charge power source.

Power Switch

Switch to ON to use battery power. Switch to OFF to power off battery.

Boot Button

Download button. Holding down Boot and then pressing Reset initiates Firmware Download mode for downloading firmware through the serial port.

DW- Button

Down button.

LCD FPC Connector

Used to connect the 1.3-inch LCD screen.

UP+ Button

Up button.

USB-to-UART Interface

A Micro-USB port used for power supply to the board, for flashing applications to the chip, as well as for communication with the chip via the on-board USB-to-UART bridge.

[![ESP32-S3-USB-OTG](../_images/pic_board_bottom_lable.png)](../_images/pic_board_bottom_lable.png)

ESP32-S3-USB-OTG Bottom View (click to enlarge)[](#id6 "Permalink to this image")

The following table starts with the Yellow LED on the left, and introduces the main components in the above figure in an anticlockwise order.

 

Main components

Description

Yellow LED

Driven by GPIO16, set high level to turn on.

Green LED

Driven by GPIO15, set high level to turn on.

Charging LED

During charging, the red light is on, which will be turned off when charged.

Battery Solder Joints

3.6 V lithium battery can be welded to power the motherboard.

Charging Circuit

Used to charge lithium battery.

Free Pins

Idle pins that can be customized.

USB-to-UART Bridge

Single USB-to-UART bridge chip provides transfer rates up to 3 Mbps.

*   **Subboard:** ESP32-S3-USB-OTG-SUB mount the 1.3-inch LCD screen
    

[![ESP32-S3-USB-OTG](../_images/pic_sub.png)](../_images/pic_sub.png)

ESP32-S3-USB-OTG Subboard (click to enlarge)[](#id7 "Permalink to this image")

### Application Examples[](#application-examples "Permalink to this heading")

The following application examples are available for ESP32-S3-USB-OTG:

*   [factory](https://github.com/espressif/esp-dev-kits/tree/77a138d/examples/esp32-s3-usb-otg/examples/factory) \- Demonstrates a factory demo for the ESP32-S3-USB-OTG development board, providing a reference for building, flashing, and monitoring projects using ESP-IDF and ESP Launchpad.
    

For more examples and the latest updates, please refer to the [examples](https://github.com/espressif/esp-dev-kits/tree/77a138d/examples/esp32-s3-usb-otg) folder.

To explore the application examples or to develop your own, please follow the steps outlined in the [Start Application Development](#start-application-development) section.

### Start Application Development[](#start-application-development "Permalink to this heading")

Before powering on the ESP32-S3-USB-OTG, please make sure that the development board is intact.

#### Required Hardware[](#required-hardware "Permalink to this heading")

*   ESP32-S3-USB-OTG
    
*   A USB 2.0 data cable (standard A to Micro-B)
    
*   Computer (Windows, Linux or macOS)
    

#### Software Setup[](#software-setup "Permalink to this heading")

Please proceed to [Get Started](https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/get-started/index.html), where Section [Installation Step by Step](https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/get-started/index.html#get-started-step-by-step) will quickly help you set up the development environment and then flash an application example onto your board.

### Contents and Packaging[](#contents-and-packaging "Permalink to this heading")

#### Retail Orders[](#retail-orders "Permalink to this heading")

If you order a few samples, each board comes in an individual package in either an antistatic bag or any packaging depending on your retailer.

[![ESP32-S3-USB-OTG](../_images/pic_product_package.png)](../_images/pic_product_package.png)

ESP32-S3-USB-OTG Package (click to enlarge)[](#id8 "Permalink to this image")

Which contains the following parts:

*   Motherboard:
    
    *   ESP32-S3-USB-OTG
        
*   Subboard:
    
    *   ESP32-S3-USB-OTG\_SUB
        
*   Fastener
    
    *   Mounting bolt (x4)
        
    *   Screw (x4)
        
    *   Nut (x4)
        

For retail orders, please go to [https://www.espressif.com/zh-hans/company/contact/buy-a-sample](https://www.espressif.com/zh-hans/company/contact/buy-a-sample).

#### Wholesale Order[](#wholesale-order "Permalink to this heading")

If purchased in bulk, the development board will be packaged in a large cardboard box.

For wholesale orders, please go to [https://www.espressif.com/en/contact-us/sales-questions](https://www.espressif.com/en/contact-us/sales-questions).

Hardware Reference[](#hardware-reference "Permalink to this heading")
----------------------------------------------------------------------

### Block Diagram[](#block-diagram "Permalink to this heading")

The block diagram below shows the components of ESP32-S3-USB-OTG and their interconnections.

[![ESP32-S3-USB-OTG](../_images/sch_function_block.png)](../_images/sch_function_block.png)

ESP32-S3-USB-OTG Block Diagram (click to enlarge)[](#id9 "Permalink to this image")

Please note that the external interface corresponding to the `USB_HOST D+ D-` signal in the functional block diagram is `USB DEV`, which means that ESP32-S3 is used as a device to receive signals from other USB hosts. The external interface corresponding to the `USB_DEV D+ D-` signal is `USB HOST`, which means that ESP32-S3 acts as a host to control other devices.

### Power Supply Options[](#power-supply-options "Permalink to this heading")

There are three power supply methods for the development board:

1.  Power supply through the `Micro_USB` interface
    
    *   Use the USB cable (standard A to Micro-B) to connect the motherboard to a power supply device, and set battery switch to OFF. Please note that in this power supply mode, only the motherboard and display are powered.
        
2.  Power supply through the `USB_DEV` interface
    
    *   Set `DEV_VBUS_EN` to high level, and set the battery switch to OFF. This mode can supply power to the `USB HOST` interface. The lithium battery will be charged at the same time (if the lithium battery is installed)
        
3.  Power supply through the battery
    
    *   Set `BOOST_EN` to high level, and set the battery switch to ON. You should solder a 1-Serial lithium battery (3.7 V ~ 4.2 V) to the power solder joint reserved on the back of the motherboard first. This mode can supply power to the `USB HOST` interface at the same time. The battery interface description is as follows:
        

[![ESP32-S3-USB-OTG](../_images/pic_board_battery_lable.png)](../_images/pic_board_battery_lable.png)

Battery Connection (click to enlarge)[](#id10 "Permalink to this image")

### USB HOST Interface Power Options[](#usb-host-interface-power-options "Permalink to this heading")

The `USB HOST` interface (Type-A female port) can supply power to the connected USB device. The power supply voltage is 5 V and the maximum current is 500 mA.

*   There are two power supply methods for the `USB HOST` interface:
    
    1.  Power is supplied through the `USB_DEV` interface, and the 5 V power is directly from the power source connected to the interface.
        
    2.  Power is supplied through the lithium battery, and the 3.6 V ~ 4.2 V voltage of the lithium battery is boosted to 5 V through the Boost circuit. The working status of Boost IC can be controlled by BOOST\_EN/GPIO13, set high to enable Boost.
        

[![ESP32-S3-USB-OTG](../_images/sch_boost_circuit.png)](../_images/sch_boost_circuit.png)

Boost Circuit (click to enlarge)[](#id11 "Permalink to this image")

*   `USB HOST` interface power supply selection:
    

  

BOOST\_EN

DEV\_VBUS\_EN

Power Source

0

1

USB\_DEV

1

0

Battery

0

0

No output

1

1

Undefined

[![ESP32-S3-USB-OTG](../_images/sch_power_switch.png)](../_images/sch_power_switch.png)

Power Switch Circuit (click to enlarge)[](#id12 "Permalink to this image")

*   500 mA current limiting circuit:
    
    1.  The current limiting IC MIC2005A can limit the maximum output current of the `USB HOST` interface to 500 mA. Please set the `IDEV_LIMIT_EN` (GPIO17) to high level to enable the current-limiting IC to output voltage.
        

[![ESP32-S3-USB-OTG](../_images/sch_500ma_limit.png)](../_images/sch_500ma_limit.png)

500 mA Current Limiting Circuit (click to enlarge)[](#id13 "Permalink to this image")

### USB Interface Switch Circuit[](#usb-interface-switch-circuit "Permalink to this heading")

[![ESP32-S3-USB-OTG](../_images/sch_usb_switch.png)](../_images/sch_usb_switch.png)

USB Interface Switch Circuit (click to enlarge)[](#id14 "Permalink to this image")

*   When **USB\_SEL** (GPIO18) is set to high level, the USB D+/D- Pin (GPIO19, 20) will be connected to `USB_DEV D+ D-`. Then you can use the `USB HOST` interface (Type-A female Port) to connect other USB devices.
    
*   When **USB\_SEL** (GPIO18) is set to low level, the USB D+/D- Pin (GPIO19, 20) will be connected to `USB_HOST D+ D-`. Then you can use the `USB DEV` interface (Type-A male port) to connect to a host like a PC.
    
*   **USB\_SEL** is pulled low level by default.
    

### LCD Interface[](#lcd-interface "Permalink to this heading")

[![ESP32-S3-USB-OTG](../_images/sch_interface_lcd.png)](../_images/sch_interface_lcd.png)

LCD Interface Circuit (click to enlarge)[](#id15 "Permalink to this image")

Please note that this interface supports connecting SPI interface screens. The screen controller used by this development board is [ST7789](https://dl.espressif.com/AE/esp-dev-kits/ST7789VW芯片手册.pdf), and `LCD_BL` (GPIO9) can be used to control the screen backlight.

### SD Card Interface[](#sd-card-interface "Permalink to this heading")

[![ESP32-S3-USB-OTG](../_images/sch_micro_sd_slot.png)](../_images/sch_micro_sd_slot.png)

SD Card Interface Circuit (click to enlarge)[](#id16 "Permalink to this image")

Please note that the SD card interface is compatible with 1-wire, 4-wire SDIO mode and SPI mode. After being powered on, the card will be in 3.3 V signaling mode. Please send the first CMD0 command to select the bus mode: SD mode or SPI mode.

### Charging Circuit[](#charging-circuit "Permalink to this heading")

[![ESP32-S3-USB-OTG](../_images/sch_charge_circuit.png)](../_images/sch_charge_circuit.png)

Charging Circuit (click to enlarge)[](#id17 "Permalink to this image")

Please note that the Type-A male port can be connected to a power adapter that outputs 5 V. When charging the battery, the red indicator LED is on, after fully charged, the red indicator LED is off. When using the charging circuit, please set the battery switch to OFF. The charging current is 212.7 mA.

### Pin Layout[](#pin-layout "Permalink to this heading")

**Function pin:**

  

No.

ESP32-S3-MINI-1 Pin

Description

1

GPIO18

USB\_SEL: Used to switch the USB interface. When high level, the USB\_HOST interface is enabled. When low level (default), the USB\_DEV interface is enabled.

2

GPIO19

Connect with USB D-.

3

GPIO20

Connect with USB D+.

4

GPIO15

LED\_GREEN: the light is lit when set high level.

5

GPIO16

LED\_YELLOW: the light is lit when set high level.

6

GPIO0

BUTTON\_OK: OK button, low level when pressed.

7

GPIO11

BUTTON\_DW: Down button, low level when pressed.

8

GPIO10

BUTTON\_UP: UP button, low level when pressed.

9

GPIO14

BUTTON\_MENU: Menu button, low level when pressed.

10

GPIO8

LCD\_RET: used to reset LCD, low level to reset.

11

GPIO5

LCD\_EN: used to enable LCD, low level to enable.

12

GPIO4

LCD\_DC: Used to switch data and command status.

13

GPIO6

LCD\_SCLK: LCD SPI Clock.

14

GPIO7

LCD\_SDA: LCD SPI MOSI.

15

GPIO9

LCD\_BL: LCD backlight control.

16

GPIO36

SD\_SCK: SD SPI CLK / SDIO CLK.

17

GPIO37

SD\_DO: SD SPI MISO / SDIO Data0.

18

GPIO38

SD\_D1: SDIO Data1.

19

GPIO33

SD\_D2: SDIO Data2.

20

GPIO34

SD\_D3: SD SPI CS / SDIO Data3.

21

GPIO1

HOST\_VOL: USB\_DEV voltage monitoring, ADC1 channel 0.

22

GPIO2

BAT\_VOL: Battery voltage monitoring, ADC1 channel 1.

23

GPIO17

LIMIT\_EN: Enable current limiting IC, high level enable.

24

GPIO21

0VER\_CURRENT: Current overrun signal, high level means overrun.

25

GPIO12

DEV\_VBUS\_EN: High level to enable DEV\_VBUS power supply.

26

GPIO13

BOOST\_EN: High level to enable Boost boost circuit.

**Extended pin:**

  

No.

ESP32-S3-MINI-1 Pin

Description

1

GPIO45

FREE\_1: Idle, can be customized.

2

GPIO46

FREE\_2: Idle, can be customized.

3

GPIO48

FREE\_3: Idle, can be customized.

4

GPIO26

FREE\_4: Idle, can be customized.

5

GPIO47

FREE\_5: Idle, can be customized.

6

GPIO3

FREE\_6: Idle, can be customized.

Related Documents[](#related-documents "Permalink to this heading")
--------------------------------------------------------------------

*   [ESP32-S3 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-s3_datasheet_en.pdf) (PDF)
    
*   [ESP32-S3-MINI-1/1U Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-s3-mini-1_mini-1u_datasheet_en.pdf) (PDF)
    
*   [Espressif Product Selection Tool](https://products.espressif.com/#/product-selector?names=)
    
*   [ESP32-S3-USB-OTG Schematic Diagram](https://dl.espressif.com/dl/schematics/SCH_ESP32-S3_USB_OTG.pdf) (PDF)
    
*   [ESP32-S3-USB-OTG PCB Layout Drawing](https://dl.espressif.com/dl/schematics/PCB_ESP32-S3_USB_OTG.pdf) (PDF)
    
*   [ST7789VW Datasheet](https://dl.espressif.com/dl/schematics/ST7789VW_datasheet.pdf) (PDF)
    

function init() { WaveDrom.ProcessAll(); } window.onload = init;

[Next](../esp32-s3-lcd-ev-board/index.html "ESP32-S3-LCD-EV-Board") [Previous](index.html "ESP32-S3-USB-OTG")

* * *

**Suggestion on this document?**  
  
 [Provide feedback](https://www.espressif.com/en/company/documents/documentation_feedback?docId=7295&sections=ESP32-S3-USB-OTG (esp32-s3-usb-otg/user_guide)&version=latest (master))

**Help improve this document?**  
  
 [Edit on GitHub](https://github.com/espressif/esp-dev-kits/blob/77a138d/docs/en/esp32-s3-usb-otg/user_guide.rst)

**Need more information?**

 [Check ESP forum](https://www.esp32.com/viewforum.php?f=23)

 [Sales Questions](https://www.espressif.com/en/contact-us/sales-questions)

 [Technical Inquiries](https://www.espressif.com/en/contact-us/technical-inquiries)

*     
    © Copyright 2016 - 2026, Espressif Systems (Shanghai) CO., LTD
    
    Built with [Sphinx](http://sphinx-doc.org/) using a [theme](https://github.com/espressif/sphinx_idf_theme) based on [Read the Docs Sphinx Theme](https://github.com/readthedocs/sphinx_rtd_theme).
    

jQuery(function () { SphinxRtdTheme.Navigation.enable(true); });

---
> Part of the [[pc-hid-passthrough]] mouse HID pass-through project. See also [[inventory]] · [[notes]] · [[index]].
> Reusing this board as an image viewer? See [[otg-image-viewer]] — how to put images on the OTG (LittleFS workflow).

# raspian-temp

A small project that I make to print out the temperature out of the raspberry pi 4 1gb on the tm1637 and with a grovepi button attached to it you can push it to switch between fahrenheit and celsius.

## Deploy

The basic deploy of the application is you install it and give it good parameters.

`raspbian_temp '{ "dio_pin" : 23, "clk_pin" : 18, "brightness" : 5, "btn_pin" : 17 }'`

* **dio_pin_tm** => Digital input/output pin of the tm1637. *Ex: pin 23 on raspberry pi 4 1gb*
* **clk_pin_tm** => Clock pin of the tm1637. *Ex: pin 18 on raspberry pi 4 1gb*
* **brightness** => brightness of the tm1637. *Ex: Brightness levels between 0 and 7*
* **button_pin** => Digital signal pin. *Ex: pin 17 on raspberry pi 4 1gb*

## Warning

* **TM1637** needs to have 3.3 or 5 V and a ground connection.
* **Grovepi** button needs to have 3.3 V and a ground connection.
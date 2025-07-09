pwgn is a simple CLI password generator that generates passwords from either a default or user defined format.
I found myself wanting a very easy to access generator that could follow a pre-defined format of my choosing, so I made my own.

By default it will generate a single password of ten characters.

Pass `-r` flag to randomise the output.

Pass a number to generate a password of x characters. Minimum 6 and max 255.
E.g `pwgn 25 // output example qldelmtwmsuADFXGGKWCD66+@`

Pass `-f` followed by a format string for your own format. E.g `pwgen -f uuuunnnnllllssss // outputs VNAE4373keip+*&&`

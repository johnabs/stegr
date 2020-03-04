# Stegr
## Steganography attempt in Rust -- work in progress.

This is still a work in progress, and is about 3/8ths of the way done. I'm just now getting to the encoding the images part, and I expect to have this one ready to enter alpha by the end of the week if all goes well.


This utility in it's simplest use-case accepts a password, a message, and an image, and attempts to hide the messages in an unnoticeable way, unless you have a digital copy of the original on hand as well. The message that is hidden is encrypted with AES-256, and the bits are hidden in randomly generated locations based on the head of your SHA-256 hashed password. Because of this, the password is necessary for both determining which pixels have had their least-significant bits flipped, but also for decrypting the password once the message has been extracted. 

Yet another option is to hide your message recursively using a root file location containing either images or sub-folders with other images. Other information from the environment including filenames, directory names, etc. may be used to add extra randomness to both the pixel order selection, as well as the image order selection. Thus, effectively hiding messages of any size across any number of files, without increasing the size of the images in a noticeable way.

An advantage to this method is that theoretically if a pixel is visited twice, the least significant bit will also be flipped twice, thus rendering 0 net change to that pixel's value, however, this should still be caught by the decoding process. Consequently, any attempts to detect any changes from the original is even more difficult, as a single missing bit would have make it very difficult to extract the encrypted message, for which you would still need the password to decrypt. 


# Extra Stuff 
I am a bit new to low level programming, particularly in Rust, and would love feedback if anyone is willing to provide it. I think there are plenty of cases where this could be useful, particularly when needing to communicate without alerting anyone to what you're saying or the fact that you're saying anything (e.g. planning a birthday party, and preferably other non morally questionable conspiracies), by sending pictures or zip files of your vacation to your collaborators.

If you, for whatever reason, really liked this project and want to support future stuff I do, donations or feedback would be greatly appreciated. 

If you can't tell that I'm already paranoid by the fact I went out of my way to try to write something like this (mostly joking), I'd prefer donations be through cryptocurrencies like Monero, Ethereum, or through BAT, as I'm a Brave Verified Creator. XMR and ETH addresses are listed below, and if you use [Brave](https://brave.com/JOH113) too you can click the little triangle in your browser to send me some of the fancy three-sided shekels (and it pays you to browse while blocking ads, which I think is pretty neat).

ETH: 0x339DCBD53f17159613F10ac6211F70fC2739FD81
XMR: 84x8sQ9TgC5EnRA7UNfE1wgaijhwgZbjRJMerApX5TjHCF8vm2V65tRUeo5TfVW74diippWFKHnWxY1ddaUthgyxLDshGd4  

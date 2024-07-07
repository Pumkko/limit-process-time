# What i Wanted
I wanted to write a program to limit the time i spend playing HOI4.

The plan at first was to build a linux daemon that would detect when i launch HOI4 and kill after one hour of gameplay, no warning nothing (That will teach me !)

Tried different things and learn about the /proc folder and its children, detecting a process launch is not easy, i thought spying on folder creation and deletion under /proc would do
but folders under /proc are not really folders so that did not work.

I decided that detection would be too tricky and a simple Cronjob could do, run the code every 10 minutes if the game has been running for more than an hour at that time kill it.


# FOR THE FUTURE
  - When the cronjob runs, check if HOI4 is running, if so add a new cronjob to kill so game time is exactly one hour
- Actually write the code to read a config file to support more process that just HOI4, i made 80% of the code to do that I just need to write a parser for the config file (thinking about using the ini format)

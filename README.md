# rust-fortune
A fortune teller written in Rust as a beginner project.

Use this little program to display fortunes. For example as a greeting in your Linux shell.

### Linux
Make the program globaly runnable

```sudo ln -s <PATH-TO-EXECUTABLE> /usr/bin/rust-fortune```


### Fish Shell
To make your fish shell greet you with a fortune every time you open up your terminal,
write this out in your terminal.

```
function fish_greeting
     rust-fortune
end

funcsave fish_greeting
```
![image](https://i.imgur.com/RTsbykr.png)
Enjoy!

### Todo
Error handling.. features..

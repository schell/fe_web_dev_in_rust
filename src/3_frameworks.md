# Frameworks

Phwew! You've made it this far. In the last section [we saw how complicated a
simple app can be](./2_web_sys_alert_button.md). It takes an awful lot of code
to simply create a button and attach a callback. Most of that code is about
safely acquiring objects and calling functions, or have to do with marshalling
values between Rust and the browser.

It would be a bummer to have to write this much code for every user experience,
so let's take a quick tour of libraries that will abstract away the complexity.

| library                                       | has vdom | is elm-like | low level (does the user see `web-sys`?) |
| --------------------------------------------- |:--------:|:-----------:|:----------------------------------------:|
| [gloo](https://github.com/rustwasm/gloo)      |          |             | yes                                      |
| [mogwai](https://github.com/schell/mogwai)    |          | sometimes   | kinda                                    |
| [sauron](https://github.com/ivanceras/sauron) | yes      | yes         |                                          |
| [yew](https://github.com/yewstack/yew)        | yes      | yes         |                                          |

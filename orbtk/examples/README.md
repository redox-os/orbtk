# OrbTk examples

[![Build and test](https://github.com/redox-os/orbtk/workflows/CI/badge.svg)](https://github.com/redox-os/orbtk/actions)
![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)

## Basic reference examples

* `minimal`: minimal example

	![minimal](https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/minimal.png)

Howto start your OrbTk journey.

* `showcase`: major reference app

<p float="left">
<img alt="showcase" width="720" src="https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/showcase.png">
</p>

It provides a structured overview of an OrbTk app. We try to
incorporate example implementation that make use of the supported
widgets inside the orbtk-widget crate.

## Specialized examples

* calculator: a calculator example

	![calculator](https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/calculator.png)

* canvas: use third party render library in canvas

	![canvas](https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/canvas.png)

* login: PasswordBox showcase implementing a login form

	<img alt="login" width="360" src="https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/login.png">

* message: MessageAdapter example

	<img alt="message" width="360" src="https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/message.png">

	1. Assign and implement a thread inside the state of a MainView.
	2. Generate a time based loop that sleeps given amount out
	seconds.
	3. When the timer expires, increment a counter and issue a message
	inside the state. A message will be triggered and send via the
	message_adapter.
	4. The receiver side will evaluate (match) the
	message type. The associated function block will increment a
	counter and update a text property inside the MainView. Thus, the View will update that counter every given time period.

* msg_handler: sender-receiver example

	<img alt="msg_handler" width="480" src="https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/msg_handler.png">

* multi_window: multi window example

	![multi_window](https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/multi_window.png)

* overlay: draw widgets on the top of the render stack

	![overlay](https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/overlay.png)

* popup: show how to open and use a popup

	<img alt="popup" width="480" src="https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/popup.png">

* stack: stack layout example

	<img alt="stack" width="360" src="https://raw.githubusercontent.com/redox-os/orbtk/develop/orbtk/screenshots/stack.png">

## License

Licensed under MIT license ([LICENSE](../LICENSE)).

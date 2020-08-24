# Create project

To start your first OrbTk project you can use [cargo](https://doc.rust-lang.org/cargo/) to create and run it.

1. Create your project with the following command:

   ```bash
   cargo new my-project
   ```

2. Now change to the directory of the generated project.

   ```bash
   cd my-project
   ```

3. Inside of this directory is a file called `Cargo.toml`. Add the latest version of OrbTk as dependency on the bottom of the file:

   ```toml
   [package]
   name = "my_project"
   version = "0.1.0"
   authors = ["m_name <my.name@my_mail.org>"]
   edition = "2018"
   
   [dependencies]
   orbtk = "0.3.1-alpha2"
   ```

4. Next replace the content of the file `main.rs` with the following content:

    ```rust,no_run
    use orbtk::prelude::*;

    fn main() {
        Application::new()
            .window(|ctx| {
                Window::create()
                    .title("My first OrbTk app")
                    .position((100.0, 100.0))
                    .size(420.0, 730.0)
                    .child(TextBlock::create().text("Hello World").margin(4.0).build(ctx))
                    .build(ctx)
            })
            .run();
    }
    ```

5. You can start the app by executing the following command:

   ```bash
   cargo run --release
   ```

Now you should you see a window that includes the text *Hello World*.
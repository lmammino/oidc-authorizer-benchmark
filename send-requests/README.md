# send-requests

This is a very simple HTTP load-testing tool written in Async Rust.

The code is probably not the best but it does the job for now.

If you have any suggestions or improvements, feel free to open an issue or a PR. I'd love that! 😍

> [!NOTE]
> You can use this tool to generate tokens for your requests. This can be useful if you want to do some manual testing.
>
> To do that you can run a simple local server with the following command (you'll need a recent version of Node.js):
>
> ```bash
> node test-server.mjs
> ```
>
> Then you can run the tool with the following command:
>
> ```bash
> cargo run -- --target-url http://localhost:3000 --total-requests 5 --requests-per-sec 1
> ```
>
> This will generate 5 requests with a frequency of 1 request per second.
> In the terminal of the Node.js test server you should see the tokens being generated.
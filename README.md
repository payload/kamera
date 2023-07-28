<div align="center">

<h1>kamera</h1>

</div>

Camera API with a reduced feature set for basic usecases and learning.

* 🚧 Mac support is based on AVFoundation
* 🚧 Windows support is based on MediaFoundation
* 🚧 Linux support is based on V4L2

* ❌ tests need to run with a single thread `cargo t -- --test-threads=1 --nocapture`
  and it is good to review the output of the test cases

* ✔️ PR check is manual running tests on Mac, Windows and Linux laptop internal camera device and a Logitech external camera
* ✔️ CI runs checks, formatting and clippy for main and PRs

```rust
use kamera::Camera;

let camera = Camera::new_default_device();
camera.start();

let Some(frame) = camera.wait_for_frame() else { return }; // always blockingly waiting for next new frame
let (w, h) = frame.size_u32();

frame.data().data_u32() // use this buffer, per default in ARGB format
// for real use cases processing or displaying frames can get more complicated when trying to be most efficient

camera.stop() // or drop it
```

## Linux system dependecies

On a Debian like system (MX Linux for example) I needed to install these system dependencies to build all crates:

```sh
sudo apt install build-essentials cmake libfontconfig1-dev clang
```

* TODO winit depends on sctk-adwaita which transitively also is depending on freetype for font rendering. I think
  * winit probably does not have a real need to do font rendering in a default or minimal configuration
  * basic font rendering could also be done by a pure rust dependency
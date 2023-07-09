<div align="center">

<h1>kamera</h1>

</div>

Camera API with a reduced feature set for basic usecases and learning.

* ❌ Linux, Web, Android, iOS and various embedded support is not existent yet.
* 🚧 Mac support is based on AVFoundation and is not behind the Camera API yet.
    * its good to review test print outs too `cargo t -- --nocapture --test-threads=1`
* 🚧 Windows support is based on MediaFoundation.
    * tests need to run with a single thread `cargo t -- --test-threads=1`
* ❌ CI is manual running tests on Mac and Windows with various camera devices.

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
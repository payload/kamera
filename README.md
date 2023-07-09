<div align="center">

<h1>kamera</h1>

</div>

Camera API with a reduced feature set for basic usecases and learning.

* ❌ Linux, Web, Android, iOS and various embedded support is not existent yet.
* 🚧 Mac support is based on AVFoundation and is not behind the Camera API yet.
    * its good to review test print outs too `cargo t -- --nocapture --test-threads=1
* 🚧 Windows support is based on MediaFoundation.
    * tests need to run with a single thread `cargo t -- --test-threads=1`
* ❌ CI is manual running tests on Mac and Windows with various camera devices.
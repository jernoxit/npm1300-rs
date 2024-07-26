# npm1300-rs
Rust implementation for interfacing with Nordic nPM1300

Documentation of the npm1300 can be found at https://docs.nordicsemi.com/bundle/ps_npm1300/page/keyfeatures_html5.html.

## SOC
This crate just does the interfacing with the nPM1300.
The SoC calculation is shipped by Nordic in the form of a binary blob which is not included nor interfaced with here.
So if you want to use Nordics SoC calculation you need to include it in your project and hand over the values you can get from this driver.

In the future this crate might include its own SoC algorithm or at least a ruff approximation of a SoC.
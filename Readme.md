# Notes
* docs can be seen using
``` cargo doc --open ```
* expect on Result finishes program with error, alternative is match on Result
* `cargo new library --lib`
* use includes in scope only what was given - one could use "*"
```
use amodule::sth::aBunchOfFunctions;
aBunchOfFunctions::fun()
```
* use 
```
::someModule::function <- to instruct rust to search inside of main directory
super::someModule::function <- search in parent module 
```

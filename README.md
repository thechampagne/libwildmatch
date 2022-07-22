# libwildmatch

[![](https://img.shields.io/github/v/tag/thechampagne/libwildmatch?label=version)](https://github.com/thechampagne/libwildmatch/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libwildmatch)](https://github.com/thechampagne/libwildmatch/blob/main/LICENSE)

A **C** library to Open a path or URL with the system-defined program.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libwildmatch.git
```
#### 2. Navigate to the root
```
cd libwildmatch
```
#### 3. Build the project
```
cargo build
```

### Example

```c
#include <assert.h>

int main()
{
  assert(wildmatch_matches("cat", "cat") != 0);
  assert(wildmatch_matches("*cat*", "dog_cat_dog") != 0);
  assert(wildmatch_matches("c?t", "cat") != 0);
  assert(wildmatch_matches("c?t", "cot") != 0);
  assert(wildmatch_matches("dog", "cat") == 0);
  assert(wildmatch_matches("*d", "cat") == 0);
  assert(wildmatch_matches("????", "cat") == 0);
  assert(wildmatch_matches("?", "cat") == 0);
  return 0;
}
```

### References
 - [wildmatch](https://github.com/becheran/wildmatch)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libwildmatch/blob/main/LICENSE).

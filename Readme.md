### Fast data structures in non-C systems languages
In this project, sort algorithm implemented in Rust language and in parallel way for Python language in order to get performance. The main problem is binding Rust to Python in this project.

### Installation
- ```pip install parallelsort-rust``` 
- [More information](https://pypi.org/project/parallelsort-rust/)

### Usage

```
import numpy as np
import time
import parallelsort_rust

arrayRustBinding = np.random.randint(low = 10, high = 100, size = 100000000).flatten().tolist()
sizeofArray = len(arrayRustBinding)
# size of threads for parallelism
threads = 8

start = time.time()
sorted_array = parallelsort_rust.parallel_sorting(arrayRustBinding, threads, sizeofArray)
end = time.time() - start

print(f"Time: {end}, isSorted: {sorted(arrayRustBinding) == sorted_array}")
```

### Used libraries for binding and learning
- [Rust lang](https://doc.rust-lang.org/std/index.html) 
- [PyO3-maturin](https://github.com/PyO3/maturin)
- [Numpy](https://numpy.org/doc/stable/index.html)
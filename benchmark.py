import numpy as np
import time
import parallelsort_rust

array = np.random.randint(low = 10, high = 100, size = 100000000).flatten().tolist()
s = len(array)
threads = 8

start = time.time()
sorted_array = parallelsort_rust.parallel_sorting(array, threads, s)
end = time.time() - start

print(f"Time: {end}, isSorted: {sorted(array) == sorted_array}")
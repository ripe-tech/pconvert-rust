# Python module

Source files:

- [`mod.rs`](./mod.rs)
    - exposes the python module and associated API
    - manages the internal global thread pool

- [`conversions.rs`](./conversions.rs)
    - type conversions from and to Python types

- [`utils.rs`](./utils.rs)
    - optional parameter parsing functions
    - other utility functions

## Python API

```python
# blends two images read from the local file system and writes the result to the file system
blend_images(bot_path, top_path, target_path, algorithm, is_inline, options)

# blends multiple images read from the local file system and writes the result to the file system
blend_multiple(img_paths, out_path, algorithm, algorithms, is_inline, options)

# returns a python dict with summary information about the internal thread pool (size, active jobs, queued jobs)
get_thread_pool_status()

# access module constants (e.g. ALGORITHMS, FILTER_TYPES, COMPILER_VERSION, ...)
pconvert_rust.ALGORITHMS
pconvert_rust.FILTER_TYPES
pconvert_rust.COMPILER_VERSION
```

The parameter `options` is a python dictionary of optional parameters. It may look like this:

```python
{
    "compression": "best",
    "filter": "nofilter",
    "num_threads": 1
}
```

If `num_threads` is specified with a value of 1 or more, the work load is distributed across multiple threads (belonging to the internally managed thread pool). 

For example, for `num_threads: 5`, pconvert ensures there exist at least 5 threads in the pool. However, these may be occupied. Hence, this property is a request of a certain degree of parallelism, but it is not certain that the number of threads is the same. 

Additionally, the pool has a maximum number of threads.

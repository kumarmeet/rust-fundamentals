# rust-fundamentals
Learning rust with its core fundamentals and language syntax 

# resources

https://www.youtube.com/watch?v=joCFbTJt0o0&t=136s&ab_channel=HarkiratSingh


https://www.youtube.com/watch?v=qP7LzZqGh30&t=12602s&ab_channel=HarkiratSingh and its quick notes -> https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-13


#Important things to learn 

Rust provides a variety of **data structures** in its standard library, each designed for specific use cases. These data structures can be broadly categorized into **collections**, **primitives**, and **custom types**. Below is a comprehensive list of the most commonly used data structures in Rust:

---

### 1. **Primitive Data Structures**
These are the basic building blocks provided by Rust.

| Data Structure | Description                                                                 |
|----------------|-----------------------------------------------------------------------------|
| `i8`, `i16`, `i32`, `i64`, `i128` | Signed integers of various sizes.                                          |
| `u8`, `u16`, `u32`, `u64`, `u128` | Unsigned integers of various sizes.                                        |
| `f32`, `f64`   | Floating-point numbers (32-bit and 64-bit).                                |
| `bool`         | Boolean value (`true` or `false`).                                         |
| `char`         | A single Unicode scalar value (4 bytes).                                   |
| `array`        | Fixed-size collection of elements of the same type (e.g., `[i32; 5]`).     |
| `slice`        | Dynamically-sized view into a contiguous sequence (e.g., `&[i32]`).        |
| `str`          | String slice (e.g., `&str`).                                               |
| `tuple`        | Fixed-size collection of heterogeneous elements (e.g., `(i32, f64, &str)`).|
| `unit`         | Empty value, represented by `()`.                                          |

---

### 2. **Collections**
Collections are data structures that store multiple values. Rust's standard library provides several collections.

#### a. **Sequences**
| Data Structure | Description                                                                 |
|----------------|-----------------------------------------------------------------------------|
| `Vec<T>`       | Dynamically-sized, heap-allocated array.                                   |
| `VecDeque<T>`  | Double-ended queue implemented with a growable ring buffer.                |
| `LinkedList<T>`| Doubly-linked list.                                                        |

#### b. **Maps**
| Data Structure | Description                                                                 |
|----------------|-----------------------------------------------------------------------------|
| `HashMap<K, V>`| Hash table-based map.                                                      |
| `BTreeMap<K, V>`| B-tree-based map with sorted keys.                                         |

#### c. **Sets**
| Data Structure | Description                                                                 |
|----------------|-----------------------------------------------------------------------------|
| `HashSet<T>`   | Hash table-based set.                                                      |
| `BTreeSet<T>`  | B-tree-based set with sorted elements.                                     |

#### d. **Other Collections**
| Data Structure | Description                                                                 |
|----------------|-----------------------------------------------------------------------------|
| `BinaryHeap<T>`| Priority queue implemented with a binary heap.                             |

---

### 3. **Custom Data Structures**
Rust allows you to define your own data structures using `struct` and `enum`.

#### a. **Structs**
- Used to group related data into a single type.
- Example:
  ```rust
  struct Point {
      x: i32,
      y: i32,
  }
  ```

#### b. **Enums**
- Used to define a type that can have one of several variants.
- Example:
  ```rust
  enum Option<T> {
      Some(T),
      None,
  }
  ```

#### c. **Unions**
- Similar to structs but allow only one field to be active at a time.
- Example:
  ```rust
  union MyUnion {
      i: i32,
      f: f32,
  }
  ```

---

### 4. **Special-Purpose Data Structures**
These are data structures designed for specific use cases.

| Data Structure | Description                                                                 |
|----------------|-----------------------------------------------------------------------------|
| `Option<T>`    | Represents either a value (`Some(T)`) or nothing (`None`).                 |
| `Result<T, E>` | Represents either a success (`Ok(T)`) or an error (`Err(E)`).              |
| `Box<T>`       | A heap-allocated pointer to a value of type `T`.                           |
| `Rc<T>`        | A reference-counted smart pointer for shared ownership.                    |
| `Arc<T>`       | A thread-safe, atomic reference-counted smart pointer.                     |
| `Cell<T>`      | Provides interior mutability for types that implement `Copy`.              |
| `RefCell<T>`   | Provides interior mutability with runtime borrow checking.                 |
| `Mutex<T>`     | A mutual exclusion primitive for thread-safe access to shared data.        |
| `RwLock<T>`    | A reader-writer lock for thread-safe access to shared data.                |

---

### 5. **External Libraries**
Rust's ecosystem includes many external libraries (crates) that provide additional data structures, such as:
- **`serde`**: For serialization and deserialization.
- **`rayon`**: For parallel iterators and collections.
- **`ndarray`**: For multi-dimensional arrays.
- **`dashmap`**: For concurrent hash maps.
- **`im`**: For immutable, persistent data structures.

---

### Summary of Data Structures in Rust
| Category               | Data Structures                                                                 |
|------------------------|---------------------------------------------------------------------------------|
| **Primitives**         | Integers, floats, booleans, chars, arrays, slices, tuples, unit.               |
| **Sequences**          | `Vec<T>`, `VecDeque<T>`, `LinkedList<T>`.                                      |
| **Maps**               | `HashMap<K, V>`, `BTreeMap<K, V>`.                                             |
| **Sets**               | `HashSet<T>`, `BTreeSet<T>`.                                                   |
| **Other Collections**  | `BinaryHeap<T>`.                                                               |
| **Custom Types**       | Structs, enums, unions.                                                        |
| **Special-Purpose**    | `Option<T>`, `Result<T, E>`, `Box<T>`, `Rc<T>`, `Arc<T>`, `Cell<T>`, `RefCell<T>`, `Mutex<T>`, `RwLock<T>`. |

---

### Total Number of Data Structures
In the standard library, Rust provides **15 primary data structures**:
1. `Vec<T>`
2. `VecDeque<T>`
3. `LinkedList<T>`
4. `HashMap<K, V>`
5. `BTreeMap<K, V>`
6. `HashSet<T>`
7. `BTreeSet<T>`
8. `BinaryHeap<T>`
9. `Option<T>`
10. `Result<T, E>`
11. `Box<T>`
12. `Rc<T>`
13. `Arc<T>`
14. `Cell<T>`
15. `RefCell<T>`

Including primitives, custom types, and external libraries, the total number of data structures in Rust is **much larger**.

---

### When to Use Which Data Structure?
- Use **`Vec<T>`** for dynamic arrays.
- Use **`HashMap<K, V>`** for key-value storage with fast lookups.
- Use **`BTreeMap<K, V>`** for sorted key-value storage.
- Use **`Option<T>`** for optional values.
- Use **`Result<T, E>`** for error handling.
- Use **`Box<T>`** for heap allocation.
- Use **`Rc<T>`** or **`Arc<T>`** for shared ownership.
- Use **`Mutex<T>`** or **`RwLock<T>`** for thread-safe data access.

Rust's rich set of data structures ensures that you can choose the right tool for the job!

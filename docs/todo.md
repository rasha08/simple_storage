## TODO Documentation

---

### **Merkle Tree**:

1. **Memory Usage**:
    - **Current**: The current implementation of the Merkle tree isn't optimal in terms of memory usage. This is
      especially true with the cloning done in the `create_tree` method.
    - **Improvement**: Implement a more memory-efficient structure, possibly by avoiding cloning or considering
      alternative tree-building approaches.

2. **Balanced vs. Unbalanced Trees**:
    - **Current**: If there's an odd number of nodes, the tree might be unbalanced.
    - **Improvement**: Ensure balanced trees by duplicating the last hash or using placeholder nodes. This is a standard
      approach but was not implemented in the interest of keeping the tree structure straightforward.

---

### **Files**:

1. **Async File Operations**:
    - **Current**: Currently utilizing standard synchronous file system operations.
    - **Improvement**: Shift to using `tokio::fs` for asynchronous file operations to benefit from non-blocking I/O.

---

### **Server**:

1. **Multithreaded File Storage**:
    - **Current**: Single-threaded file storing mechanism.
    - **Improvement**: Implement multithreaded file storage using `tokio::fs` and `tokio` tasks to handle concurrent
      file uploads more efficiently.

---

### **Client**:

1. **User Feedback**:
    - **Current**: Limited feedback for user actions.
    - **Improvement**: Implement more comprehensive user feedback mechanisms, such as success and error notifications.

2. **Error Handling**:
    - **Current**: Limited error handling on the client side.
    - **Improvement**: Develop a dedicated error page or modal to inform users about any issues or failures.

---

### **Overall**:

1. **Persistent Data**:
    - **Current**: LevelDB data is wiped after every server or client app restart for testing purposes.
    - **Improvement**: Implement a mechanism to allow for permanent persistence, so data remains even after application
      restarts.


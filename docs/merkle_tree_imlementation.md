### Merkle Tree Implementation

A Merkle tree is a binary tree in which every leaf node contains the hash of a block of data, and each non-leaf node
contains the cryptographic hash of its children. This structure allows for efficient and secure verification of the
contents of large data structures.

#### Key Components:

1. **Node:** Represents an individual block in the tree containing hash data and potential pointers to left and right
   children.
2. **Position Enum:** Determines the position of a child, whether it's left or right. This is crucial for hashing
   operations to ensure consistency.
3. **Create Tree:** Constructs the Merkle tree from given data.
4. **Generate Proof:** Produces a proof of existence for a specific data block.
5. **Verify Proof:** Validates the proof against the Merkle root.

---

### Implementation Details:

#### 1. **Node Structure:**

This is a basic structure representing a node in the tree. Each node has a hash and potential left or right children.

#### 2. **Position Enum:**

The `Position` enum determines if a hash is coming from the left or right child. This distinction ensures that when two
nodes are combined to produce a parent node, their order is consistently maintained.

```rust
enum Position {
    Left,
    Right,
}
```

#### 3. **Creating the Tree (`create_tree` method):**

This method accepts a vector of hashes (from data blocks) and constructs a Merkle tree.

- **Input:** `file_hashes` - List of file hashes.
- **Output:** The root node of the tree.

The hashes are wrapped into nodes. As long as there are multiple nodes, the function combines them in pairs to form
parent nodes. If there's an odd number of nodes, the last node is carried over to the next level. This process is
repeated until only the root node remains.

#### 4. **Generating Proof (`generate_proof` method):**

To prove the existence of a particular data block in the tree, we provide a "proof" - a list of hashes that, when
combined with our data block's hash, will produce the Merkle root.

- **Input:** `hash` - The hash we want to generate a proof for.
- **Output:** A list of hashes, each with a position, forming the proof.

This function works recursively. As it traverses the tree, it collects hashes that are on the "opposite" side of the
current traversal path.

#### 5. **Verifying the Proof (`verify_proof` method):**

To verify a proof, one needs to combine the provided hashes in the order specified, with the data block's hash, aiming
to reproduce the Merkle root.

- **Input:** `file_hash`, `proof`, `root_hash` - The hash we're checking, the proof, and the known root hash.
- **Output:** Boolean indicating whether the proof is valid.

The function takes the `file_hash` and combines it with each hash in the `proof`, in the order specified by the
accompanying `Position` value. If the final hash matches the `root_hash`, the proof is valid.

---

### Decision Making:

1. **Position Enum:**
    - **Reason:** Without knowing the position, there's ambiguity in the order two hashes should be combined. This can
      lead to inconsistencies and failed verifications even if the proof is correct.
    - **Trade-offs:** Adds a bit more overhead to each proof with the need to store the position. However, the benefit
      of consistency far outweighs this.

2. **Recursive Proof Generation:**
    - **Reason:** A Merkle tree is inherently recursive in structure. Using recursion for proof generation aligns with
      this natural structure.
    - **Trade-offs:** Recursive implementations can be harder to follow and may have stack overflow risks for extremely
      deep trees. In practice, Merkle trees rarely reach depths that would cause stack concerns.

---

### Known Trade-offs:

1. **Memory Usage:** The implementation isn't the most memory-efficient, particularly with cloning in the `create_tree`
   method. However, it's straightforward and easier to understand.
2. **Balanced vs. Unbalanced Trees:** This implementation allows for unbalanced trees if there's an odd number of nodes.
   Some implementations ensure balanced trees by duplicating the last hash, but this choice was to keep the tree
   structure more intuitive.

---

### Conclusion:

This Merkle tree implementation provides the foundational methods required to construct, prove, and verify data in the
context of the tree. It prioritizes clarity and correctness over performance optimizations, making it ideal for
educational and initial developmental use. For high-performance applications, consider memory optimization and
non-recursive alternatives.
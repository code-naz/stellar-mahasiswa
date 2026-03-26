# Stellar Mahasiswa DApp

**Stellar Mahasiswa DApp** - Blockchain-Based Decentralized Student Management System

## Project Description

Stellar Mahasiswa DApp is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, immutable, and transparent platform for managing student records directly on the blockchain. By utilizing smart contracts, this system ensures that student data is stored permanently and can only be managed through predefined functions, removing the need for traditional centralized databases.

The system allows administrators or users to register students, view the complete directory, and remove records, leveraging the high-efficiency and robust security of the Stellar network.

## Project Vision

Our vision is to modernize academic record-keeping in the digital age by:

- **Decentralizing Academic Data**: Moving student registries from vulnerable centralized servers to a global, distributed ledger.

- **Ensuring Data Integrity**:  Providing a permanent, tamper-proof record of student enrollment that cannot be illegally altered.

- **Promoting Transparency**:  Allowing for public verification of student status while maintaining blockchain-level security.

- **Building Trustless Education Systems**: Creating a foundation where academic credentials and records are guaranteed by code, not just institutional promises.

## Key Features

### 1. **Student Registration**

- Add student data with a single contract call.
- Store essential information: Name and Major/Department.
- Automated unique ID generation for each student record.
- Persistent storage on the Stellar blockchain.

### 2. **Global Directory Access**

- Fetch the entire list of registered students in one call.
- Structured data format for seamless integration with web or mobile frontends.
- Real-time synchronization with the current blockchain state.

### 3. **Secure Record Management**

- Remove specific student records using their unique IDs.
- Immediate update of the student registry after deletion.
- Efficient storage management within the Soroban instance.

### 4. **Transparency & Auditability**

- All additions and removals are verifiable on the blockchain.
- Immutable history of student data management activities.
- Protection against unauthorized data tampering.

## Contract Details

- Contract Address: CCC6BJ3QPAR7QFHFMQ6MDEBEVGTQNHGRXO4YUORMR24W6MEKJ3EXMULS
  ![alt text](Screenshot.png)

## Future Scope

### Short-Term Enhancements

1. **GPA & Academic Credits**: Add functionality to store and update grades and credit scores.
2. **Advanced Search**: Implement filters to find students by Major or Year of entry.
3. **Update Function**: Add a function to update student details (e.g., changing majors) without deleting the record.

### Medium-Term Development

5. **Digital Diplomas**: Issue verifiable digital certificates or NFT-based diplomas upon graduation.
6. **Role-Based Access (RBAC)**: Implement permissions so only authorized campus "Admins" can add or delete records.
7. **Scholarship Tracking**: Integrate asset transfers to automate scholarship disbursements based on student records.

### Long-Term Vision

8. **Cross-University Verification**: A unified network where different institutions can verify student transfers securely.
9. **Decentralized Identity (DID)**: Integrate with W3C DID standards for sovereign student identities.
10. **DAO Governance**: Allow student organizations or faculty boards to vote on registry policy changes.


---

## Technical Requirements

- Soroban SDK : Latest version
- Rust programming language : For smart contract development
- Stellar blockchain network : Testnet for deployment

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the three main functions:

- `tambah_mahasiswa(nama, jurusan)` – Register a new student with their name and major.
- `get_all_mahasiswa()` – Retrieve the list of all students currently stored in the contract.
- `hapus_mahasiswa(id)`– Remove a student record from the registry using their unique ID.

---

**Stellar Mahasiswa DApp** - Securing Academic Records on the Blockchain.

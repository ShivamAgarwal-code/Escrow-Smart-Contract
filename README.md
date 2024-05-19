# Escrow Smart contract for renting a book - OpenBooks.

### Creating an escrow smart contract for renting books on the Solana blockchain using Rust. 

### Here's a detailed guide on how to achieve this, including the smart contract code and necessary APIs.

## Prerequisities

- Rust programming language.
- Solana development environment.
- Anchor framework for Solana smart contracts.
- Nest.js for the backend API.

## Step-by-Step Guide to deploy the Escrow Smart Contract.

1. **Set up the Solana development environment.**

    Ensure you have the Rust programming language and Solana development tools installed on your system.
    
    > Install Rust

    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

    > Install Solana CLI

    `sh -c "$(curl -sSfL https://release.solana.com/v1.8.2/install)"`

    > Install Anchor

    `cargo install --git https://github.com/project-serum/anchor --tag v0.20.1 anchor-cli`

2. **Create the Anchor project.**

    Create a new Anchor project for the smart contract.

    `anchor init open_books`
    
    `cd open_books`

3. **Define the Smart Contract.**

    Create a file named **lib.rs** in the programs/open_books/src/ directory with the following code provided in the **[lib.rs](./contract/lib.rs)** file.

4. **Build and Deploy the Smart Contract.**

    > Build the Smart Contract

    `anchor build`

    > Deploy the Smart Contract

    `anchor deploy`

## Step-by-Step Guide to create the Backend API.

1. **Set up the Nest.js project.**

    First, ensure you have Node.js and NestJS CLI installed. If not, install them:

    `npm install -g @nestjs/cli`

    > Create a new Nest.js project:

    `nest new open-books-backend`

    `cd open-books-backend`

2. **Install Necessary Dependencies.**

    Install Solana and Anchor dependencies:
    
    `npm install @solana/web3.js @project-serum/anchor @nestjs/common`

3. **Create the Books Module.**

    Generate the 'books' module, controller and service:

    `nest generate module books`

    `nest generate controller books`

    `nest generate service books`

4. **Configure the Books Service.**

    Create a service that will handle interactions with the Solana blockchain.

    Check the **[books.service.ts](./backend/books.service.ts)** file for the service code.

5. **Create the Books Controllers.**

    Check the **[books.controller.ts](./backend/books.controller.ts)** file for the service code.

6. **Configure the Module.**

    Check the **[books.module.ts](./backend/books.module.ts)** file for the service code.

7. **Integrate the Module into the Main Application.**

    Check the **[app.module.ts](./backend/app.module.ts)** file for the service code.

8. **Start your Application.**

    Ensure your NestJs Application is running: 

    `npm run start`

9. **Environment Variables.**

    Ensure you have the necessary environment variables set up in your .env file for things like secret keys:

    `RENTER_SECRET_KEY=[your_renter_secret_key_here]`

10. **Testing Endpoints**

    You can use tools like Postman or cURL to test the API endpoints.

    > Initialise a Book Rental

    `curl -X POST http://localhost:3000/books/initialize -H "Content-Type: application/json" -d '{"rentPricePerDay": 5}'`

    > Rent a Book

    `curl -X POST http://localhost:3000/books/rent -H "Content-Type: application/json" -d '{"renter": "RENTER_PUBLIC_KEY", "days": 7}'`

    > Return a Book

    `curl -X POST http://localhost:3000/books/return -H "Content-Type: application/json" -d '{"renter": "RENTER_PUBLIC_KEY"}'`
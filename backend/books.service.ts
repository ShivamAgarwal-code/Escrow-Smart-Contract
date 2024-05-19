import { Injectable } from '@nestjs/common';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { Program, AnchorProvider, BN, web3 } from '@project-serum/anchor';
import idl from './idl.json';  // Assume you have your IDL in a JSON file

@Injectable()
export class BooksService {
  private connection: Connection;
  private provider: AnchorProvider;
  private program: Program;

  constructor() {
    // Initialize Solana connection and provider
    this.connection = new Connection('https://api.devnet.solana.com');
    const wallet = Keypair.generate();
    this.provider = new AnchorProvider(this.connection, wallet, {});
    const programId = new PublicKey('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgQQTV5Lcb8a');
    this.program = new Program(idl, programId, this.provider);
  }

  async initializeBookRental(rentPricePerDay: number): Promise<string> {
    const owner = Keypair.generate();

    const tx = await this.program.rpc.initialize(new BN(rentPricePerDay), {
      accounts: {
        bookRental: Keypair.generate().publicKey, // Replace with the actual book rental account
        owner: owner.publicKey,
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [owner],
    });

    return tx;
  }

  async rentBook(renter: string, days: number): Promise<string> {
    const tx = await this.program.rpc.rentBook(new BN(days), {
      accounts: {
        bookRental: new PublicKey('BOOK_RENTAL_ACCOUNT_PUBKEY'), // Replace with actual account
        renter: new PublicKey(renter),
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [Keypair.fromSecretKey(new Uint8Array(JSON.parse(process.env.RENTER_SECRET_KEY)))],
    });

    return tx;
  }

  async returnBook(renter: string): Promise<string> {
    const owner = Keypair.generate();

    const tx = await this.program.rpc.returnBook({
      accounts: {
        bookRental: new PublicKey('BOOK_RENTAL_ACCOUNT_PUBKEY'), // Replace with actual account
        owner: owner.publicKey,
        renter: new PublicKey(renter),
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [owner, Keypair.fromSecretKey(new Uint8Array(JSON.parse(process.env.RENTER_SECRET_KEY)))],
    });

    return tx;
  }
}

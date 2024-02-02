import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { ChimkenRun } from "../target/types/chimken_run";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("chimken-run", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ChimkenRun as Program<ChimkenRun>;
  const provider = anchor.getProvider();

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`
    );
    return signature;
  };

  const admin = Keypair.generate();
  const competition = PublicKey.findProgramAddressSync(
    [Buffer.from("chimkenadmin"), admin.publicKey.toBuffer()],
    program.programId
  )[0];

  const treasury = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury"), competition.toBuffer()],
    program.programId
  )[0];

  const fee = PublicKey.findProgramAddressSync(
    [Buffer.from("fee"), competition.toBuffer()],
    program.programId
  )[0];

  const user = Keypair.generate();
  const user2 = Keypair.generate();
  it("Airdrop", async () => {
    await Promise.all([
      await provider.connection
        .requestAirdrop(admin.publicKey, LAMPORTS_PER_SOL * 10)
        .then(confirm),
      await provider.connection
        .requestAirdrop(user.publicKey, LAMPORTS_PER_SOL * 10)
        .then(confirm),
      await provider.connection
        .requestAirdrop(user2.publicKey, LAMPORTS_PER_SOL * 10)
        .then(confirm),
    ]);
  });

  it("Initialized!", async () => {
    const tx = await program.methods
      .initialize(new BN(1e9), new BN(1))
      .accounts({
        admin: admin.publicKey,
        competition,
        treasury,
        fee,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Join", async () => {
    const tx = await program.methods
      .join(new BN(1e9))
      .accounts({
        user: user.publicKey,
        competition,
        treasury,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Join", async () => {
    const tx = await program.methods
      .join(new BN(1e9))
      .accounts({
        user: user2.publicKey,
        competition,
        treasury,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user2])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Finish", async () => {
    const tx = await program.methods
      .finish(16)
      .accounts({
        user: user.publicKey,
        competition,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Finish", async () => {
    const tx = await program.methods
      .finish(5)
      .accounts({
        user: user2.publicKey,
        competition,
        systemProgram: SystemProgram.programId,
      })
      .signers([user2])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Winner", async () => {
    const tx = await program.methods
      .winner()
      .accounts({
        user: user2.publicKey,
        competition,
        systemProgram: SystemProgram.programId,
        treasury,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user2])
      .rpc()
      .then(confirm)
      .then(log);
  });
});

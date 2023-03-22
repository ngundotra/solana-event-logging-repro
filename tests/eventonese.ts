import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { Eventonese } from "../target/types/eventonese";
import { ProgramB } from "../target/types/program_b";
import { ProgramC } from "../target/types/program_C";

describe("eventonese", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Eventonese as Program<Eventonese>;
  const pB = anchor.workspace.ProgramB as Program<ProgramB>;
  const pC = anchor.workspace.ProgramC as Program<ProgramC>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .joke(0)
      .accounts({
        payer: program.provider.publicKey!,
        switch: PublicKey.findProgramAddressSync(
          [Buffer.from("eventonese")],
          program.programId
        )[0],
        programB: pB.programId,
        programC: pC.programId,
      })
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx);

    const tx2 = await program.methods
      .joke(1)
      .accounts({
        payer: program.provider.publicKey!,
        switch: PublicKey.findProgramAddressSync(
          [Buffer.from("eventonese")],
          program.programId
        )[0],
        programB: pB.programId,
        programC: pC.programId,
      })
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx2);
  });
});

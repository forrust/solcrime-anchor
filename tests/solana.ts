import * as anchor from '@project-serum/anchor';
import { Program, BN } from '@project-serum/anchor';
import { Solana } from '../target/types/solana';
import * as assert from "assert";
import * as bs58 from "bs58";

describe("solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solana as Program<Solana>;

  it('can store an outcome and reward', async () => {
    const outcome = 'Success';
    const reward = new BN(1);
    const crime = anchor.web3.Keypair.generate();
    const accounts = {
      crime: crime.publicKey,
      author: program.provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    };
    await program.methods.storeCrime(outcome, reward).accounts(accounts).signers([crime]).rpc();

    const crimeAccount = await program.account.crime.fetch(crime.publicKey);

    assert.ok(crimeAccount.outcome === outcome);
    assert.ok(crimeAccount.reward.eq(reward));
    assert.ok(crimeAccount.author.equals(program.provider.publicKey));
    assert.ok(crimeAccount.timestamp);
  });

  it('can fetch all crimes', async () => {
    const crimeAccounts = await program.account.crime.all();
    console.log(crimeAccounts);
    assert.equal(crimeAccounts.length, 1);
  });

  it('can filter crimes by author', async () => {
    const authorPublicKey = program.provider.publicKey
    const tweetAccounts = await program.account.crime.all([
      {
        memcmp: {
          offset: 8, // Discriminator.
          bytes: authorPublicKey.toBase58(),
        }
      }
    ]);

    assert.equal(tweetAccounts.length, 1);
  });

});

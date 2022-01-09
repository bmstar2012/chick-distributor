import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { ChickDistributor } from '../target/types/chick_distributor';

describe('chick-distributor', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.ChickDistributor as Program<ChickDistributor>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});

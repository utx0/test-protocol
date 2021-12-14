import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TestProtocol } from '../target/types/test_protocol';

describe('test-protocol', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.TestProtocol as Program<TestProtocol>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});

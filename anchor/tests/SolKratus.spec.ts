import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair} from '@solana/web3.js'
import {SolKratus} from '../target/types/SolKratus'

describe('SolKratus', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.SolKratus as Program<SolKratus>

  const SolKratusKeypair = Keypair.generate()

  it('Initialize SolKratus', async () => {
    await program.methods
      .initialize()
      .accounts({
        SolKratus: SolKratusKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([SolKratusKeypair])
      .rpc()

    const currentCount = await program.account.SolKratus.fetch(SolKratusKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment SolKratus', async () => {
    await program.methods.increment().accounts({ SolKratus: SolKratusKeypair.publicKey }).rpc()

    const currentCount = await program.account.SolKratus.fetch(SolKratusKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment SolKratus Again', async () => {
    await program.methods.increment().accounts({ SolKratus: SolKratusKeypair.publicKey }).rpc()

    const currentCount = await program.account.SolKratus.fetch(SolKratusKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement SolKratus', async () => {
    await program.methods.decrement().accounts({ SolKratus: SolKratusKeypair.publicKey }).rpc()

    const currentCount = await program.account.SolKratus.fetch(SolKratusKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set SolKratus value', async () => {
    await program.methods.set(42).accounts({ SolKratus: SolKratusKeypair.publicKey }).rpc()

    const currentCount = await program.account.SolKratus.fetch(SolKratusKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the SolKratus account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        SolKratus: SolKratusKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.SolKratus.fetchNullable(SolKratusKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})

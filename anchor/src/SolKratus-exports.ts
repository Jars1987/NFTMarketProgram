// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import SolKratusIDL from '../target/idl/SolKratus.json'
import type { SolKratus } from '../target/types/SolKratus'

// Re-export the generated IDL and type
export { SolKratus, SolKratusIDL }

// The programId is imported from the program IDL.
export const SOL_KRATUS_PROGRAM_ID = new PublicKey(SolKratusIDL.address)

// This is a helper function to get the SolKratus Anchor program.
export function getSolKratusProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...SolKratusIDL, address: address ? address.toBase58() : SolKratusIDL.address } as SolKratus, provider)
}

// This is a helper function to get the program ID for the SolKratus program depending on the cluster.
export function getSolKratusProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the SolKratus program on devnet and testnet.
      return new PublicKey('coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF')
    case 'mainnet-beta':
    default:
      return SOL_KRATUS_PROGRAM_ID
  }
}
